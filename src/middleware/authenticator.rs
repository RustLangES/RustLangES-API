use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

use serde::{Deserialize, Serialize};

use crate::{errors::Errors, models::discord::UserData, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthMiddleware(UserData);

pub async fn auth(
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Errors> {
    let access_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let access_token = access_token.ok_or(Errors::UNAUTHORIZED)?;

    let user: UserData = data.redis_client.get(access_token).await?;

    req.extensions_mut().insert(AuthMiddleware(user));
    Ok(next.run(req).await)
}
