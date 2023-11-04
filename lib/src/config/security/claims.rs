use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    email: String,
    exp: usize,
}

impl Claims {
    fn new(sub: i32, email: String) -> Self {
        // TODO: Make this configurable
        let exp = chrono::Utc::now().timestamp() + 60 * 60 * 24 * 365; // 365 days
        Self {
            sub,
            email,
            exp: exp as usize,
        }
    }
}
