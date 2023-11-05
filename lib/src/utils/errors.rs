use actix_web::{http::StatusCode, HttpResponse};

pub struct ErrorResponse;

impl ErrorResponse {
    pub fn build(status: u16, message: &'static str) -> HttpResponse {
        HttpResponse::build(
            StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        )
        .body(message)
    }
}

pub enum CmsError {
    SeeOther(&'static str),
    Unauthorized(Option<&'static str>),
    NotFound(Option<&'static str>),
    InternalServerError(Option<&'static str>),
}

impl CmsError {
    pub fn build_response(&self) -> HttpResponse {
        match self {
            CmsError::SeeOther(location) => HttpResponse::SeeOther()
                .append_header(("Location", *location))
                .append_header(("HX-Redirect", *location))
                .finish(),
            CmsError::Unauthorized(message) => {
                ErrorResponse::build(401, message.unwrap_or("Unauthorized"))
            }
            CmsError::NotFound(message) => {
                ErrorResponse::build(404, message.unwrap_or("Not Found"))
            }
            CmsError::InternalServerError(message) => {
                ErrorResponse::build(500, message.unwrap_or("Internal Server Error"))
            }
        }
    }

    pub fn from<T>(error: T) -> CmsError
    where
        T: std::fmt::Debug,
    {
        dbg!(error);
        CmsError::InternalServerError(None)
    }
}
