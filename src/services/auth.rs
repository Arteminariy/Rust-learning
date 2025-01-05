use uuid::Uuid;
use crate::traits::login_data::LoginData;
use crate::traits::token_response::TokenResponse;
use crate::error::CustomError;
use crate::helpers::decode_token::decode_token;
use crate::helpers::generate_tokens::generate_tokens;
use crate::helpers::hash_password::hash_password;
use crate::helpers::verify_password::verify_password;
use crate::models::users::{NewUser};
use crate::services::users::UserService;
use crate::traits::change_password_dto::ChangePasswordDto;
use crate::traits::claims::RefreshClaim;
use crate::traits::refresh_data::RefreshData;
use crate::wrappers::handle_result::ServiceResult;

pub struct AuthService {
    pub user_service: UserService,
}

impl AuthService {
    pub fn register(&self, new_user: NewUser) -> ServiceResult<TokenResponse> {
        let pass_hash: String = hash_password(&new_user.password_hash).expect("hashing password error occurred");
        let new_user = NewUser {
            password_hash: pass_hash,
            ..new_user
        };
        let user = self.user_service.create_user(new_user).expect("Failed to create user in register");
        match generate_tokens(&user) {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(CustomError::InternalServerError(e.to_string()))
        }
    }

    pub fn login(&self, login_data: LoginData) -> ServiceResult<TokenResponse> {
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

    pub fn refresh(&self, refresh_data: RefreshData) -> ServiceResult<TokenResponse> {
        match decode_token::<RefreshClaim>(&refresh_data.refresh_token) {
            Ok(data) => {
                match self.user_service.get_user_by_name(&data.claims.sub) {
                    Ok(user) => {
                        Ok(generate_tokens(&user).unwrap())
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(CustomError::Unauthorized(e.to_string()))
        }
    }

    pub fn change_password(&self, user_id: Uuid, data: ChangePasswordDto) -> ServiceResult<()> {
        let user = self.user_service.get_user(user_id)?;
        match verify_password(&user.password_hash, &data.old_password) {
            Ok(_) => {
                let new_password_hash = hash_password(&data.new_password).expect("hashing new password error occurred");
                let user = self.user_service.get_user(user_id);
                match user {
                    Ok(_) => {
                        let updated_user = self.user_service.change_password(user_id, new_password_hash);

                        match updated_user {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e)
                        }
                    }
                    Err(e) => Err(e)
                }
            }
            Err(_) => Err(CustomError::Unauthorized("Invalid old password".to_string()))
        }
    }
}

