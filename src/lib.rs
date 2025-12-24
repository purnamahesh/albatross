// use axum::Error;
use std::error::Error;

use crate::routing::create_router;

mod routing;

pub async fn app() -> Result<(), Box<dyn Error>> {
    let router = create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8055").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
