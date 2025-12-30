use axum::{
    Extension, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use database::pool::create_conn_pool;
use feed_fetcher::feed_handlers::{list_subscribed_feed, subscribe_feed, unsubscribe_feed};
use feed_fetcher::{
    article_handlers::{article_mark_read, get_article, list_articles, list_feed_articles},
    worker::bg_article_fetcher,
};

async fn health_check() -> Response {
    (StatusCode::OK, "up and running").into_response()
}

pub async fn create_router() -> Router {
    let pool_conn = create_conn_pool().await;

    let worker_conn = pool_conn.clone();
    tokio::spawn(async move { bg_article_fetcher(worker_conn).await });

    Router::new()
        .route("/health", get(health_check))
        .route("/feeds", post(subscribe_feed))
        .route("/feeds", get(list_subscribed_feed))
        .route("/feeds/{id}", post(unsubscribe_feed))
        .route("/feeds/{id}/articles", get(list_feed_articles))
        .route("/articles", get(list_articles))
        .route("/articles/{id}", get(get_article))
        .route("/articles/{id}/read", post(article_mark_read))
        .layer(Extension(pool_conn.clone()))
}
