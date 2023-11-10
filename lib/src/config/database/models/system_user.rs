use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct SystemUser {
    pub id: Thing,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: String,
    pub admin: bool,
}
