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
    base_path: &'static str,
}

impl AppState {
    pub fn new(database: Surreal<Any>, base_path: &'static str) -> Self {
        let template_dir = "lib/templates/**/*.html";
        let tera = Tera::new(template_dir).expect("Failed to compile templates");

        Self {
            database,
            tera,
            base_path,
        }
    }

    pub fn database(&self) -> &Surreal<Any> {
        &self.database
    }

    pub fn tera(&self) -> &Tera {
        &self.tera
    }

    pub fn base_path(&self) -> &'static str {
        self.base_path
    }

    pub fn base_path_or_root(&self) -> &'static str {
        if self.base_path.is_empty() {
            "/"
        } else {
            self.base_path
        }
    }
}
