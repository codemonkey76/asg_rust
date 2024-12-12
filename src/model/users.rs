use crate::{error::AppResult, security::hash_password};
use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub password: String,
    pub customer_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
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
}
