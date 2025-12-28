// Parse articles into

use std::{cell::Cell, error::Error};

use chrono::{NaiveDateTime, Utc};
use models::{db::Feed, rest::Article};
use rss::Channel;

pub async fn feed_parser(
    feed: &Feed,
    channel: Channel,
) -> Result<Vec<Article>, Box<dyn Error + Sync + Send>> {
    let mut articles: Vec<Article> = vec![];

    let default_pub_date = Utc::now();
    let default_pub_date_str = Cell::new(default_pub_date.format("%Y-%m-%dT%H:%M:%SZ").to_string());

    for article in channel.items() {
        // println!("{:?}\n\n", article); // TODO remove
        let pub_date = match NaiveDateTime::parse_from_str(
            &article
                .pub_date
                .clone()
                .unwrap_or(default_pub_date_str.take()),
            "%Y-%m-%dT%H:%M:%SZ",
        ) {
            Ok(date) => date.and_utc(),
            Err(_err) => default_pub_date,
        };
        articles.push(Article {
            feed_id: feed.id,
            title: article.title.clone().unwrap_or("".to_string()),
            url: article.link.clone().unwrap_or("".to_string()),
            published: pub_date,
            content: article.content.clone().unwrap_or("".to_string()),
        });
    }

    Ok(articles)
}
