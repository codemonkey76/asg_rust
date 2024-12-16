use serde::{Deserialize, Serialize};

pub mod permissions;
pub mod repository;
pub mod roles;
pub mod users;

#[derive(Deserialize)]
pub struct ListOptions {
    pub q: Option<String>,
    pub page: Option<u16>,
    pub per_page: Option<u16>,
    pub sort_by: Option<String>,
    pub ascending: Option<bool>,
}

//#[async_trait]
//pub trait Model: for<'r> FromRow<'r, PgRow> + Unpin + Sized {
//    async fn list(pool: &PgPool, options: &ListOptions) -> AppResult<List<Self>> {
//        let search_query = format!("%{}%", options.q.clone().unwrap_or_default());
//        let page = options.page.unwrap_or(1);
//        let per_page = options.per_page.unwrap_or(10).clamp(10, 1000);
//        let offset = (page as i64 - 1) * per_page as i64;
//
//        let where_clause: String = Self::search_columns()
//            .iter()
//            .map(|col| format!("{} ILIKE $1", col))
//            .collect::<Vec<_>>()
//            .join(" OR ");
//
//        let sort_by = options.sort_by.as_deref().unwrap_or("id");
//        let sort_order = if options.ascending.unwrap_or(true) {
//            "ASC"
//        } else {
//            "DESC"
//        };
//
//        let total_count_query = format!(
//            "SELECT COUNT(*) FROM {} WHERE {}",
//            Self::table_name(),
//            where_clause
//        );
//        let total_count: (i64,) = sqlx::query_as(&total_count_query)
//            .bind(&search_query)
//            .fetch_one(pool)
//            .await?;
//
//        let query = format!(
//            "SELECT * FROM {}
//            WHERE {}
//            ORDER BY {} {}
//            LIMIT $2 OFFSET $3",
//            Self::table_name(),
//            where_clause,
//            sort_by,
//            sort_order
//        );
//
//        let data = sqlx::query_as::<_, Self>(&query)
//            .bind(&search_query)
//            .bind(per_page as i64)
//            .bind(offset)
//            .fetch_all(pool)
//            .await?;
//
//        let total_pages = (total_count.0 as f64 / per_page as f64).ceil() as u16;
//        Ok(List {
//            data,
//            pagination: Paginator {
//                current_page: page,
//                per_page,
//                total_pages,
//                total_count: total_count.0 as u16,
//            },
//        })
//    }
//    async fn create(pool: &PgPool) -> AppResult<Self> {}
//
//    fn search_columns() -> &'static [&'static str];
//    fn table_name() -> &'static str;
//}

#[derive(Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub pagination: Paginator,
}

#[derive(Serialize)]
pub struct Paginator {
    pub current_page: u16,
    pub per_page: u16,
    pub total_pages: u16,
    pub total_count: u16,
}
