use uuid::Uuid;
use crate::error::CustomError;
use crate::models::roles::{NewRole, Role, UpdateRole};
use crate::pagination::{List, ResponsePagination};
use crate::repositories::roles::RolesRepository;
use crate::wrappers::handle_result::ServiceResult;

pub struct RolesService {
    pub repo: RolesRepository,
}

impl RolesService {
    pub fn create(&self, role_dto: NewRole) -> ServiceResult<Role> {
        self.repo.create(role_dto).map_err(CustomError::from)
    }

    pub fn get_one(&self, role_id: Uuid) -> ServiceResult<Role> {
        self.repo.get_one(role_id).map_err(CustomError::from)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> ServiceResult<List<Role>> {
        self.repo.get_list(pagination).map_err(CustomError::from)
    }

    pub fn update(&self, role_id: Uuid, role_dto: UpdateRole) -> ServiceResult<Role> {
        self.repo.update(role_id, role_dto).map_err(CustomError::from)
    }

    pub fn delete(&self, role_id: Uuid) -> ServiceResult<()> {
        self.repo.delete(role_id).map_err(CustomError::from)
    }
}
