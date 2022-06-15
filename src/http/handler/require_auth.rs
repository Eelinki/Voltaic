use anyhow::{anyhow, Context};
use axum::http::{header, Request};
use axum::middleware::Next;
use axum::response::{Response};
use crate::config::DB;
use crate::http::{Result};
use crate::http::Error::{BadRequest, Unauthorized};
use crate::repository::user::verify_session;

pub async fn require_auth<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let pool: &DB = req.extensions().get().context("no db extension").map_err(|e| anyhow!(e))?;

    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let token = auth_header.ok_or_else(|| BadRequest("Invalid session".to_string()))?;
    let token = token.strip_prefix("Bearer ")
        .ok_or_else(|| BadRequest("Invalid token".to_string()))?;

    let session_id = hex::decode(token).map_err(|_| BadRequest("Invalid token".to_string()))?;

    let session_is_valid = verify_session(&pool.db, session_id).await?;

    if !session_is_valid {
        return Err(Unauthorized("Invalid session".to_string()))
    };

    Ok(next.run(req).await)
}