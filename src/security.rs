use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::error::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(&password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}
