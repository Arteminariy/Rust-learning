use crate::error::CustomError;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::users::UserRepository;
use crate::wrappers::handle_result::ServiceResult;

pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn create_user(&self, user_dto: NewUser) -> ServiceResult<User> {
        self.repo.create_user(user_dto).map_err(CustomError::from)
    }

    pub fn get_user(&self, user_id: i32) -> ServiceResult<User> {
        self.repo.get_user(user_id).map_err(CustomError::from)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> ServiceResult<List<User>> {
        self.repo.get_list(pagination).map_err(CustomError::from)
    }

    pub fn update_user(&self, user_id: i32, user_dto: UpdateUser) -> ServiceResult<User> {
        self.repo.update_user(user_id, user_dto).map_err(CustomError::from)
    }

    pub fn delete_user(&self, user_id: i32) -> ServiceResult<()> {
        self.repo.delete_user(user_id).map_err(CustomError::from)
    }

    pub fn get_user_by_name(&self, name: &str) -> ServiceResult<User> {
        self.repo.get_by_name(name).map_err(CustomError::from)
    }

    pub fn change_password(&self, user_id: i32, new_password_hash: String) -> ServiceResult<User> {
        self.repo.change_password(user_id, new_password_hash).map_err(CustomError::from)
    }
}
