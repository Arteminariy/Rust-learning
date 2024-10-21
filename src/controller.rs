use rocket::{get, post, put, delete, routes, Route, State};
use rocket::serde::json::Json;
use crate::error::{CustomError, ErrorResponse};
use crate::hoc::handle_result;
use crate::service::UserService;
use crate::models::{NewUser, UpdateUser, User};
use crate::pagination::{List, RequestPagination, ResponsePagination};

#[post("/users", format = "json", data = "<user_dto>")]
pub fn create_user(user_service: &State<UserService>, user_dto: Json<NewUser>) -> Result<Json<User>, ErrorResponse> {
    handle_result(user_service.create_user(user_dto.into_inner()))
}

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

pub fn routes() -> Vec<Route> {
    routes![create_user, get_user, get_list, update_user, delete_user]
}
