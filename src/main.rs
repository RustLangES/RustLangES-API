use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
pub mod errors;
use errors::Errors;
pub mod models;
pub mod services;
use tower_http::cors::{Any, CorsLayer};

use controllers::track::track;

pub mod controllers;

#[derive(Debug)]
pub struct AppState {
    // pub store: MemoryStore,
    pub db_pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(Errors::MigrationError)?;

    let initial_state = Arc::new(AppState {
        db_pool: pool.clone(),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let router = Router::new();

    let track_routes = Router::new()
        .route("/track/count", post(track::count_visit_references))
        .route("/track", get(track::list_visit_references));

    let router = router
        .merge(track_routes)
        .with_state(initial_state)
        .layer(cors);

    Ok(router.into())
}
