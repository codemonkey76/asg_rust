use serde::Serialize;
use sqlx::prelude::FromRow;

use super::repository::{ModelRepository, PgQuery};

#[derive(Serialize, Debug, FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

pub struct RoleForCreate {
    pub name: String,
}

impl ModelRepository for Role {
    type CreateModel = RoleForCreate;

    type UpdateModel = RoleForCreate;

    const TABLE_NAME: &'static str = "roles";

    const CREATE_FIELDS: &'static [&'static str] = &["name"];

    const UPDATE_FIELDS: &'static [&'static str] = &["name"];

    const SEARCH_COLUMNS: &'static [&'static str] = &["name"];

    fn bind_create(query: PgQuery<'_, Self>, data: Self::CreateModel) -> PgQuery<'_, Self> {
        query.bind(data.name)
    }

    fn bind_update(
        query: PgQuery<'_, Self>,
        id: i32,
        data: Self::UpdateModel,
    ) -> PgQuery<'_, Self> {
        query.bind(id).bind(data.name)
    }
}