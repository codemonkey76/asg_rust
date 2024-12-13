use axum::async_trait;
use sqlx::{postgres::PgRow, prelude::FromRow, PgPool};

use crate::error::{AppError, AppResult};

#[async_trait]
pub trait ModelRepository: Sized + for<'r> FromRow<'r, PgRow> + Unpin {
    type CreateModel: Send + Sync;
    type UpdateModel: Send + Sync;

    async fn create(pool: &PgPool, data: Self::CreateModel) -> AppResult<Self> {
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            Self::table_name(),
            Self::create_fields().join(", "),
            Self::create_placeholders().join(", ")
        );

        let query = sqlx::query_as::<_, Self>(&query);
        let query = Self::bind_create(query, data);

        query.fetch_one(pool).await.map_err(AppError::from)
    }

    async fn get(pool: &PgPool, id: i32) -> AppResult<Self> {
        let query = format!("SELECT * FROM {} WHERE id = $1", Self::table_name());
        sqlx::query_as::<_, Self>(&query)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    fn table_name() -> &'static str;
    fn create_fields() -> Vec<&'static str>;
    fn create_placeholders() -> Vec<&'static str>;
    fn bind_create(
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        data: Self::CreateModel,
    ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
