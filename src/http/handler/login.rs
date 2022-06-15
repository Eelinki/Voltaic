use anyhow::anyhow;
use axum::{extract, Router};
use axum::routing::post;
use axum::extract::Json;
use bcrypt::verify;
use rand::{Rng};
use serde::{Serialize, Deserialize};
use crate::http::{Error, route};
use crate::repository::user;
use crate::config::{DB};
use crate::http::{Result};
use crate::http::Error::Unauthorized;

#[derive(Deserialize, Debug)]
struct Login {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct SessionResponse {
    session_id: String
}

pub fn login() -> Router {
    #[axum_macros::debug_handler]
    async fn handler(payload: Json<Login>, pool: extract::Extension<DB>) -> Result<Json<SessionResponse>> {
        let user = match user::by_email(&pool.db, payload.email.clone()).await {
            Ok(user) => user,
            Err(Error::NotFound) => return Err(Unauthorized("Invalid username or password".to_string())),
            Err(e) => return Err(Error::Anyhow(anyhow!(e)))
        };

        let verified = verify(payload.password.clone(), &user.password)
            .map_err(|e| anyhow::anyhow!("Failed to verify password hash: {}", e))?;

        if !verified {
            return Err(Unauthorized("Invalid username or password".to_string()))
        }

        //create session
        let session_id = rand::thread_rng().gen::<[u8; 32]>();
        let session = user::create_session(&pool.db, &user, Vec::from(session_id)).await?;
        let session_response = SessionResponse {
            session_id: hex::encode(session.id)
        };

        Ok(Json(session_response))
    }

    route("/login", post(handler))
}