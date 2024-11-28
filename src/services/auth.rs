use crate::traits::login_data::LoginData;
use crate::traits::token_response::TokenResponse;
use crate::error::CustomError;
use crate::helpers::generate_tokens::generate_tokens;
use crate::helpers::hash_password::hash_password;
use crate::helpers::verify_password::verify_password;
use crate::models::users::NewUser;
use crate::services::users::UserService;

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
        Ok(generate_tokens(&user).unwrap())
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
}

