use std::env;
use std::time::SystemTime;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::traits::claims::AccessClaim;
use crate::models::users::User;
use crate::traits::claims::RefreshClaim;
use crate::traits::token_response::TokenResponse;

pub fn generate_tokens(user: &User) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let token_secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let access_expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 300;
    let refresh_expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 3600;
    let access_claim = AccessClaim {
        id: user.id.clone(),
        name: user.name.clone(),
        age: user.age.clone(),
        is_married: user.is_married.clone(),
        role_id: user.role_id.clone(),
        exp: access_expiration as usize,
    };
    let refresh_claim = RefreshClaim {
        sub: user.name.clone(),
        exp: refresh_expiration as usize,
    };

    let access_token = encode(&Header::default(), &access_claim, &EncodingKey::from_secret(token_secret.as_bytes()))?;
    let refresh_token = encode(&Header::default(), &refresh_claim, &EncodingKey::from_secret(token_secret.as_bytes()))?;
    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}