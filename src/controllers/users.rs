use rocket::{get, post, put, delete, routes, Route, State};
use rocket::serde::json::Json;
use crate::services::users::UserService;
use crate::error::{ErrorResponse};
use crate::wrappers::handle_result::handle_result;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::pagination::{List, ResponsePagination};

#[get("/users/<user_id>")]
pub fn get_user(user_service: &State<UserService>, user_id: String) -> Result<Json<User>, ErrorResponse> {
    let user_id = user_id.parse::<i32>().unwrap();
    handle_result(user_service.get_user(user_id))
}

#[get("/users?<size>&<page>")]
pub fn get_list(user_service: &State<UserService>, size: Option<i64>, page: Option<i64>) -> Result<Json<List<User>>, ErrorResponse> {
    let pagination = ResponsePagination {
        size: size.unwrap_or(10),
        page: page.unwrap_or(1),
    };
    handle_result(user_service.get_list(pagination))
}

#[put("/users/<user_id>", format = "json", data = "<user_dto>")]
pub fn update_user(user_service: &State<UserService>, user_id: String, user_dto: Json<UpdateUser>) -> Result<Json<User>, ErrorResponse> {
    let user_id = user_id.parse::<i32>().unwrap();
    handle_result(user_service.update_user(user_id, user_dto.into_inner()))
}

#[delete("/users/<user_id>")]
pub fn delete_user(user_service: &State<UserService>, user_id: String) -> Result<Json<()>, ErrorResponse> {
    let user_id = user_id.parse::<i32>().unwrap();
    handle_result(user_service.delete_user(user_id))
}

pub fn user_routes() -> Vec<Route> {
    routes![get_user, get_list, update_user, delete_user]
}
