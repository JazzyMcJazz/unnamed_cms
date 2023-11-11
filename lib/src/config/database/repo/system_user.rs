use crate::{
    config::database::models::SystemUser,
    prelude::*,
    utils::{query_strings::*, CmsResponse},
};

pub async fn find_by_credentials(
    db: &Surreal<Any>,
    (email, password): (String, String),
) -> Result<SystemUser, CmsResponse> {
    let mut result = match db
        .query(FIND_USER_BY_CREDENTIALS)
        .bind(("email", &email))
        .bind(("password", &password))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(CmsResponse::from(e)),
    };

    let user: SystemUser = match result.take(0) {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(CmsResponse::NotFound(Some("Invalid email or password"))),
        },
        Err(e) => return Err(CmsResponse::from(e)),
    };

    Ok(user)
}
