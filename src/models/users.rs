use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
#[changeset_options(treat_none_as_null = "true")]
pub struct UpdateUser {
    pub name: String,
    pub role_id: Option<Uuid>,
}