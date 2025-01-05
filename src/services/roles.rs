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
    pub fn create_role(&self, role_dto: NewRole) -> ServiceResult<Role> {
        self.repo.create_role(role_dto).map_err(CustomError::from)
    }

    pub fn get_role(&self, role_id: Uuid) -> ServiceResult<Role> {
        self.repo.get_role(role_id).map_err(CustomError::from)
    }

    pub fn get_list(&self, pagination: ResponsePagination) -> ServiceResult<List<Role>> {
        self.repo.get_list(pagination).map_err(CustomError::from)
    }

    pub fn update_role(&self, role_id: Uuid, role_dto: UpdateRole) -> ServiceResult<Role> {
        self.repo.update_role(role_id, role_dto).map_err(CustomError::from)
    }

    pub fn delete_role(&self, role_id: Uuid) -> ServiceResult<()> {
        self.repo.delete_role(role_id).map_err(CustomError::from)
    }
}
