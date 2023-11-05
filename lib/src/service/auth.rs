use actix_web::cookie::{time::OffsetDateTime, Cookie};

use crate::{prelude::*, utils::LoginForm};

pub async fn login<'a>(
    db: &Surreal<Any>,
    path: &'a str,
    credentials: LoginForm,
) -> Result<Cookie<'a>, CmsError> {
    let user = match db
        .find_user_by_credentials((credentials.email, credentials.password))
        .await
    {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let claims = Claims::new(user.id.id.to_raw(), user.email, user.name, true);
    let token = claims.sign_jwt()?;

    let cookie = Cookie::build("cms_id", token)
        .path(path)
        .secure(true)
        .http_only(true)
        // .expires() // TODO: set expiration
        .finish();

    Ok(cookie)
}

pub fn logout<'a>(path: &'static str) -> Cookie<'a> {
    let expires = OffsetDateTime::from_unix_timestamp(0).unwrap();
    Cookie::build("cms_id", "")
        .path(path)
        .secure(true)
        .http_only(true)
        .expires(expires)
        .finish()
}
