use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString};
use argon2::password_hash::rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}