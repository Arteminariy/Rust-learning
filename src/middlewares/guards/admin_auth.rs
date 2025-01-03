use rocket::{async_trait, request, Request};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use crate::error::{CustomError, ErrorResponse};
use crate::helpers::decode_token::decode_token;
use crate::traits::claims::AccessClaim;

pub struct AdminAuth {
    pub user_id: i32,
}
#[async_trait]
impl<'r> FromRequest<'r> for AdminAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            request.local_cache(|| ErrorResponse::from(CustomError::Unauthorized("No access token".to_string())));
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].split(' ').collect::<Vec<&str>>()[1]; // Removing "Bearer"

        match decode_token::<AccessClaim>(token) {
            Ok(data) => {
                match data.claims.role_id {
                    Some(role_id) => {
                        match role_id {
                            1 => Outcome::Success(AdminAuth {
                                user_id: data.claims.id,
                            }),
                            _ => {
                                request.local_cache(|| ErrorResponse::from(CustomError::Forbidden("Not an admin".to_string())));
                                Outcome::Error((Status::Forbidden, ()))
                            }
                        }
                    }
                    None => {
                        request.local_cache(|| ErrorResponse::from(CustomError::Forbidden("No role".to_string())));
                        Outcome::Error((Status::Forbidden, ()))
                    }
                }
            }
            Err(_) => {
                request.local_cache(|| ErrorResponse::from(CustomError::Unauthorized("Invalid access token".to_string())));
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}