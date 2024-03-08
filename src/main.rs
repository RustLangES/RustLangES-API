use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use controllers::{answer::answer, health_check::health_checker_handler, survey::survey::load_survey, vote::vote::vote};
use middleware::authenticator::authenticator;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;

pub mod errors;
use errors::Errors;
pub mod models;
pub mod services;
use tower_http::cors::{Any, CorsLayer};

use crate::utils::newtypes::async_client::AsyncClient;
use controllers::{auth::auth, track::track};

pub mod controllers;
pub mod middleware;
pub mod utils;

#[derive(Debug)]
pub struct AppState {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub db_pool: PgPool,
    pub admin_ids: Vec<String>,
    pub redis_client: AsyncClient,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:16695/RustLangEs")]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(Errors::MigrationError)?;

    let admin_ids = secret_store
        .get("ADMIN_IDS")
        .ok_or(Errors::SecretNotFound)?
        .split(",")
        .map(|id| id.to_string())
        .collect();

    let initial_state = Arc::new(AppState {
        client_id: secret_store.get("CLIENT_ID").ok_or(Errors::SecretNotFound)?,
        client_secret: secret_store.get("CLIENT_SECRET").ok_or(Errors::SecretNotFound)?,
        redirect_uri: secret_store.get("REDIRECT_URI").ok_or(Errors::SecretNotFound)?,
        redis_client: AsyncClient::open(secret_store.get("REDIS_URI").ok_or(Errors::SecretNotFound)?)?,
        admin_ids,
        db_pool: pool.clone(),
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let router = Router::new().route("/healthchecker", get(health_checker_handler));

    let auth_routes = Router::new().route("/discord", get(auth::discord));
    let answer_routes = Router::new().route("/answer", get(answer::get_answers));

    let track_routes = Router::new()
        .route("/track/count", post(track::count_visit_references))
        .route("/track", get(track::list_visit_references));

    let vote_routes = Router::new().route("/vote", post(vote));

    let survey_routes = Router::new().route("/survey", post(load_survey));

    let router = router
        .merge(track_routes)
        .merge(vote_routes)
        .merge(survey_routes)
        .merge(answer_routes)
        .route_layer(axum::middleware::from_fn_with_state(
            initial_state.clone(),
            authenticator,
        ))
        .merge(auth_routes)
        .with_state(initial_state)
        .layer(cors);

    Ok(router.into())
}
