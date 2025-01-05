use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaim {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaim {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub exp: usize,
}