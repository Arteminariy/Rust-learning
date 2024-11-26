use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub is_married: bool,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub name: String,
    pub age: i32,
    pub is_married: bool,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub name: String,
    pub age: i32,
    pub is_married: bool,
}