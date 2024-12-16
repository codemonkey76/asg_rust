use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::SharedAppState,
    auth::security::hash_password,
    model::{
        repository::ModelRepository,
        users::{User, UserForCreate},
        List, ListOptions, Paginator,
    },
};

#[derive(Serialize)]
pub enum CreateUserResponse {
    User(User),
    Error(String),
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}
