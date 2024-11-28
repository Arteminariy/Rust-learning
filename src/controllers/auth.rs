use rocket::{post, routes, Route, State};
use rocket::serde::json::Json;
use crate::traits::login_data::LoginData;
use crate::error::{CustomError, ErrorResponse};
use crate::models::users::{NewUser};
use crate::services::auth::AuthService;
use crate::traits::refresh_data::RefreshData;
use crate::traits::token_response::TokenResponse;

fn error_matcher(e: CustomError) -> ErrorResponse {
    match e {
        CustomError::NotFound(msg) => ErrorResponse {
            code: 404,
            message: msg,
        },
        CustomError::Unauthorized(msg) => ErrorResponse {
            code: 401,
            message: msg,
        },
        CustomError::Forbidden(msg) => ErrorResponse {
            code: 403,
            message: msg,
        },
        CustomError::UnprocessableEntity(msg) => ErrorResponse {
            code: 422,
            message: msg,
        },
        CustomError::InternalServerError(msg) => ErrorResponse {
            code: 500,
            message: msg,
        }
    }
}

#[post("/auth/login", format = "json", data = "<login_dto>")]
pub fn login(auth_service: &State<AuthService>, login_dto: Json<LoginData>) -> Result<Json<TokenResponse>, ErrorResponse> {
    auth_service.login(login_dto.into_inner()).map(Json).map_err(error_matcher)
}

#[post("/auth/register", format = "json", data = "<register_data>")]
pub fn register(auth_service: &State<AuthService>, register_data: Json<NewUser>) -> Result<Json<TokenResponse>, ErrorResponse> {
    auth_service.register(register_data.into_inner()).map(Json).map_err(error_matcher)
}

#[post("/auth/refresh", format = "json", data = "<refresh_data>")]
pub fn refresh(auth_service: &State<AuthService>, refresh_data: Json<RefreshData>) -> Result<Json<TokenResponse>, ErrorResponse> {
    auth_service.refresh(refresh_data.into_inner()).map(Json).map_err(error_matcher)
}

pub fn auth_routes() -> Vec<Route> {
    routes![login, register, refresh]
}