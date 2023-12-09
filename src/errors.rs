use axum::{http::StatusCode, response::IntoResponse};
use shuttle_runtime::CustomError;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        error!("Error: {:#?}", self);

        match self {
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
            // e => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     format!("Something happens {:#?}", e),
            // )
            //     .into_response(),
        }
    }
}

impl From<Errors> for shuttle_runtime::Error {
    fn from(error: Errors) -> Self {
        error!("Error migrating database: {:#?}", error);

        match error {
            Errors::MigrationError(e) => Self::Custom(CustomError::new(e)),
            Errors::DatabaseError(e) => Self::Custom(CustomError::new(e)),
        }
    }
}
