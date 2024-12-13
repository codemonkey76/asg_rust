use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::SharedAppState,
    auth::{self, jwt::generate_jwt},
    error::AppError,
    model::users::User,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    Success { token: String },
    Error { error: String },
}

impl LoginResponse {
    pub fn success(token: &str) -> Self {
        LoginResponse::Success {
            token: token.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        LoginResponse::Error {
            error: message.to_string(),
        }
    }
}

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<LoginResponse>) {
    (status, Json(LoginResponse::error(message)))
}

#[cfg(not(feature = "deploy"))]
pub async fn login(
    State(state): State<SharedAppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let generic_error = error_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        "An unexpected error occurred",
    );
    let unauthorized_error = error_response(StatusCode::UNAUTHORIZED, "Invalid credentials");

    let (user_id, password_hash) =
        match User::get_password_hash(&state.db_pool, &payload.email).await {
            Ok(data) => data,
            Err(AppError::UserNotFound(_)) => return unauthorized_error,
            Err(e) => {
                tracing::error!("Database error: {:?}", e);
                return generic_error;
            }
        };

    match auth::security::verify_password(&payload.password, &password_hash) {
        Ok(true) => match generate_jwt(&user_id.to_string(), &state.app_key) {
            Ok(token) => (StatusCode::OK, Json(LoginResponse::success(&token))),
            Err(e) => {
                tracing::error!("JWT generation error: {:?}", e);
                generic_error
            }
        },
        Ok(false) => unauthorized_error,
        Err(e) => {
            tracing::error!("Password verification error: {:?}", e);
            generic_error
        }
    }
}
