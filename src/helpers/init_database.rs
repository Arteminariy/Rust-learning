use std::env;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;
use crate::helpers::hash_password::hash_password;
use crate::models::roles::NewRole;
use crate::models::users::NewUser;
use crate::schema::{roles, users};

pub fn init_database(pool: Pool<ConnectionManager<PgConnection>>) {
    dotenv().ok();
    let conn = pool.get().unwrap();

    let roles_count: i64 = roles::table.count().get_result(&conn).unwrap_or(0);

    if roles_count == 0 {
        let new_role = NewRole {
            name: "admin".to_string(),
        };

        diesel::insert_into(roles::table)
            .values(&new_role)
            .execute(&conn)
            .expect("Error creating admin role");
    }

    let users_count: i64 = users::table.count().get_result(&conn).unwrap_or(0);

    if users_count == 0 {
        let username = env::var("INIT_USER_DEFAULT_NAME").unwrap_or("admin".to_string());
        let admin_role_id: i32 = roles::table
            .filter(roles::name.eq(username))
            .select(roles::id)
            .first(&conn)
            .expect("Admin role not found");

        let password = env::var("INIT_USER_DEFAULT_PASSWORD").unwrap_or("admin123".to_string());
        let hashed_password = hash_password(&password).expect("Failed to hash password");
        let new_user = NewUser {
            name: "admin".to_string(),
            age: 0,
            is_married: false,
            role_id: Some(admin_role_id),
            password_hash: hashed_password,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error creating admin user");
    }
}
