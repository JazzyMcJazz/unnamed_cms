use surrealdb::{engine::any::Any, Surreal};
use tera::Tera;

const TEMPLATE_ERROR: &str = "Template error";
pub trait TemplateError {
    fn template_error(&self) -> String;
}

impl TemplateError for Tera {
    fn template_error(&self) -> String {
        String::from(TEMPLATE_ERROR)
    }
}

#[derive(Clone)]
pub struct AppState {
    database: Surreal<Any>,
    tera: Tera,
}

impl AppState {
    pub fn new(database: Surreal<Any>) -> Self {
        let template_dir = "lib/templates/**/*.html";
        let tera = Tera::new(template_dir).unwrap();

        Self { database, tera }
    }

    pub fn database(&self) -> &Surreal<Any> {
        &self.database
    }

    pub fn tera(&self) -> &Tera {
        &self.tera
    }
}
