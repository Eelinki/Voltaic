use std::env;
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::repository::collection::Collection;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").ok()
            .unwrap_or_else(|| String::from("sqlite://data.db"));

        let port = env::var("PORT").ok()
            .and_then(|val| val.parse().ok())
            .unwrap_or(6009);

        Config {
            database_url,
            port
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

#[derive(Clone)]
pub struct GlobalConfig {
    pub config: Arc<Config>,
}

#[derive(Clone)]
pub struct DB {
    pub db: SqlitePool,
}

#[derive(Clone)]
pub struct Collections {
    pub collections: Vec<Collection>,
}