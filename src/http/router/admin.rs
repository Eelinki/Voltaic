use axum::{middleware, Router};
use crate::http::handler;
use crate::http::handler::require_auth;

pub fn admin() -> Router {
    Router::new().nest("/admin", Router::new()
        .merge(handler::login())
        .merge(Router::new()
            .merge(handler::logout())
            .route_layer(middleware::from_fn(require_auth))
        ))
}
