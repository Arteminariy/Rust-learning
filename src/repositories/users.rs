use diesel::prelude::*;
use crate::schema::users;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::users::{NewUser, UpdateUser, User};
use crate::pagination::{List, RequestPagination, ResponsePagination};

pub struct UserRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepository {
    pub fn create_user(&self, user_dto: NewUser) -> Result<User, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        diesel::insert_into(users::table)
            .values(&user_dto)
            .get_result(&conn)
    }

    pub fn get_user(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        users::table.find(user_id).get_result(&conn)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> Result<List<User>, diesel::result::Error> {
        let conn = self.pool.get().unwrap();

        let total_count: i64 = users::table
            .count()
            .get_result(&conn)?;

        let items: Vec<User> = users::table
            .limit(pagination.size)
            .offset((pagination.page - 1) * pagination.size)
            .load::<User>(&conn)?;

        let total_pages = (total_count as f64 / pagination.size as f64).ceil() as i64;

        Ok(List {
            pagination: RequestPagination {
                total_count,
                total_pages,
            },
            items,
        })
    }

    pub fn update_user(&self, user_id: i32, user_dto: UpdateUser) -> Result<User, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        diesel::update(users::table.filter(users::id.eq(user_id))).set(&user_dto).get_result(&conn)
    }

    pub fn delete_user(&self, user_id: i32) -> Result<(), diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        let rows_deleted = diesel::delete(users::table.filter(users::id.eq(user_id))).execute(&conn)?;
        if rows_deleted == 0 {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(())
        }
    }
}
