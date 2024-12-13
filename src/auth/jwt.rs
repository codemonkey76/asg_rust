use crate::{
    auth::claims::Claims,
    error::{AppError, AppResult},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_jwt(user_id: &str, app_key: &[u8]) -> AppResult<String> {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // Valid for 1 hour

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_key),
    )?)
}

pub fn decode_jwt(token: &str, secret: &[u8]) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;

    Ok(token_data.claims)
}
