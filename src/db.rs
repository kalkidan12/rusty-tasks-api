use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use crate::config::Config;

pub async fn init() -> PgPool {
    let config = Config::from_env();

    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database")
}
