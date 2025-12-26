// fetch articles

use models::models::Feed;
use rss::Channel;
use std::error::Error;

pub async fn feed_fetcher(feed: &Feed) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    let r = reqwest::get(feed.url.as_str()).await?.bytes().await?;

    let channel = Channel::read_from(&r[..])?;

    Ok(channel)
}
