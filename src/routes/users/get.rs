use crate::app_state::SharedAppState;
use crate::model::{repository::ModelRepository, users::User};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub enum GetUserResponse {
    Success(User),
    Error { error: String },
}

pub async fn get(State(state): State<SharedAppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match User::get(&state.db_pool, id).await {
        Ok(data) => (StatusCode::OK, Json(GetUserResponse::Success(data))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(GetUserResponse::Error {
                error: "Not found".to_string(),
            }),
        ),
    }
}
