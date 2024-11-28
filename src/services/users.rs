use crate::models::users::{NewUser, UpdateUser, User};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::users::UserRepository;

pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn create_user(&self, user_dto: NewUser) -> Result<User, diesel::result::Error> {
        self.repo.create_user(user_dto)
    }

    pub fn get_user(&self, user_id: i32) -> Result<User, diesel::result::Error> {
        self.repo.get_user(user_id)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> Result<List<User>, diesel::result::Error> {
        self.repo.get_list(pagination)
    }

    pub fn update_user(&self, user_id: i32, user_dto: UpdateUser) -> Result<User, diesel::result::Error> {
        self.repo.update_user(user_id, user_dto)
    }

    pub fn delete_user(&self, user_id: i32) -> Result<(), diesel::result::Error> {
        self.repo.delete_user(user_id)
    }

    pub fn get_user_by_name(&self, name: &str) -> Result<User, diesel::result::Error> {
        self.repo.get_by_name(name)
    }
}
