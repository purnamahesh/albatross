use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feed {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub feed_id: Uuid,
    pub title: String,
    pub url: String,
    pub published: DateTime<Utc>,
    pub content: String,
}

// #[derive(Deserialize)]
// struct ArticleQuery {
//     feed_id: Option<Uuid>,
//     unread_only: Option<bool>,
//     limit: Option<i64>,
//     offset: Option<i64>,
// }
