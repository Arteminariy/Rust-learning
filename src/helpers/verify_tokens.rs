use std::env;
use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use crate::traits::claims::{AccessClaim, RefreshClaim};

pub fn verify_access_token(at: &str) -> Result<TokenData<AccessClaim>, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<AccessClaim>(at, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default())
}

pub fn verify_refresh_token(rt: &str) -> Result<TokenData<RefreshClaim>, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<RefreshClaim>(rt, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default())
}