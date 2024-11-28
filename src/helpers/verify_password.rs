use argon2::{Argon2, password_hash::{PasswordVerifier, PasswordHash}};

pub fn verify_password(hash: &str, password: &str) -> argon2::password_hash::Result<()> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}