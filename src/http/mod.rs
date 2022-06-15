use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Extension, Router};
use axum::http::{HeaderValue, Method};
use axum::http::header::{CONTENT_TYPE, AUTHORIZATION};
use axum::routing::MethodRouter;
use hyper::{Server};
use log::{error, info};
use sqlx::SqlitePool;
use tower_http::cors::{CorsLayer};
use crate::config::{Collections, Config, DB, GlobalConfig};

mod handler;
mod router;
mod error;

pub use error::{Error, ResultExt};
use crate::repository::collection::get_collections;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

pub async fn serve(config: Config, db: SqlitePool) {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    let collections = get_collections(&db).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let app = Router::new()
        .merge(handler::home())
        .merge(router::admin())
        .merge(router::api())
        .layer(Extension(DB { db }))
        .layer(Extension(GlobalConfig { config: Arc::new(config) }))
        .layer(Extension(Collections { collections }))
        .layer(cors);

    let server = Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    info!("Listening on {}", server.local_addr());

    if let Err (e) = server.await {
        error!("Server error: {}", e);
    }
}
