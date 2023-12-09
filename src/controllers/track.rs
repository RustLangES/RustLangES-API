pub mod track {
    use crate::{
        errors::Errors, models::request::reference_query::ReferenceQuery,
        services::track_service::TrackService, AppState,
    };
    use axum::{
        extract::{Query, State},
        http::StatusCode,
        response::IntoResponse,
        Json,
    };
    use std::{collections::HashMap, sync::Arc};

    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn count_visit_references(
        State(state): State<Arc<AppState>>,
        Query(query): Query<ReferenceQuery>,
    ) -> Result<impl IntoResponse, Errors> {
        let reference = query.reference.to_lowercase();

        TrackService::count_a_visit(&state.db_pool.clone(), reference).await?;

        Ok((StatusCode::NO_CONTENT, ()))
    }

    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn list_visit_references(
        State(state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, Errors> {
        let result = TrackService::get_visits_by_domain(&state.db_pool.clone()).await?;

        let mut references = HashMap::with_capacity(result.len());
        for row in result {
            references.insert(row.domain, row.visits);
        }

        Ok((StatusCode::OK, Json(references)))
    }
}
