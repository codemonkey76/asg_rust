use crate::{
    error::{AppError, AppResult},
    security::hash_password,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

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
impl User {
    pub async fn create(pool: &PgPool, name: &str, email: &str, password: &str) -> AppResult<Self> {
        let hashed_password = hash_password(password)?;

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
            name,
            email,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

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
