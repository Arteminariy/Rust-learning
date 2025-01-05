use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use uuid::Uuid;
use crate::helpers::hash_password::hash_password;
use crate::models::roles::NewRole;
use crate::models::users::CreateUserDto;
use crate::schema::{roles, users};

pub fn init_database(pool: Pool<ConnectionManager<PgConnection>>) {
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
        let admin_role_id: Uuid = roles::table
            .filter(roles::name.eq("admin"))
            .select(roles::id)
            .first(&conn)
            .expect("Admin role not found");

        let hashed_password = hash_password("admin123").expect("Failed to hash password");
        let new_user = CreateUserDto {
            name: "admin".to_string(),
            role_id: Some(admin_role_id),
            password_hash: hashed_password,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error creating admin user");
    }
}
