use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub name: String,
    pub password: String,
}