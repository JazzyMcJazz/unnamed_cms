use actix_web::web::{self, ServiceConfig};

use crate::api;

use super::security::middlware::{Authentication, Authorization};

pub fn configure(cfg: &mut ServiceConfig, base_path: &'static str) {
    cfg.service(
        web::scope(base_path)
            .wrap(Authentication {})
            .service(
                web::scope("/login")
                    .route("", web::get().to(api::html::login_page))
                    .route("", web::post().to(api::html::login)),
            )
            .service(
                web::scope("")
                    .wrap(Authorization::new(base_path))
                    .route("", web::get().to(api::html::index))
                    .route("/logout", web::post().to(api::html::logout)),
            ),
    );
}
