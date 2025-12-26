// Parse articles into

use std::error::Error;

use chrono::NaiveDateTime;
use models::models::{Article, Feed};
use rss::Channel;
use uuid::Uuid;

pub async fn feed_parser(
    feed: &Feed,
    channel: Channel,
) -> Result<Vec<Article>, Box<dyn Error + Sync + Send>> {
    let mut articles: Vec<Article> = vec![];

    for article in channel.items() {
        // println!("{:?}", article); // TODO remove
        let pub_date = NaiveDateTime::parse_from_str(
            &article
                .pub_date
                .clone()
                .unwrap_or("Fri, 26 Dec 2025 04:17:47 GMT".to_string()),
            "%a, %d %b %Y %H:%M:%S %Z",
        )?.and_utc();
        articles.push(Article {
            id: Uuid::new_v4(),
            feed_id: feed.id,
            title: article.title.clone().unwrap_or("".to_string()),
            link: article.link.clone().unwrap_or("".to_string()),
            published: pub_date,
            content: article.content.clone().unwrap_or("".to_string()),
        });
    }

    Ok(articles)
}
