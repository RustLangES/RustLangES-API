pub mod survey {
    use std::{
        fs::{self},
        sync::Arc,
    };

    use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension};

    use crate::{
        errors::Errors, middleware::authenticator::AuthMiddleware, models::request::survey_year::SurveyYear,
        services::survey_service::SurveyService, utils::extractors::validate_query::ValidatedQuery, AppState,
    };

    pub async fn load_survey(
        Extension(auth_guard): Extension<AuthMiddleware>,
        State(state): State<Arc<AppState>>,
        ValidatedQuery(SurveyYear { year }): ValidatedQuery<SurveyYear>,
    ) -> Result<impl IntoResponse, Errors> {
        let discord_id = auth_guard.0.id.clone();
        if !state.admin_ids.contains(&discord_id) {
            return Err(Errors::UNAUTHORIZED);
        };

        let dbpool = &state.db_pool;

        let survey_directory = format!("survey_questions/{}", year);
        let question_dir = fs::read_dir(&survey_directory)?;

        SurveyService::insert_a_survey(dbpool, year).await?;

        for file_entry in question_dir {
            SurveyService::insert_a_question(dbpool, file_entry).await?;
        }

        Ok((StatusCode::NO_CONTENT, ()).into_response())
    }
}
