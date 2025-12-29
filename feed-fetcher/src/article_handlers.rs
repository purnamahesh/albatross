use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use models::{db::Article, rest::ArticleQuery};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn list_articles(
    Extension(conn): Extension<Pool<Postgres>>,
    Query(query_params): Query<ArticleQuery>,
) -> Response {
    let mut query = " SELECT * FROM article where 1=1 ".to_string();

    if query_params.feed_id.is_some() {
        query = query + " and feed_id = \'" + &query_params.feed_id.unwrap().to_string() + "\'"
    }
    if query_params.unread_only.is_some() {
        query = query + " and read = " + &(!query_params.unread_only.unwrap()).to_string()
    }
    if query_params.limit.is_some() {
        query = query + " limit " + &query_params.limit.unwrap().to_string()
    }
    if query_params.offset.is_some() {
        query = query + " offset " + &query_params.offset.unwrap().to_string()
    }

    let result = sqlx::query_as::<_, Article>(&query).fetch_all(&conn).await;

    match result {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn list_feed_articles(
    Path(feed_id): Path<Uuid>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> Response {
    let result = sqlx::query_as::<_, Article>("SELECT * FROM article where feed_id = $1;")
        .bind(feed_id)
        .fetch_all(&conn)
        .await;

    match result {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn get_article(
    Path(id): Path<Uuid>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> Response {
    let result = sqlx::query_as::<_, Article>("SELECT * FROM article where id = $1;")
        .bind(id)
        .fetch_one(&conn)
        .await;

    match result {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn article_mark_read(
    Path(id): Path<Uuid>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> Response {
    let result = sqlx::query("UPDATE article SET read = true where id = $1;")
        .bind(id)
        .execute(&conn)
        .await;

    match result {
        Ok(affected_rows) => {
            if affected_rows.rows_affected() > 0 {
                (StatusCode::OK, format!("Article {} marked as read", id)).into_response()
            } else {
                (StatusCode::NOT_FOUND, format!("Article {} not found", id)).into_response()
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
