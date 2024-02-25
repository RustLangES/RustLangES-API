use axum::Json;
use serde::Serialize;

use crate::errors::Errors;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub trait ToErrorResponse {
    fn to_error_response(self) -> Json<ErrorResponse>;
}

impl ToErrorResponse for String {
    fn to_error_response(self) -> Json<ErrorResponse> {
        Json(ErrorResponse { message: self })
    }
}

impl ToErrorResponse for Errors {
    fn to_error_response(self) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            message: self.to_string(),
        })
    }
}
