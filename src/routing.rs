use axum::{
    Extension, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use database::pool::create_conn_pool;
use feed_fetcher::feed_handlers::{list_subscribed_feed, subscribe_feed, unsubscribe_feed};
use feed_fetcher::{
    article_handlers::list_articles,
    worker::{self, bg_article_fetcher},
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
        .route("/articles", get(list_articles))
        .layer(Extension(pool_conn.clone()))
}
