use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct Feed {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub feed_id: Uuid,
    pub title: String,
    pub url: String,
    pub published: DateTime<Utc>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ArticleQuery {
    pub feed_id: Option<Uuid>,
    pub unread_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
