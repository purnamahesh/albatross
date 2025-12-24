use std::{ascii::AsciiExt, sync::Arc};
use uuid::Uuid;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use models::models::{AppState, Feed};

pub async fn subscribe_feed(
    State(app_state): State<Arc<AppState>>,
    Json(mut body): Json<Feed>,
) -> Response {
    let mut feeds = app_state.subscribed_feeds.write().unwrap();

    body.id = Some(Uuid::new_v4());

    feeds.push(body);

    (StatusCode::CREATED).into_response()
}

pub async fn list_subscribed_feed(State(app_state): State<Arc<AppState>>) -> Response {
    (
        StatusCode::ACCEPTED,
        Json(app_state.subscribed_feeds.read().unwrap().clone()),
    )
        .into_response()
}

pub async fn unsubscribe_feed(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Response {
    let mut feeds = app_state.subscribed_feeds.write().unwrap();

    match feeds.iter().position(|feed| feed.id.unwrap().eq(&id)) {
        Some(pos) => (StatusCode::ACCEPTED, Json(feeds.remove(pos))).into_response(),
        None => (StatusCode::NOT_FOUND, format!("id: {} is not found", id)).into_response(),
    }
}
