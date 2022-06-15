use axum::http::StatusCode;
use axum::response::{IntoResponse};
use axum::Router;
use axum::routing::get;
use crate::http::route;

pub fn home() -> Router {
    async fn handler() -> impl IntoResponse {
        (StatusCode::OK, "Voltaic")
    }

    route("/", get(handler))
}
