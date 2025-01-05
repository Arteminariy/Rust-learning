use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use uuid::Uuid;
use crate::models::roles::{NewRole, Role, UpdateRole};
use crate::pagination::{List, RequestPagination, ResponsePagination};
use crate::schema::{roles};

pub struct RolesRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl RolesRepository {
    pub fn create_role(&self, role_dto: NewRole) -> Result<Role, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        diesel::insert_into(roles::table)
            .values(&role_dto)
            .get_result::<Role>(&conn)
    }

    pub fn get_role(&self, role_id: Uuid) -> Result<Role, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        roles::table.find(role_id).get_result::<Role>(&conn)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> Result<List<Role>, diesel::result::Error> {
        let conn = self.pool.get().unwrap();

        let total_count: i64 = roles::table
            .count()
            .get_result(&conn)?;

        let items: Vec<Role> = roles::table
            .limit(pagination.size)
            .offset((pagination.page - 1) * pagination.size)
            .load::<Role>(&conn)?;

        let total_pages = (total_count as f64 / pagination.size as f64).ceil() as i64;

        Ok(List {
            pagination: RequestPagination {
                total_count,
                total_pages,
            },
            items,
        })
    }

    pub fn update_role(&self, role_id: Uuid, role_dto: UpdateRole) -> Result<Role, diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        diesel::update(roles::table.filter(roles::id.eq(role_id))).set(&role_dto).get_result::<Role>(&conn)
    }

    pub fn delete_role(&self, role_id: Uuid) -> Result<(), diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        let rows_deleted = diesel::delete(roles::table.filter(roles::id.eq(role_id))).execute(&conn)?;
        if rows_deleted == 0 {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(())
        }
    }
}
