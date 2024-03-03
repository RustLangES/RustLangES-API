use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use redis::RedisError;
use shuttle_runtime::CustomError;
use thiserror::Error;
use tracing::error;

use crate::models::responses::error_response::ToErrorResponse;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Secret not found")]
    SecretNotFound,

    #[error("Error saving a session")]
    CookieNotSaved,

    #[error("You are not logged in, please provide token")]
    UNAUTHORIZED,

    #[error(transparent)]
    InvalidHeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    AsyncSessions(#[from] async_session::Error),

    #[error(transparent)]
    SerdeJSON(#[from] serde_json::Error),

    #[error("Failed to deserialize query string")]
    QueryRejection,

    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("You can't add other comment to this question")]
    UnavailableAnswer,

    // #[error(transparent)]
    // Anyhow(#[from] anyhow::Error),
    #[error("Invalid session")]
    InvalidSession,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        error!("Error: {:#?}", self);

        match self {
            Errors::UnavailableAnswer => (
                StatusCode::BAD_REQUEST,
                Errors::UnavailableAnswer.to_string().to_error_response(),
            )
                .into_response(),
            Errors::IOError(_) => (StatusCode::NOT_FOUND, "Error getting a path".to_error_response()).into_response(),
            Errors::UNAUTHORIZED => {
                (StatusCode::UNAUTHORIZED, Errors::UNAUTHORIZED.to_error_response()).into_response()
            }
            Errors::Reqwest(e) => e.to_response(),
            Self::MigrationError(e) => (
                StatusCode::CONFLICT,
                format!("Error migrating database: {e:#?}").to_error_response(),
            )
                .into_response(),
            Self::DatabaseError(e) => {
                error!("Database Error: {:#?}", e);
                (
                    StatusCode::CONFLICT,
                    format!("Error executing a database query").to_error_response(),
                )
                    .into_response()
            }
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something happens {:#?}", e).to_error_response(),
            )
                .into_response(),
        }
    }
}

impl From<Errors> for shuttle_runtime::Error {
    fn from(error: Errors) -> Self {
        error!("Error migrating database: {:#?}", error);

        match error {
            Errors::MigrationError(e) => Self::Custom(CustomError::new(e)),
            Errors::DatabaseError(e) => Self::Custom(CustomError::new(e)),
            e => Self::Custom(CustomError::new(e)),
        }
    }
}

trait ToResponse {
    fn to_response(&self) -> Response;
}

impl ToResponse for reqwest::Error {
    fn to_response(&self) -> Response {
        if self.is_decode() {
            (StatusCode::BAD_REQUEST, format!("Bad Request: {}", self)).into_response()
        } else if self.is_timeout() {
            (StatusCode::REQUEST_TIMEOUT, format!("Request Timeout: {}", self)).into_response()
        } else if self.is_status() {
            (StatusCode::BAD_REQUEST, format!("Bad Request: {}", self)).into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", self),
            )
                .into_response()
        }
    }
}
