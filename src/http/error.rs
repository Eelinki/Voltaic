use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use log::error;
use serde_json::json;
use sqlx::error::DatabaseError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),

    #[error("Internal server error")]
    Anyhow(#[from] anyhow::Error)
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Anyhow(ref e) => {
                error!("Generic error: {:?}", e)
            }

            Self::Sqlx(ref e) => {
                error!("SQLx error: {:?}", e);
            }

            _ => ()
        }

        let json = json!({
            "error": {
                "message": self.to_string()
            }
        });

        (self.status_code(), Json(json)).into_response()
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
    where
        E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}