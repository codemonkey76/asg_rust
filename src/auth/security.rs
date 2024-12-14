use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use base64::{engine::general_purpose, Engine};
use rand::thread_rng;

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

pub fn hash_password(
    password: &str,
    app_key: &[u8],
) -> Result<String, argon2::password_hash::Error> {
    let decoded_app_key = general_purpose::STANDARD
        .decode(app_key)
        .expect("Invalid Base64-ncoded APP_KEY");
    dbg!(&decoded_app_key.len());

    let mut rng = thread_rng();
    let unique_salt = SaltString::generate(&mut rng);
    let unique_salt = unique_salt.as_str();
    dbg!(&unique_salt.len());

    let mut combined_raw_salt = Vec::new();
    combined_raw_salt.extend_from_slice(&decoded_app_key);
    combined_raw_salt.extend_from_slice(unique_salt.as_bytes());
    dbg!(&combined_raw_salt.len());

    let combined_salt_b64 = general_purpose::STANDARD_NO_PAD.encode(&combined_raw_salt);

    dbg!(&combined_salt_b64.len());

    let combined_salt = SaltString::from_b64(&combined_salt_b64)?;
    dbg!(&combined_salt);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &combined_salt)?
        .to_string();

    Ok(password_hash)
}
