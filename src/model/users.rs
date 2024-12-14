use crate::error::{AppError, AppResult};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

use super::repository::{ModelRepository, PgQuery};

#[derive(Serialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub password: String,
    pub customer_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub remember_token: Option<String>,
}

pub struct UserForCreate {
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}

pub struct UserForUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub customer_id: Option<String>,
}

#[cfg(not(feature = "deploy"))]
impl User {
    pub async fn set_email_verified_at(pool: &PgPool, id: i32) -> AppResult<()> {
        sqlx::query!(
            "UPDATE users SET email_verified_at = now() WHERE id = $1",
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn get_password_hash(pool: &PgPool, email: &str) -> AppResult<(i32, String)> {
        let result = sqlx::query!("SELECT id, password FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await;

        match result {
            Ok(record) => Ok((record.id, record.password)),
            Err(_) => Err(AppError::UserNotFound(email.to_string())),
        }
    }
}

impl ModelRepository for User {
    type CreateModel = UserForCreate;

    type UpdateModel = UserForUpdate;

    const TABLE_NAME: &'static str = "users";

    const CREATE_FIELDS: &'static [&'static str] = &["name", "email", "password"];

    const UPDATE_FIELDS: &'static [&'static str] = &["name", "email", "password", "customer_id"];

    const SEARCH_COLUMNS: &'static [&'static str] = &["name", "email"];

    fn bind_create(query: PgQuery<'_, Self>, data: Self::CreateModel) -> PgQuery<'_, Self> {
        query
            .bind(data.name)
            .bind(data.email)
            .bind(data.hashed_password)
    }

    fn bind_update(
        query: PgQuery<'_, Self>,
        id: i32,
        data: Self::UpdateModel,
    ) -> PgQuery<'_, Self> {
        query
            .bind(id)
            .bind(data.name)
            .bind(data.email)
            .bind(data.hashed_password)
            .bind(data.customer_id)
    }
}
