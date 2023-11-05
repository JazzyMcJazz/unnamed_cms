use surrealdb::{engine::any::Any, Surreal};
use tera::{Error, Tera};

const TEMPLATE_ERROR: &str = "Template error";
pub trait TemplateError {
    fn template_error(&self, e: Error) -> String;
}

impl TemplateError for Tera {
    fn template_error(&self, e: Error) -> String {
        dbg!(e);
        String::from(TEMPLATE_ERROR)
    }
}

#[derive(Clone)]
pub struct AppState {
    database: Surreal<Any>,
    tera: Tera,
    prefix: &'static str,
}

impl AppState {
    pub fn new(database: Surreal<Any>, prefix: &'static str) -> Self {
        let template_dir = "lib/templates/**/*.html";
        let tera = Tera::new(template_dir).unwrap();

        Self {
            database,
            tera,
            prefix,
        }
    }

    pub fn database(&self) -> &Surreal<Any> {
        &self.database
    }

    pub fn tera(&self) -> &Tera {
        &self.tera
    }

    pub fn prefix(&self) -> &'static str {
        if self.prefix.is_empty() {
            return "/";
        }
        self.prefix
    }
}
