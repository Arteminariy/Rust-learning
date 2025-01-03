use rocket::{delete, get, post, put, routes, Route, State};
use rocket::serde::json::Json;
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
    let role_id = role_id.parse::<i32>().unwrap();
    handle_result(roles_service.get_role(role_id))
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
    let role_id = role_id.parse::<i32>().unwrap();
    handle_result(roles_service.update_role(role_id, role_dto.into_inner()))
}

#[delete("/roles/<role_id>")]
pub fn delete_role(roles_service: &State<RolesService>, role_id: String, _token: AdminAuth) -> NoContentResult {
    let role_id = role_id.parse::<i32>().unwrap();
    handle_delete_result(roles_service.delete_role(role_id))
}

pub fn roles_routes() -> Vec<Route> {
    routes![create_role, get_role, get_list, update_role, delete_role]
}