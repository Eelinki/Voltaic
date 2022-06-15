use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract, Json, Router};
use axum::extract::Path;
use axum::routing::get;
use crate::config::{Collections, DB};
use crate::http::{route, Result, Error};
use crate::repository::collection::{get_item};

pub fn item() -> Router {
    async fn handler(pool: extract::Extension<DB>,
                     collections: extract::Extension<Collections>,
                     Path((key, id)): Path<(String, i64)>) -> Result<Response> {
        if collections.collections.iter().any(|i| i.slug == key) {
            let collection = get_item(&pool.db, key, id).await?;

            return Ok((StatusCode::OK, Json(collection)).into_response())
        }

        Err(Error::NotFound)
    }

    route("/:key/:id", get(handler))
}
