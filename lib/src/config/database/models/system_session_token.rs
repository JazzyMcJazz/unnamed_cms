use serde::Deserialize;

use super::{Session, SessionId};

#[derive(Debug, Deserialize)]
pub struct SessionIdToken {
    pub session: SessionId,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SessionToken {
    pub session: Session,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
