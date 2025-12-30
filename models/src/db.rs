use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, prelude::Type};
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Feed {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub active: bool,
}

#[derive(Debug, Serialize, FromRow, Type)]
pub struct Article {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub url: String,
    pub title: String,
    pub content: String,
    pub read: bool,
    pub published: DateTime<Utc>,
}
