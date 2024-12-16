use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub app_key: Vec<u8>,
}

pub type SharedAppState = Arc<AppState>;

impl FromRef<SharedAppState> for AppState {
    fn from_ref(shared: &SharedAppState) -> Self {
        (**shared).clone()
    }
}
