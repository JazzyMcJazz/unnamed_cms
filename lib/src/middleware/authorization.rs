use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web, Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use std::{
    fmt,
    future::{ready, Ready},
};

use crate::config::Claims;
use crate::prelude::*;

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
