use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use models::rest::Article;

use crate::{fetcher::feed_fetcher, parser::feed_parser};

#[axum::debug_handler]
pub async fn list_articles() -> Response {
    // let feeds = app_state.subscribed_feeds.read().unwrap().clone();
    // let mut articles: Vec<Article> = vec![];
    // for feed in feeds.iter() {
    //     match feed_fetcher(feed).await {
    //         Ok(ch) => match feed_parser(feed, ch.clone()).await {
    //             Ok(mut channels_articles) => {
    //                 articles.append(&mut channels_articles);
    //             }
    //             Err(err) => {
    //                 eprintln!("error at feed_parser:{:?}", err);
    //                 return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    //             }
    //         },
    //         Err(err) => {
    //             eprintln!("error at feed_fetcher:{:?}", err);
    //             return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
    //         }
    //     };
    // }

    // (StatusCode::OK, Json(articles)).into_response()
    todo!()
}
