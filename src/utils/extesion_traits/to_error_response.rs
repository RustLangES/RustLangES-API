use axum::Json;
use crate::{errors::Errors, models::responses::error_response::ErrorResponse};

pub trait ToErrorResponse {
    fn to_error_response(self) -> Json<ErrorResponse>;
}

impl<T: AsRef<str>> ToErrorResponse for T {
    fn to_error_response(self) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            message: self.as_ref().to_string(),
        })
    }
}

impl ToErrorResponse for Errors {
    fn to_error_response(self) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            message: self.to_string(),
        })
    }
}
