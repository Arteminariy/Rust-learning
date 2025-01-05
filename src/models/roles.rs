use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::roles;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "roles"]
#[serde(rename_all = "camelCase")]
pub struct NewRole {
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "roles"]
#[serde(rename_all = "camelCase")]
pub struct UpdateRole {
    pub name: String,
}