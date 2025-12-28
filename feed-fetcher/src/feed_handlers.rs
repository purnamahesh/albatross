use sqlx::{Pool, Postgres};
use uuid::Uuid;

// use sqlx::postgres::PgQueryResult;

use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use models::rest::Feed;

pub async fn subscribe_feed(
    Extension(conn): Extension<Pool<Postgres>>,
    Json(body): Json<Feed>,
) -> Response {
    let result = sqlx::query(
        "INSERT INTO feed (id, url, title, description) values (gen_random_uuid(), $1, $2, $3);",
    )
    .bind(body.url.as_str())
    .bind(body.title.as_str())
    .bind(body.description)
    .execute(&conn)
    .await;

    match result {
        Ok(_result_set) => (StatusCode::CREATED, "Subscribed to feed".to_string()).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };

    (StatusCode::CREATED).into_response()
}

pub async fn list_subscribed_feed(Extension(conn): Extension<Pool<Postgres>>) -> Response {
    let result = sqlx::query_as::<_, models::db::Feed>("SELECT * FROM feed;")
        .fetch_all(&conn)
        .await;

    match result {
        Ok(subed_feeds) => return (StatusCode::CREATED, Json(subed_feeds)).into_response(),
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
}

pub async fn unsubscribe_feed(
    Path(id): Path<Uuid>,
    Extension(conn): Extension<Pool<Postgres>>,
) -> Response {
    let result = sqlx::query("DELETE FROM feed where id = $1;")
        .bind(id)
        .execute(&conn)
        .await;

    match result {
        Ok(affected_rows) => {
            return {
                if affected_rows.rows_affected() > 0 {
                    (StatusCode::OK, format!("Unsubscribed from feed {}", id)).into_response()
                } else {
                    (StatusCode::NOT_FOUND, format!("feed {} not found", id)).into_response()
                }
            };
        }
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
}
