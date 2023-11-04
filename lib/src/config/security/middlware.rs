use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error, HttpMessage, HttpResponse, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::DecodingKey;
use std::{
    env, fmt,
    future::{ready, Ready},
};
use tera::Context;

use super::claims::Claims;

/// Middleware for authenticating users
/// Adds the user's ID to the request extensions
/// Should be applied to all routes
pub struct Authentication {
    prefix: &'static str,
}

impl Authentication {
    pub fn new(prefix: &'static str) -> Self {
        Authentication { prefix }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service,
            prefix: self.prefix,
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
    prefix: &'static str,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = S::Response;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut context = Context::new();

        // Add the current path to the context
        context.insert("path", req.path());
        context.insert("next", "/");
        context.insert("prefix", self.prefix);

        // Add the next path to the context (if it exists)
        req.query_string().split('&').for_each(|q| {
            if q.contains("next=") {
                context.insert("next", q.split('=').last().unwrap_or("/"));
            }
        });

        // Add the user to the context (if they are logged in)
        if let Some(cookie) = req.cookie("id") {
            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let token_data = jsonwebtoken::decode::<Claims>(
                cookie.value(),
                &DecodingKey::from_secret(secret.as_ref()),
                &jsonwebtoken::Validation::default(),
            );

            if let Ok(token) = token_data {
                context.insert("user", &token.claims);
                req.extensions_mut().insert(token.claims);
            }
        }

        // Add the context to the request extensions
        if req.method() == "GET" {
            req.extensions_mut().insert(context);
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

/// Middleware for authorizing users
/// Checks if the user is logged in
/// Should be applied only to protected routes
pub struct Authorization {
    prefix: &'static str,
}

impl Authorization {
    pub fn new(prefix: &'static str) -> Self {
        Authorization { prefix }
    }
}

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
        ready(Ok(AuthorizationMiddleware {
            service,
            prefix: self.prefix,
        }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
    prefix: &'static str,
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
        if req.extensions().get::<Claims>().is_none() {
            let prefix = self.prefix;
            return Box::pin(async move {
                Err(AuthError {
                    prefix,
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
    prefix: &'static str,
    next: String,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let prefix = match self.prefix {
            "/" => "",
            _ => self.prefix,
        };

        let path = match self.next.as_str() {
            "" => format!("{}/login", &prefix),
            _ => format!("{}/login?next={}", &prefix, &self.next),
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
