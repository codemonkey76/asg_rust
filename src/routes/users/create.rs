use crate::validators::password_rules;
use crate::{
    app_state::SharedAppState,
    auth::security::hash_password,
    model::{
        repository::ModelRepository,
        users::{User, UserForCreate},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub enum CreateUserResponse {
    Success { data: User },
    Error { error: String },
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 80))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 3, max = 64), custom(function = "password_rules"))]
    password: String,
}

#[debug_handler]
pub async fn create(
    State(state): State<SharedAppState>,
    Valid(Json(payload)): Valid<Json<CreateUserRequest>>,
) -> impl IntoResponse {
    let hashed_password = match hash_password(&payload.password, &state.app_key) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateUserResponse::Error {
                    error: "failed to hash password".to_string(),
                }),
            );
        }
    };

    let user_for_create = UserForCreate {
        name: payload.name,
        email: payload.email,
        hashed_password,
    };

    match User::create(&state.db_pool, user_for_create).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(CreateUserResponse::Success { data: user }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateUserResponse::Error {
                error: "failed to create user".to_string(),
            }),
        ),
    }
}
