use std::{env, sync::Arc};

use crate::{
    app_state::{config::get_app_key, AppState},
    auth,
};
use axum::{routing::post, Router};
use sqlx::PgPool;
pub async fn initialize_app() -> Router {
    // Decode APP_KEY
    let app_key = get_app_key().expect("Failed to decode APP_KEY");

    // Initialize Database Pool
    let db_pool = PgPool::connect(env::var("DATABASE_URL"))
        .await
        .expect("Failed to connect to the database");

    // Create Shared Application State
    let app_state = Arc::new(AppState { db_pool, app_key });

    Router::new()
        .route("/login", post(auth::login_handler))
        .with_state(app_state)
}
