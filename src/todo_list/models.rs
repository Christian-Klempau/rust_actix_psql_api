use serde::Deserialize;
use sqlx::{self, FromRow};

#[derive(Deserialize, Clone, FromRow)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone, FromRow)]
pub struct UpdateEntryData {
    pub title: String,
}
