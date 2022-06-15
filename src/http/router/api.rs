use axum::{Router};
use crate::http::handler;

pub fn api() -> Router {
    Router::new()
        .nest("/api", Router::new()
            .merge(handler::collection::list())
            .merge(handler::collection::item())
        )
}
