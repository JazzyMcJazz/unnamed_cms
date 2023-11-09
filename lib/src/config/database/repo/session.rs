use surrealdb::{engine::any::Any, Surreal};

use crate::{
    config::database::models::{SessionIdToken, SessionToken},
    prelude::*,
};

pub async fn create_session(
    db: &Surreal<Any>,
    user_id: String,
) -> Result<SessionIdToken, CmsError> {
    let mut result = match db.query(CREATE_SESSION).bind(("user_id", user_id)).await {
        Ok(result) => result,
        Err(e) => return Err(CmsError::from(e)),
    };

    let token: SessionIdToken = match result.take(result.num_statements() - 1) {
        Ok(token) => match token {
            Some(token) => token,
            None => return Err(CmsError::NotFound(Some("Internal Server Error"))),
        },
        Err(e) => return Err(CmsError::from(e)),
    };

    Ok(token)
}

pub async fn delete_session(db: &Surreal<Any>, session_id: String) -> Result<(), CmsError> {
    match db
        .query(DELETE_SESSION)
        .bind(("session_id", session_id))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsError::from(e)),
    };

    Ok(())
}

pub async fn refresh_session(db: &Surreal<Any>, token: &str) -> Result<SessionToken, CmsError> {
    let mut result = match db
        .query(REFRESH_SESSION)
        .bind(("refresh_token", token))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsError::from(e)),
    };

    let token: SessionToken = match result.take(result.num_statements() - 1) {
        Ok(token) => match token {
            Some(token) => token,
            None => return Err(CmsError::NotFound(Some("Internal Server Error"))),
        },
        Err(e) => return Err(CmsError::from(e)),
    };

    Ok(token)
}
