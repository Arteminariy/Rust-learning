use uuid::Uuid;
use crate::error::CustomError;
use crate::models::users::{UpdateUserDto, UserDto};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::users::UserRepository;
use crate::wrappers::handle_result::ServiceResult;

pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn get_one(&self, user_id: Uuid) -> ServiceResult<UserDto> {
        self.repo.get_one(user_id).map(UserDto::from).map_err(CustomError::from)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> ServiceResult<List<UserDto>> {
        self.repo.get_list(pagination).map(|list| {
            let items: Vec<UserDto> = list.items.into_iter().map(UserDto::from).collect();
            List { pagination: list.pagination, items }
        }).map_err(CustomError::from)
    }

    pub fn update(&self, user_id: Uuid, user_dto: UpdateUserDto) -> ServiceResult<UserDto> {
        self.repo.update(user_id, user_dto).map(UserDto::from).map_err(CustomError::from)
    }

    pub fn delete(&self, user_id: Uuid) -> ServiceResult<()> {
        self.repo.delete(user_id).map_err(CustomError::from)
    }
}
