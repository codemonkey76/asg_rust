use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::error::{AppError, AppResult};

pub fn verify_password(provided_password: &str, password_hash: &str) -> AppResult<bool> {
    let argon2 = Argon2::default();
    match PasswordHash::new(password_hash) {
        Ok(parsed_hash) => {
            if argon2
                .verify_password(provided_password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Err(AppError::InvalidPasswordHash),
    }
}
