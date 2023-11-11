mod models;
mod repo;

use surrealdb::{engine::any::Any, Error, Surreal};

use crate::utils::CmsResponse;

use self::models::*;
pub use self::models::{ResourceType, SystemResource};

#[async_trait::async_trait]
pub trait Repository {
    // SETUP
    async fn init(&self) -> Result<(), Error>;
    async fn dev_clear(&self) -> Result<(), Error>;

    // CREATE
    async fn create_resource(
        &self,
        resource: &SystemResource,
    ) -> Result<SystemResource, CmsResponse>;
    async fn create_session(&self, user_id: String) -> Result<SessionIdToken, CmsResponse>;

    // READ
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsResponse>;
    async fn find_resource_by_type(
        &self,
        resource_type: ResourceType,
    ) -> Result<Vec<SystemResource>, CmsResponse>;

    // UPDATE
    async fn refresh_session(&self, token: &str) -> Result<SessionToken, CmsResponse>;

    // DELETE
    async fn delete_session(&self, session_id: String) -> Result<(), CmsResponse>;
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
    async fn create_resource(
        &self,
        resource: &SystemResource,
    ) -> Result<SystemResource, CmsResponse> {
        repo::system_resource::create(self, resource).await
    }
    async fn create_session(&self, user_id: String) -> Result<SessionIdToken, CmsResponse> {
        repo::session::create_session(self, user_id).await
    }

    // Read
    async fn find_user_by_credentials(
        &self,
        (email, password): (String, String),
    ) -> Result<SystemUser, CmsResponse> {
        repo::system_user::find_by_credentials(self, (email, password)).await
    }
    async fn find_resource_by_type(
        &self,
        resource_type: ResourceType,
    ) -> Result<Vec<SystemResource>, CmsResponse> {
        repo::system_resource::find_resource_by_type(self, resource_type).await
    }

    // Update
    async fn refresh_session(&self, token: &str) -> Result<SessionToken, CmsResponse> {
        repo::session::refresh_session(self, token).await
    }

    // Delete
    async fn delete_session(&self, session_id: String) -> Result<(), CmsResponse> {
        repo::session::delete_session(self, session_id).await
    }
}
