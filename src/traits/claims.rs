use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaim {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaim {
    pub name: String,
    pub age: i32,
    pub is_married: bool,
    pub role_id: Option<i32>,
    pub exp: usize,
}