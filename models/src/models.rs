use std::sync::RwLock;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feed {
    pub id: Option<Uuid>, // TODO: Remove Option<_>
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub feed_id: Option<Uuid>, // TODO: Remove Option<_>
    pub title: String,
    pub link: String,
    pub published: DateTime<Utc>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub subscribed_feeds: RwLock<Vec<Feed>>,
}

#[derive(Deserialize)]
struct ArticleQuery {
    feed_id: Option<Uuid>,
    unread_only: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
}
