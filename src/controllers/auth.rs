use rocket::{post, put, routes, Route, State};
use rocket::serde::json::Json;
use crate::traits::login_data::LoginData;
use crate::middlewares::guards::token_auth::TokenAuth;
use crate::models::users::{CreateUserDto};
use crate::services::auth::AuthService;
use crate::traits::change_password_dto::ChangePasswordDto;
use crate::traits::refresh_data::RefreshData;
use crate::traits::token_response::TokenResponse;
use crate::wrappers::handle_result::{handle_result, ControllerResult};

#[post("/auth/login", format = "json", data = "<login_dto>")]
pub fn login(auth_service: &State<AuthService>, login_dto: Json<LoginData>) -> ControllerResult<TokenResponse> {
    handle_result(auth_service.login(login_dto.into_inner()))
}

#[post("/auth/register", format = "json", data = "<register_data>")]
pub fn register(auth_service: &State<AuthService>, register_data: Json<CreateUserDto>) -> ControllerResult<TokenResponse> {
    handle_result(auth_service.register(register_data.into_inner()))
}

#[post("/auth/refresh", format = "json", data = "<refresh_data>")]
pub fn refresh(auth_service: &State<AuthService>, refresh_data: Json<RefreshData>) -> ControllerResult<TokenResponse> {
    handle_result(auth_service.refresh(refresh_data.into_inner()))
}

#[put("/auth/change-password", format = "json", data = "<change_password_dto>")]
pub fn change_password(auth_service: &State<AuthService>, token_auth: TokenAuth, change_password_dto: Json<ChangePasswordDto>) -> ControllerResult<()> {
    handle_result(auth_service.change_password(token_auth.user_id, change_password_dto.into_inner()))
}

pub fn auth_routes() -> Vec<Route> {
    routes![login, register, refresh, change_password]
}