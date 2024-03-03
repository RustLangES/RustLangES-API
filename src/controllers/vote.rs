pub mod vote {
    use std::sync::Arc;

    use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
    use tracing::error;

    use crate::{
        errors::Errors, middleware::authenticator::AuthMiddleware, models::request::vote::Vote,
        services::vote_service::VoteService, AppState,
    };

    pub async fn vote(
        Extension(auth_guard): Extension<AuthMiddleware>,
        State(state): State<Arc<AppState>>,
        Json(votes): Json<Vec<Vote>>,
    ) -> Result<impl IntoResponse, Errors> {
        let discord_id = auth_guard.0.id.clone();

        for vote in votes.iter() {
            if let Err(error) = VoteService::insert_vote(&state.db_pool, &discord_id, vote).await {
                error!("Ha ocurrido un error {:?} {:?}", vote, auth_guard.0);
                error!("Error: {error:?}");
                return Err(error);
            }
        }

        Ok((StatusCode::NO_CONTENT, ()).into_response())
    }
}
