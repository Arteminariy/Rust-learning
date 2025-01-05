use rocket::{delete, get, post, put, routes, Route, State};
use rocket::serde::json::Json;
use uuid::{Uuid};
use crate::error::ErrorResponse;
use crate::middlewares::guards::admin_auth::AdminAuth;
use crate::middlewares::guards::token_auth::TokenAuth;
use crate::models::roles::{NewRole, Role, UpdateRole};
use crate::pagination::{List, ResponsePagination};
use crate::services::roles::RolesService;
use crate::wrappers::handle_result::{handle_delete_result, handle_result, ControllerResult, NoContentResult};

#[post("/roles", format = "json", data = "<role_dto>")]
pub fn create_role(roles_service: &State<RolesService>, role_dto: Json<NewRole>, _token: AdminAuth) -> ControllerResult<Role> {
    handle_result(roles_service.create_role(role_dto.into_inner()))
}

#[get("/roles/<role_id>")]
pub fn get_role(roles_service: &State<RolesService>, role_id: String, _token: TokenAuth) -> ControllerResult<Role> {
    match Uuid::parse_str(&role_id) {
        Ok(role_id) => handle_result(roles_service.get_role(role_id)),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

#[get("/roles?<size>&<page>")]
pub fn get_list(roles_service: &State<RolesService>, size: Option<i64>, page: Option<i64>, _token: TokenAuth) -> ControllerResult<List<Role>> {
    let pagination = ResponsePagination {
        size: size.unwrap_or(10),
        page: page.unwrap_or(1),
    };
    handle_result(roles_service.get_list(pagination))
}

#[put("/roles/<role_id>", format = "json", data = "<role_dto>")]
pub fn update_role(roles_service: &State<RolesService>, role_id: String, role_dto: Json<UpdateRole>, _token: AdminAuth) -> ControllerResult<Role> {
    match Uuid::parse_str(&role_id) {
        Ok(role_id) => handle_result(roles_service.update_role(role_id, role_dto.into_inner())),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

#[delete("/roles/<role_id>")]
pub fn delete_role(roles_service: &State<RolesService>, role_id: String, _token: AdminAuth) -> NoContentResult {
    match Uuid::parse_str(&role_id) {
        Ok(role_id) =>  handle_delete_result(roles_service.delete_role(role_id)),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

pub fn roles_routes() -> Vec<Route> {
    routes![create_role, get_role, get_list, update_role, delete_role]
}