use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract, Json, Router};
use axum::extract::Path;
use axum::routing::get;
use crate::config::{Collections, DB};
use crate::http::{route, Result, Error};
use crate::repository::collection::get_all;

pub fn list() -> Router {
    async fn handler(pool: extract::Extension<DB>,
                     collections: extract::Extension<Collections>,
                     Path(key): Path<String>) -> Result<Response> {
        if collections.collections.iter().any(|i| i.slug == key) {
            let collection = get_all(&pool.db, key).await?;

            return Ok((StatusCode::OK, Json(collection)).into_response())
        }

        Err(Error::NotFound)
    }

    route("/:key", get(handler))
}
