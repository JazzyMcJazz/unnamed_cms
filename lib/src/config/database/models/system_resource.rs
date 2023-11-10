use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub enum ResourceType {
    SystemTable,
    SystemField,
    Table,
    Field,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemResource {
    id: Thing,
    name: String,
    resource_type: ResourceType,
    description: String,
    created_at: String,
    updated_at: String,
}
