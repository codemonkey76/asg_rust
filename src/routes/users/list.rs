use crate::app_state::SharedAppState;
use crate::model::{repository::ModelRepository, users::User, List, ListOptions, Paginator};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
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
