use std::sync::Arc;

use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use feed_fetcher::feed_handlers::{list_subscribed_feed, subscribe_feed, unsubscribe_feed};
use models::models::{AppState, Feed};
use std::sync::RwLock;

async fn health_check() -> Response {
    (StatusCode::OK, "up and running").into_response()
}

pub fn create_router() -> Router {
    // let x = Vec::<Feed>::new();
    let state = Arc::new(AppState {
        subscribed_feeds: RwLock::new(Vec::<Feed>::new()),
    });

    Router::new()
        .route("/health", get(health_check))
        .route("/feeds", post(subscribe_feed))
        .route("/feeds", get(list_subscribed_feed))
        .route("/feeds/{id}", post(unsubscribe_feed))
        .with_state(state.clone())
}
