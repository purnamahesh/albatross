use albatross::app;
use dotenvy::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _env_map = dotenv()?;
    app().await?;
    Ok(())
}
