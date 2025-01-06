use std::env;
use std::time::SystemTime;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::traits::claims::AccessClaim;
use crate::models::users::UserModel;
use crate::traits::claims::RefreshClaim;
use crate::traits::token_response::TokenResponse;

pub fn generate_tokens(user: &UserModel) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let token_secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let access_lifetime = env::var("ACCESS_LIFETIME_SECONDS").unwrap_or(300.to_string());
    let refresh_lifetime = env::var("REFRESH_LIFETIME_SECONDS").unwrap_or(3600.to_string());

    let access_expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + access_lifetime.parse::<u64>().unwrap();
    let refresh_expiration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + refresh_lifetime.parse::<u64>().unwrap();
    let access_claim = AccessClaim {
        id: user.id.clone(),
        name: user.name.clone(),
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