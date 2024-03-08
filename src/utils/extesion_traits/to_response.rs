use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{errors::Errors, models::responses::error_response::ToErrorResponse};

pub trait ToResponse {
    fn to_response(self) -> Response;
}

impl<T: AsRef<str>> ToResponse for (StatusCode, T) {
    fn to_response(self) -> Response {
        let (status, message) = self;
        (status, message.to_error_response()).into_response()
    }
}

impl ToResponse for (StatusCode, Errors) {
    fn to_response(self) -> Response {
        let (status, error) = self;
        (status, error.to_error_response()).into_response()
    }
}

impl ToResponse for reqwest::Error {
    fn to_response(self) -> Response {
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

impl<T> ToResponse for (HeaderMap, StatusCode, T)
where
    T: IntoResponse,
{
    fn to_response(self) -> Response {
        let mut res = self.2.into_response();
        *res.status_mut() = self.1;
        *res.headers_mut() = self.0;
        res
    }
}
