use axum::{http::StatusCode, response::{IntoResponse, Response}};
use shuttle_runtime::CustomError;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Secret not found")]
    SecretNotFound,

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

    #[error("Invalid session")]
    InvalidSession,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        error!("Error: {:#?}", self);

        match self {
            Errors::Reqwest(e) => e.to_response(),
            Self::MigrationError(e) => (
                StatusCode::CONFLICT,
                format!("Error migrating database: {e:#?}"),
            )
                .into_response(),
            Self::DatabaseError(e) => {
                error!("Database Error: {:#?}", e);
                (
                    StatusCode::CONFLICT,
                    format!("Error executing a database query"),
                )
                    .into_response()
            }
            e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something happens {:#?}", e),
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
            (
                StatusCode::REQUEST_TIMEOUT,
                format!("Request Timeout: {}", self),
            )
                .into_response()
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
