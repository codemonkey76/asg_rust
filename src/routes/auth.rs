use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, model::users::User};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login_handler(Json(paylod): Json<LoginRequest>) -> impl IntoResponse {
    match User::get_password_hash(&state.db_pool, &payload.email).await {
        Ok(password_hash) => {}
        Err(AppError::UserNotFound(_)) => (StatusCode::UNAUTHORIZED, "Invalid credentials".into()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unexpected error occurred".into(),
        ),
    }
}
