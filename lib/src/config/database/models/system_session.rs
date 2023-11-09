use serde::Deserialize;
use surrealdb::sql::Thing;

use super::SystemUser;

#[derive(Debug, Deserialize)]
pub struct SessionId {
    pub id: Thing,
}

#[derive(Debug, Deserialize)]
pub struct Session {
    pub id: Thing,
    pub user: SystemUser,
}
