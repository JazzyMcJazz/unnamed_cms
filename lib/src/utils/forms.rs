use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}
