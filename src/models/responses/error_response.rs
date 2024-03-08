use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
