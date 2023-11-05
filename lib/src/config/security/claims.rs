use std::env;

use crate::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    email: String,
    name: String,
    exp: usize,
    pub is_authenticated: bool,
}

impl Claims {
    pub fn new(sub: String, email: String, name: String, is_authenticated: bool) -> Self {
        // TODO: Make this configurable
        let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
        Self {
            sub,
            email,
            name,
            exp: exp as usize,
            is_authenticated,
        }
    }

    pub fn new_anon() -> Self {
        Self {
            sub: "".to_string(),
            email: "".to_string(),
            name: "".to_string(),
            exp: 0,
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
}
