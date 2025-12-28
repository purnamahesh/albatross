use std::process::exit;

use sqlx::{PgPool, Pool, Postgres};

pub async fn create_conn_pool() -> Pool<Postgres> {
    let conn_url = match std::env::var("DATABASE_URL") {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    let conn_pool = match PgPool::connect(&conn_url).await {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    conn_pool
}
