pub mod answer {
    use std::sync::Arc;

    use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

    use crate::{
        errors::Errors, middleware::authenticator::AuthMiddleware,
        services::answer_service::AnswerService, AppState,
    };

    pub async fn get_answers(
        Extension(auth_guard): Extension<AuthMiddleware>,
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, Errors> {
        let discord_id = auth_guard.0.id.clone();

        let answers = AnswerService::get_answers_by_discord_id(&state.db_pool, &discord_id).await?;

        Ok((StatusCode::OK, Json(answers)))
    }
}
