use diesel::{Queryable, Insertable, AsChangeset};
use rocket::serde::{Serialize, Deserialize};
use uuid::{Uuid};
use crate::schema::users;

#[derive(Debug, Queryable)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct CreateUserEntity {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
#[changeset_options(treat_none_as_null = "true")]
pub struct UpdateUserDto {
    pub name: String,
    pub role_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
}

impl From<UserModel> for UserDto {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            role_id: value.role_id,
        }
    }
}
