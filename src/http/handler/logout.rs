use anyhow::anyhow;
use axum::{extract, Router, TypedHeader};
use axum::routing::post;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::http::{route};
use crate::repository::user;
use crate::config::{DB};
use crate::http::{Result};

pub fn logout() -> Router {
    #[axum_macros::debug_handler]
    async fn logout(pool: extract::Extension<DB>, auth_header: TypedHeader<Authorization<Bearer>>) -> Result<Response> {
        let session_id = hex::decode(auth_header.token())
            .map_err(|e| anyhow!(e))?;

        user::destroy_session(&pool.db, session_id).await?;

        Ok((StatusCode::OK, auth_header).into_response())
    }

    route("/logout", post(logout))
}