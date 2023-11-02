use crate::config::{self, app_state::AppState};
use actix_web::web::ServiceConfig;
use surrealdb::{engine::any::Any, Error, Surreal};

pub struct UnnamedCms<'a> {
    database: &'a Surreal<Any>,
    prefix: &'static str,
}

impl<'a> UnnamedCms<'a> {
    pub fn new(database: &'a Surreal<Any>) -> Self {
        Self {
            database,
            prefix: "",
        }
    }

    pub fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn config(&self, cfg: &mut ServiceConfig) {
        let app_state = AppState::new(self.database.clone());
        config::app_data::configure(cfg, app_state);
        config::routes::configure(cfg, self.prefix);
    }

    pub async fn init_db(db: &Surreal<Any>) -> Result<(), Error> {
        config::database::configure(db).await?;
        Ok(())
    }
}
