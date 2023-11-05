mod models;
mod repo;

use surrealdb::{engine::any::Any, Error, Surreal};

use crate::utils::CmsError;

use self::models::SystemUser;

#[async_trait::async_trait]
pub trait Repository {
    async fn init(&self) -> Result<(), Error>;
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsError>;
    async fn dev_clear(&self) -> Result<(), Error>;
}

#[async_trait::async_trait]
impl Repository for Surreal<Any> {
    async fn init(&self) -> Result<(), Error> {
        repo::init::init(self).await
    }
    async fn dev_clear(&self) -> Result<(), Error> {
        repo::init::dev_clear(self).await
    }
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsError> {
        repo::system_user::find_by_credentials(self, (email, password)).await
    }
}
