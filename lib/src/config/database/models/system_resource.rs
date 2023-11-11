use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::api::html::CollectionForm;

#[derive(Debug, Deserialize, Serialize)]
pub enum ResourceType {
    SystemCollection,
    SystemField,
    Collection,
    Field,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemResource {
    id: Option<Thing>,
    name: String,
    pub display_name: String,
    pub resource_type: ResourceType,
    pub description: String,
    created_at: Option<String>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SystemResource {
    pub fn id(&self) -> Option<String> {
        self.id.as_ref().map(|id| id.to_string())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn from_form(form: &CollectionForm) -> Self {
        let re = Regex::new(r"[^a-z]").unwrap();
        let display_name = form.display_name.trim();
        let name = &display_name.to_lowercase();
        let name = re.replace_all(name, "_");

        Self {
            id: None,
            name: name.into(),
            display_name: display_name.into(),
            resource_type: ResourceType::Collection,
            description: form.description.trim().to_string(),
            created_at: None,
            updated_at: None,
        }
    }
}
