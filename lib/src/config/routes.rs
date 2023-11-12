use actix_files::Files;
use actix_web::{
    middleware::DefaultHeaders,
    web::{self, ServiceConfig},
};

use crate::{
    api,
    middleware::{AddCollectionData, Authentication, Authorization},
};

pub fn configure(cfg: &mut ServiceConfig, base_path: &'static str) {
    cfg.service(
        web::scope(format!("{base_path}/assets").as_str())
            .wrap(DefaultHeaders::new().add(("Cache-Control", "max-age=3600")))
            .service(Files::new("", "./lib/static")),
    );

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
                    .wrap(Authorization {})
                    .route(
                        if base_path.is_empty() { "/" } else { "" },
                        web::get().to(api::html::index),
                    )
                    .route("/logout", web::post().to(api::html::logout))
                    .service(
                        web::scope("/collections")
                            .wrap(AddCollectionData {})
                            .route("", web::get().to(api::html::collections_index))
                            .route("", web::post().to(api::html::collections_new))
                            .route("/+", web::get().to(api::html::collections_new))
                            .route("/{collection}", web::get().to(api::html::collection_data)),
                    ),
            ),
    );
}
