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

pub enum CmsResponse {
    SeeOther(String),
    Unauthorized(Option<&'static str>),
    NotFound(Option<&'static str>),
    InternalServerError(Option<&'static str>),
}

impl CmsResponse {
    pub fn build_response(&self) -> HttpResponse {
        let (c, m) = match self {
            CmsResponse::SeeOther(location) => {
                return HttpResponse::SeeOther()
                    .append_header(("Location", location.to_owned()))
                    .append_header(("HX-Redirect", location.to_owned()))
                    .finish()
            }
            CmsResponse::Unauthorized(message) => (401, message.unwrap_or("Unauthorized")),
            CmsResponse::NotFound(message) => (404, message.unwrap_or("Not Found")),
            CmsResponse::InternalServerError(message) => {
                (500, message.unwrap_or("Internal Server Error"))
            }
        };
        ErrorResponse::build(c, m)
    }

    pub fn from<T>(error: T) -> CmsResponse
    where
        T: std::fmt::Debug,
    {
        dbg!(error);
        CmsResponse::InternalServerError(None)
    }
}
