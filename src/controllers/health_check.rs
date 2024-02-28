use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust y Axum Framework";

    let json_response = serde_json::json!({
        "message": MESSAGE
    });

    (StatusCode::OK, Json(json_response))
}
