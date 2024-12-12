use crate::{auth::claims::Claims, error::AppResult};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_jwt(user_id: &str, secret: &str) -> AppResult<String> {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // Valid for 1 hour

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}
