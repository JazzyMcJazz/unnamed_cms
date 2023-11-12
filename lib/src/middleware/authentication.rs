use actix_web::{
    cookie::{time::OffsetDateTime, Cookie},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use tera::Context;

use crate::{config::app_state::AppState, service::AuthService};

/// Middleware for authenticating users
/// Adds the user's ID to the request extensions
/// Should be applied to all routes
pub struct Authentication;

impl<S: 'static, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let state = req.app_data::<web::Data<AppState>>().unwrap().clone();

        let mut context = Context::new();

        // Add the current path to the context
        context.insert("path", req.path());
        context.insert("next", state.base_path_or_root());
        context.insert("base_path", state.base_path());

        // Add the next path to the context (if it exists)
        req.query_string().split('&').for_each(|q| {
            if q.contains("next=") {
                context.insert("next", q.split('=').last().unwrap_or(state.base_path()));
            }
        });

        let cms_id = req.cookie("cms_id");
        let cms_r = req.cookie("cms_r");

        Box::pin(async move {
            // Add the user to the context (if they are logged in)
            let (claims, refresh_token) =
                AuthService::handle_authentication(state.database(), (cms_id, cms_r)).await;

            context.insert("user", &claims);
            req.extensions_mut().insert(claims.clone());
            req.extensions_mut().insert(context);

            let mut res = svc.call(req).await?;

            // If refresh event occurred, sign and add the new cookies to the response
            if let Some(refresh_token) = refresh_token {
                let exp = claims.exp as i64;
                let result = claims.sign_jwt();
                let access_token = match result {
                    Ok(token) => token,
                    Err(_) => return Ok(res),
                };

                let access_cookie = Cookie::build("cms_id", access_token)
                    .path(state.base_path_or_root())
                    .secure(true)
                    .http_only(true)
                    .expires(OffsetDateTime::from_unix_timestamp(exp).unwrap())
                    .finish();

                let refresh_cookie = Cookie::build("cms_r", refresh_token.token)
                    .path(state.base_path_or_root())
                    .secure(true)
                    .http_only(true)
                    .expires(
                        OffsetDateTime::from_unix_timestamp(refresh_token.exp.timestamp()).unwrap(),
                    )
                    .finish();

                res.response_mut().add_cookie(&access_cookie)?;
                res.response_mut().add_cookie(&refresh_cookie)?;
            }

            Ok(res)
        })
    }
}
