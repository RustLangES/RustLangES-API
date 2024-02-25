use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};

pub trait ToResponse {
    fn to_response(self) -> Response;
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
