use async_session::MemoryStore;
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;
pub mod errors;
use errors::Errors;
pub mod models;
pub mod services;
use tower_http::cors::{Any, CorsLayer};

use controllers::{track::track, auth::auth};

pub mod controllers;
pub mod utils;

#[derive(Debug)]
pub struct AppState {
    pub store: MemoryStore,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub db_pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:16695/RustLangEs"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(Errors::MigrationError)?;

    let initial_state = Arc::new(AppState {
        store: MemoryStore::new(),
        client_id: secret_store.get("CLIENT_ID").ok_or(Errors::SecretNotFound)?,
        client_secret: secret_store.get("CLIENT_SECRET").ok_or(Errors::SecretNotFound)?,
        redirect_uri: secret_store.get("REDIRECT_URI").ok_or(Errors::SecretNotFound)?,
        db_pool: pool.clone(),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let router = Router::new();
    
    let auth_routes = Router::new()
        .route("/discord", get(auth::discord));


    let track_routes = Router::new()
        .route("/track/count", post(track::count_visit_references))
        .route("/track", get(track::list_visit_references));

    let router = router
        .merge(track_routes)
        .merge(auth_routes)
        .with_state(initial_state)
        .layer(cors);

    Ok(router.into())
}
