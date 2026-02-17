use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_valid::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegister {
    #[validate(max_length = 10)]
    #[validate(min_length = 6)]
    #[validate(pattern = r"^[A-Za-z0-9_]+$", message = "username should only contain alphanumeric and _ characters.")]
    pub username: String,
    #[validate(max_length = 16)]
    #[validate(min_length = 8)]
    #[validate(pattern = r"^[^\s]+$", message = "password shouldn't contain any spaces.")]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserRegisterResponse {
    pub username: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserLogin {
    #[validate(max_length = 10)]
    #[validate(min_length = 6)]
    #[validate(pattern = r"^[A-Za-z0-9_]+$", message = "username should only contain alphanumeric and _ characters.")]
    pub username: String,
    #[validate(max_length = 16)]
    #[validate(min_length = 8)]
    #[validate(pattern = r"^[^\s]+$", message = "password shouldn't contain any spaces.")]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct Claim {
    pub exp: usize,
    pub sub: String,
}
