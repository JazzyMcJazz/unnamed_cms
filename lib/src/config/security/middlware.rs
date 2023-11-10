use actix_web::{
    cookie::{time::OffsetDateTime, Cookie},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web, Error, HttpMessage, HttpResponse, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::{
    fmt,
    future::{ready, Ready},
    rc::Rc,
};
use tera::Context;

use crate::{config::app_state::AppState, service::handle_authentication};

use super::claims::Claims;

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
                handle_authentication(state.database(), (cms_id, cms_r)).await;

            context.insert("user", &claims);
            req.extensions_mut().insert(claims.clone());
            // Add the context to the request extensions
            if req.method() == "GET" {
                req.extensions_mut().insert(context);
            }

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

/// Middleware for authorizing users
/// Checks if the user is logged in
/// Should be applied only to protected routes
pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if the user is logged into the relevant app and redirect if not
        if req.extensions().get::<Claims>().is_none()
            || !req.extensions().get::<Claims>().unwrap().is_authenticated
        {
            let base_path = req.app_data::<web::Data<AppState>>().unwrap().base_path();
            return Box::pin(async move {
                Err(AuthError {
                    base_path,
                    next: req.path().to_string().clone(),
                }
                .into())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

#[derive(Debug)]
pub struct AuthError {
    base_path: &'static str,
    next: String,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let path = match self.next.as_str() {
            "" => format!("{}/login", &self.base_path),
            _ => format!("{}/login?next={}", &self.base_path, &self.next),
        };

        HttpResponse::Found()
            .append_header(("HX-Redirect", path.clone()))
            .append_header(("Location", path))
            .finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
