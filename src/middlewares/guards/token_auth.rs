use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use crate::error::{CustomError, ErrorResponse};
use crate::helpers::decode_token::decode_token;
use crate::traits::claims::AccessClaim;

pub struct TokenAuth {
    pub user_id: i32,
}
#[async_trait]
impl<'r> FromRequest<'r> for TokenAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            request.local_cache(|| ErrorResponse::from(CustomError::Unauthorized("No access token".to_string())));
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].split(' ').collect::<Vec<&str>>()[1]; // Removing "Bearer"
        match decode_token::<AccessClaim>(token) {
            Ok(token_data) => Outcome::Success(TokenAuth {
                user_id: token_data.claims.id,
            }),
            Err(_) => {
                request.local_cache(|| ErrorResponse::from(CustomError::Unauthorized("Invalid access token".to_string())));
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}