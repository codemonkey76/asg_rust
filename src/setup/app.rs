use std::{env, sync::Arc};

use crate::{
    app_state::{config::get_app_key, AppState},
    middleware::{authorization, log_requests},
    routes,
};
use axum::{middleware, routing, Router};
use sqlx::PgPool;
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[cfg(not(feature = "deploy"))]
pub async fn initialize_app() -> Router {
    init_logging();
    // Decode APP_KEY
    let app_key = get_app_key().expect("Failed to decode APP_KEY");

    // Initialize Database Pool
    let db_pool = PgPool::connect(&env::var("DATABASE_URL").unwrap_or("".to_string()))
        .await
        .expect("Failed to connect to the database");

    // Create Shared Application State
    let app_state = Arc::new(AppState { db_pool, app_key });

    // Define public (unauthenticated) routes
    let public_routes = Router::new().route("/login", routing::post(routes::auth::login));

    // Define protected (authenticated) routes
    let protected_routes = Router::new()
        .route("/users", routing::get(routes::users::list))
        .route("/users/:id", routing::get(routes::users::get))
        .route("/users", routing::post(routes::users::create))
        //      .route("/users/:id", routing::put(routes::_users::update))
        //     .route("/users/:id", routing::delete(routes::_users::delete))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            authorization,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(middleware::from_fn(log_requests))
        .with_state(app_state)
}

fn init_logging() {
    let file_log_level = env::var("FILE_LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
    let console_log_level = env::var("CONSOLE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_file = rolling::daily("logs", "request.log");

    let file_layer = fmt::layer()
        .with_writer(log_file)
        .with_filter(EnvFilter::new(file_log_level));

    let console_layer = fmt::layer().with_filter(EnvFilter::new(console_log_level));

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
}
