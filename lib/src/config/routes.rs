use actix_web::web::{self, ServiceConfig};

use crate::views;

pub fn configure(cfg: &mut ServiceConfig, prefix: &str) {
    cfg.route(prefix, web::get().to(views::index::index));
}
