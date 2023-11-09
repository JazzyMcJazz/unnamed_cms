mod models;
mod repo;

use surrealdb::{engine::any::Any, Error, Surreal};

use crate::utils::CmsError;

use self::models::{SessionIdToken, SessionToken, SystemUser};

#[async_trait::async_trait]
pub trait Repository {
    // Setup
    async fn init(&self) -> Result<(), Error>;
    async fn dev_clear(&self) -> Result<(), Error>;

    // Create
    async fn create_session(&self, user_id: String) -> Result<SessionIdToken, CmsError>;
    // Read
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsError>;

    // Update
    async fn refresh_session(&self, token: &str) -> Result<SessionToken, CmsError>;

    // Delete
    async fn delete_session(&self, session_id: String) -> Result<(), CmsError>;
}

#[async_trait::async_trait]
impl Repository for Surreal<Any> {
    async fn init(&self) -> Result<(), Error> {
        repo::init::init(self).await
    }
    async fn dev_clear(&self) -> Result<(), Error> {
        repo::init::dev_clear(self).await
    }

    // Create
    async fn create_session(&self, user_id: String) -> Result<SessionIdToken, CmsError> {
        repo::session::create_session(self, user_id).await
    }

    // Read
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsError> {
        repo::system_user::find_by_credentials(self, (email, password)).await
    }

    // Update
    async fn refresh_session(&self, token: &str) -> Result<SessionToken, CmsError> {
        repo::session::refresh_session(self, token).await
    }

    // Delete
    async fn delete_session(&self, session_id: String) -> Result<(), CmsError> {
        repo::session::delete_session(self, session_id).await
    }
}
