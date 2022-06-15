mod model;
mod repository;

use voltaic::config::Config;
use sqlx::sqlite::{SqlitePool};
use voltaic::http;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let config = Config::new();

    let pool = SqlitePool::connect(&config.database_url)
        .await
        .expect("There was a problem connecting to the database");

    http::serve(config, pool).await;
}
