// background worker for fetching articles

use std::time::Duration;

use models::db::Feed;
use sqlx::{Pool, Postgres};
use tokio::time::sleep;

use crate::{fetcher::feed_fetcher, parser::feed_parser};

pub async fn bg_article_fetcher(conn: Pool<Postgres>) {
    loop {
        println!("Worker running...");
        let result = sqlx::query_as::<_, Feed>("SELECT * FROM feed where active=true;")
            .fetch_all(&conn)
            .await;

        match result {
            Ok(feeds) => {
                for feed in &feeds {
                    match feed_fetcher(feed).await {
                        Ok(ch) => match feed_parser(feed, ch).await {
                            Ok(articles) => {
                                for article in &articles {
                                    let result = sqlx::query(r"INSERT INTO public.article (id, feed_id, url, title, content, read, published) VALUES(gen_random_uuid(), $1, $2, $3, $4, false, $5) ON CONFLICT (url) DO NOTHING;")
                                    .bind(feed.id)
                                    .bind(&article.url)
                                    .bind(&article.title)
                                    .bind(&article.content)
                                    .bind(&article.published)
                                    .execute(&conn)
                                    .await;

                                    match result {
                                        Ok(affected_rows) => {
                                            // if affected_rows.rows_affected() > 0 {
                                            //     println!("Insert successful!");
                                            // } else {
                                            //     println!("Insert unsuccessful!");
                                            // };
                                        }
                                        Err(err) => {
                                            eprintln!("Insert unsuccessful! Error: {}", err)
                                        }
                                    };
                                }
                            }
                            Err(err) => {
                                eprintln!("Error: {}", err);
                                return;
                            }
                        },
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            return;
                        }
                    };
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        };
        println!("Worker sleeping for 5mins...");
        sleep(Duration::from_secs(300)).await;
    }
}
