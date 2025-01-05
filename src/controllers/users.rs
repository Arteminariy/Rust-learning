use rocket::{get, put, delete, routes, Route, State};
use rocket::serde::json::Json;
use uuid::{Uuid};
use crate::error::ErrorResponse;
use crate::services::users::UserService;
use crate::middlewares::guards::token_auth::TokenAuth;
use crate::wrappers::handle_result::{handle_delete_result, handle_result, ControllerResult, NoContentResult};
use crate::models::users::{UpdateUser, User};
use crate::pagination::{List, ResponsePagination};


#[get("/users/<user_id>")]
pub fn get_user(user_service: &State<UserService>, user_id: String, _token: TokenAuth) -> ControllerResult<User> {
    match Uuid::parse_str(&user_id) {
        Ok(user_id) => handle_result(user_service.get_user(user_id)),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

#[get("/users?<size>&<page>")]
pub fn get_list(user_service: &State<UserService>, size: Option<i64>, page: Option<i64>, _token: TokenAuth) -> ControllerResult<List<User>> {
    let pagination = ResponsePagination {
        size: size.unwrap_or(10),
        page: page.unwrap_or(1),
    };
    handle_result(user_service.get_list(pagination))
}

#[put("/users/<user_id>", format = "json", data = "<user_dto>")]
pub fn update_user(user_service: &State<UserService>, user_id: String, user_dto: Json<UpdateUser>, _token: TokenAuth) -> ControllerResult<User> {
    match Uuid::parse_str(&user_id) {
        Ok(user_id) => handle_result(user_service.update_user(user_id, user_dto.into_inner())),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

#[delete("/users/<user_id>")]
pub fn delete_user(user_service: &State<UserService>, user_id: String, _token: TokenAuth) -> NoContentResult {
    match Uuid::parse_str(&user_id) {
        Ok(user_id) => handle_delete_result(user_service.delete_user(user_id)),
        Err(e) => Err(ErrorResponse { code: 400, message: e.to_string() })
    }
}

pub fn user_routes() -> Vec<Route> {
    routes![get_user, get_list, update_user, delete_user]
}
