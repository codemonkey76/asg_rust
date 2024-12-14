use axum::async_trait;
use sqlx::{postgres::PgRow, prelude::FromRow, PgPool};

use crate::error::{AppError, AppResult};

use super::{List, ListOptions, Paginator};

pub type PgQuery<'q, T> = sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>;

#[async_trait]
pub trait ModelRepository: Sized + for<'r> FromRow<'r, PgRow> + Unpin {
    type CreateModel: Send + Sync;
    type UpdateModel: Send + Sync;

    const TABLE_NAME: &'static str;
    const CREATE_FIELDS: &'static [&'static str];
    const UPDATE_FIELDS: &'static [&'static str];
    const SEARCH_COLUMNS: &'static [&'static str];

    fn create_placeholders() -> String {
        (1..=Self::CREATE_FIELDS.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ")
    }

    async fn create(pool: &PgPool, data: Self::CreateModel) -> AppResult<Self> {
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            Self::TABLE_NAME,
            Self::CREATE_FIELDS.join(", "),
            Self::create_placeholders()
        );

        let query = sqlx::query_as::<_, Self>(&query);

        let query = Self::bind_create(query, data);

        query.fetch_one(pool).await.map_err(AppError::from)
    }

    async fn get(pool: &PgPool, id: i32) -> AppResult<Self> {
        let query = format!("SELECT * FROM {} WHERE id = $1", Self::TABLE_NAME);
        sqlx::query_as::<_, Self>(&query)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    async fn update(pool: &PgPool, id: i32, data: Self::UpdateModel) -> AppResult<Self> {
        let update_fields = Self::UPDATE_FIELDS
            .iter()
            .enumerate()
            .map(|(i, field)| format!("{} = ${}", field, i + 1))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "UPDATE {} SET {} WHERE id = $1 RETURNING *",
            Self::TABLE_NAME,
            update_fields
        );

        let query = sqlx::query_as::<_, Self>(&query);
        let query = Self::bind_update(query, id, data);

        query.fetch_one(pool).await.map_err(AppError::from)
    }

    async fn list(pool: &PgPool, options: &ListOptions) -> AppResult<List<Self>> {
        let search_query = format!("%{}%", options.q.clone().unwrap_or_default());
        let page = options.page.unwrap_or(1);
        let per_page = options.per_page.unwrap_or(10).clamp(10, 1000);
        let offset = (page as i64 - 1) * per_page as i64;

        let where_clause: String = Self::SEARCH_COLUMNS
            .iter()
            .map(|col| format!("{} ILIKE $1", col))
            .collect::<Vec<_>>()
            .join(" OR ");

        let sort_by = options.sort_by.as_deref().unwrap_or("id");
        let sort_order = if options.ascending.unwrap_or(true) {
            "ASC"
        } else {
            "DESC"
        };

        let total_count_query = format!(
            "SELECT COUNT(*) FROM {} WHERE {}",
            Self::TABLE_NAME,
            where_clause
        );
        let total_count: (i64,) = sqlx::query_as(&total_count_query)
            .bind(&search_query)
            .fetch_one(pool)
            .await?;

        let query = format!(
            "SELECT * FROM {} WHERE {} ORDER BY {} {} LIMIT $2 OFFSET $3",
            Self::TABLE_NAME,
            where_clause,
            sort_by,
            sort_order
        );

        let data = sqlx::query_as::<_, Self>(&query)
            .bind(&search_query)
            .bind(per_page as i64)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let total_pages = (total_count.0 as f64 / per_page as f64).ceil() as u16;

        Ok(List {
            data,
            pagination: Paginator {
                current_page: page,
                per_page,
                total_pages,
                total_count: total_count.0 as u16,
            },
        })
    }

    async fn delete(pool: &PgPool, id: i32) -> AppResult<()> {
        let query = format!("DELETE FROM {} WHERE id = $1", Self::TABLE_NAME);
        let _ = sqlx::query(&query)
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    fn bind_create(query: PgQuery<'_, Self>, data: Self::CreateModel) -> PgQuery<'_, Self>;

    fn bind_update(query: PgQuery<'_, Self>, id: i32, data: Self::UpdateModel)
        -> PgQuery<'_, Self>;
}
