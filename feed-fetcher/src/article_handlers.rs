use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use models::db::Article;
use sqlx::{Pool, Postgres};

#[axum::debug_handler]
pub async fn list_articles(Extension(conn): Extension<Pool<Postgres>>) -> Response {
    let result = sqlx::query_as::<_, Article>("SELECT * FROM article;")
        .fetch_all(&conn)
        .await;

    match result {
        Ok(articles) => (StatusCode::CREATED, Json(articles)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
