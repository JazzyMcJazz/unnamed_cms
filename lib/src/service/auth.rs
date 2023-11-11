use actix_web::cookie::{time::OffsetDateTime, Cookie};

use crate::{config::RefreshToken, prelude::*, utils::LoginForm};

pub struct AuthService;

impl AuthService {
    pub async fn login<'a>(
        db: &Surreal<Any>,
        path: &'a str,
        credentials: LoginForm,
    ) -> Result<(Cookie<'a>, Cookie<'a>), CmsResponse> {
        let user = match db
            .find_user_by_credentials((credentials.email, credentials.password))
            .await
        {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        let refresh_token = db.create_session(user.id.id.to_raw()).await?;
        let claims = Claims::new(
            user.id.id.to_raw(),
            user.email,
            user.name,
            refresh_token.session.id.id.to_raw(),
            user.admin,
            true,
        );
        let access_token = claims.sign_jwt()?;

        let access_expires = match OffsetDateTime::from_unix_timestamp(claims.exp as i64) {
            Ok(access_expires) => access_expires,
            Err(e) => return Err(CmsResponse::from(e)),
        };
        let refresh_expires =
            match OffsetDateTime::from_unix_timestamp(refresh_token.expires_at.timestamp()) {
                Ok(refresh_expires) => refresh_expires,
                Err(e) => return Err(CmsResponse::from(e)),
            };

        let access_cookie = Cookie::build("cms_id", access_token)
            .path(path)
            .secure(true)
            .http_only(true)
            .expires(access_expires)
            .finish();

        let refresh_cookie = Cookie::build("cms_r", refresh_token.token)
            .path(path)
            .secure(true)
            .http_only(true)
            .expires(refresh_expires)
            .finish();

        Ok((access_cookie, refresh_cookie))
    }

    pub async fn logout<'a>(
        db: &Surreal<Any>,
        claims: Claims,
        path: &'static str,
    ) -> (Cookie<'a>, Cookie<'a>) {
        let _ = db.delete_session(claims.session_id).await;
        let expires = OffsetDateTime::from_unix_timestamp(0).unwrap();

        let access_cookie = Cookie::build("cms_id", "")
            .path(path)
            .secure(true)
            .http_only(true)
            .expires(expires)
            .finish();

        let refresh_cookie = Cookie::build("cms_r", "")
            .path(path)
            .secure(true)
            .http_only(true)
            .expires(expires)
            .finish();

        (access_cookie, refresh_cookie)
    }

    pub async fn handle_authentication(
        db: &Surreal<Any>,
        (cms_id, cms_r): (Option<Cookie<'_>>, Option<Cookie<'_>>),
    ) -> (Claims, Option<RefreshToken>) {
        if let Some(cookie) = cms_id {
            (
                Claims::from_jwt(cookie.value()).unwrap_or_else(|_| Claims::new_anon()),
                None,
            )
        } else if let Some(cookie) = cms_r {
            let (claims, refresh_token) = Claims::from_refresh_token(db, cookie.value()).await;
            (claims, refresh_token)
        } else {
            (Claims::new_anon(), None)
        }
    }
}
