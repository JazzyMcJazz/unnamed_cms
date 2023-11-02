use actix_web::web::{self, ServiceConfig};

use super::app_state::AppState;

pub fn configure(cfg: &mut ServiceConfig, state: AppState) {
    cfg.app_data(web::Data::new(state));
}
