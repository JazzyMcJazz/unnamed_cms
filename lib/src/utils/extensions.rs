use actix_web::{HttpMessage, HttpRequest};
use tera::Context;

use crate::config::Claims;

pub struct Extensions;

impl Extensions {
    pub fn unwrap_claims(req: &HttpRequest) -> Claims {
        let ext = req.extensions();
        ext.get::<Claims>().cloned().unwrap_or(Claims::new_anon())
    }

    pub fn unwrap_context(req: &HttpRequest) -> Context {
        let ext = req.extensions();
        ext.get::<Context>().cloned().unwrap_or(Context::new())
    }

    pub fn unwrap_claims_and_context(req: &HttpRequest) -> (Claims, Context) {
        let ext = req.extensions();

        let claims = ext.get::<Claims>().cloned().unwrap_or(Claims::new_anon());

        let context = ext
            .get::<Context>()
            .cloned()
            .unwrap_or(Context::new())
            .to_owned();
        (claims, context)
    }
}
