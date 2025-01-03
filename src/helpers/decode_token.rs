use std::env;
use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};

pub fn decode_token<T: for<'de> serde::Deserialize<'de>>(rt: &str) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<T>(rt, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default())
}