use std::sync::Arc;

use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
    pub app_key: Vec<u8>,
}

pub type SharedAppState = Arc<AppState>;
