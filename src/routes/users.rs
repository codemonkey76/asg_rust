use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{
    app_state::SharedAppState,
    model::{repository::ModelRepository, users::User, List, ListOptions, Paginator},
};

pub async fn list(
    State(state): State<SharedAppState>,
    Query(options): Query<ListOptions>,
) -> impl IntoResponse {
    match User::list(&state.db_pool, &options).await {
        Ok(data) => Json(data),
        Err(_) => Json(List {
            data: vec![],
            pagination: Paginator {
                current_page: options.page.unwrap_or(1),
                per_page: options.per_page.unwrap_or(1),
                total_pages: 0,
                total_count: 0,
            },
        }),
    }
}

#[derive(Serialize)]
pub enum GetUserResponse {
    User(User),
    Error,
}

pub async fn get(State(state): State<SharedAppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match User::get(&state.db_pool, id).await {
        Ok(data) => Json(GetUserResponse::User(data)),
        Err(_) => Json(GetUserResponse::Error),
    }
}
