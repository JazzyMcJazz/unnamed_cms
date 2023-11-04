use actix_web::web::{self, ServiceConfig};

use crate::api;

use super::security::middlware::{Authentication, Authorization};

pub fn configure(cfg: &mut ServiceConfig, prefix: &'static str) {
    cfg.service(
        web::scope(prefix)
            .wrap(Authentication::new(prefix))
            .route("/login", web::get().to(api::html::login))
            .service(
                web::scope("")
                    .wrap(Authorization::new(prefix))
                    .route("", web::get().to(api::html::index)),
            ),
    );
}
