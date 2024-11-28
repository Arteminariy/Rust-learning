use crate::traits::login_data::LoginData;
use crate::traits::token_response::TokenResponse;
use crate::error::CustomError;
use crate::helpers::generate_tokens::generate_tokens;
use crate::helpers::hash_password::hash_password;
use crate::helpers::verify_password::verify_password;
use crate::helpers::verify_tokens::verify_refresh_token;
use crate::models::users::NewUser;
use crate::services::users::UserService;
use crate::traits::refresh_data::RefreshData;

pub struct AuthService {
    pub user_service: UserService,
}

impl AuthService {
    pub fn register(&self, new_user: NewUser) -> Result<TokenResponse, CustomError> {
        let pass_hash: String = hash_password(&new_user.password_hash).expect("hashing password error occurred");
        let new_user = NewUser {
            password_hash: pass_hash,
            ..new_user
        };
        let user = self.user_service.create_user(new_user).map_err(|e| CustomError::InternalServerError(e.to_string())).expect("Failed to create user in register");
        match generate_tokens(&user) {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(CustomError::InternalServerError(e.to_string()))
        }
    }

    pub fn login(&self, login_data: LoginData) -> Result<TokenResponse, CustomError> {
        let user = self.user_service.get_user_by_name(&login_data.name);

        match user {
            Ok(user) => {
                match verify_password(&user.password_hash, &login_data.password) {
                    Ok(_) => Ok(generate_tokens(&user).unwrap()),
                    Err(_) => Err(CustomError::Unauthorized("Invalid login data".into()))
                }
            }
            Err(_) => Err(CustomError::NotFound("User not found".to_string()))
        }
    }

    pub fn refresh(&self, refresh_data: RefreshData) -> Result<TokenResponse, CustomError> {
        match verify_refresh_token(&refresh_data.refresh_token) {
            Ok(data) => {
                match self.user_service.get_user_by_name(&data.claims.sub) {
                    Ok(user) => {
                        Ok(generate_tokens(&user).unwrap())
                    }
                    Err(e) => {
                        Err(CustomError::NotFound(e.to_string()))
                    }
                }
            }
            Err(e) => Err(CustomError::Unauthorized(e.to_string()))
        }
    }
}

