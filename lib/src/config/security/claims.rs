use std::env;

use crate::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct RefreshToken {
    pub token: String,
    pub exp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    email: String,
    name: String,
    pub session_id: String,
    pub exp: usize,
    is_admin: bool,
    pub is_authenticated: bool,
}

impl Claims {
    pub fn new(
        sub: String,
        email: String,
        name: String,
        session_id: String,
        is_admin: bool,
        is_authenticated: bool,
    ) -> Self {
        // TODO: Make this configurable
        let exp = chrono::Utc::now().timestamp() + 60 * 15; // 15 minutes
        Self {
            sub,
            email,
            name,
            session_id,
            exp: exp as usize,
            is_admin,
            is_authenticated,
        }
    }

    pub fn new_anon() -> Self {
        Self {
            sub: "".to_string(),
            email: "".to_string(),
            name: "".to_string(),
            session_id: "".to_string(),
            exp: 0,
            is_admin: false,
            is_authenticated: false,
        }
    }

    pub fn sign_jwt(&self) -> Result<String, CmsError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token = match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(token) => token,
            Err(e) => return Err(CmsError::from(e)),
        };

        Ok(token)
    }

    pub fn from_jwt(token: &str) -> Result<Self, CmsError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let claims = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        ) {
            Ok(token) => token.claims,
            Err(_) => return Err(CmsError::Unauthorized(None)),
        };

        Ok(claims)
    }

    pub async fn from_refresh_token(
        db: &Surreal<Any>,
        token: &str,
    ) -> (Self, Option<RefreshToken>) {
        let refresh_token = match db.refresh_session(token).await {
            Ok(session) => session,
            Err(_) => return (Self::new_anon(), None),
        };

        if refresh_token.expires_at < chrono::Utc::now() {
            let _ = db
                .delete_session(refresh_token.session.id.id.to_raw())
                .await;
            return (Self::new_anon(), None);
        }

        let session = refresh_token.session;
        let user = session.user;

        (
            Self::new(
                user.id.id.to_raw(),
                user.email,
                user.name,
                session.id.id.to_raw(),
                user.admin,
                true,
            ),
            Some(RefreshToken {
                token: refresh_token.token,
                exp: refresh_token.expires_at,
            }),
        )
    }
}
