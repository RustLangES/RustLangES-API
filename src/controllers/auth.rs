pub mod auth {
    use std::sync::Arc;
    use std::time::Duration;

    use crate::errors::Errors;
    use crate::models::discord::DiscordCode;
    use crate::services::discord_service::DiscordService;
    use crate::services::user_service::UserService;
    use crate::utils::extractors::validate_query::ValidatedQuery;
    use crate::AppState;
    use anyhow::Result;
    use async_session::{Session, SessionStore};
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::{extract::Query, response::IntoResponse, Json};
    use tracing::{info, instrument};

    #[instrument]
    pub async fn discord(
        State(state): State<Arc<AppState>>,
        ValidatedQuery(code): ValidatedQuery<DiscordCode>,
    ) -> Result<impl IntoResponse, Errors> {
        info!("Discord callback");
        let code = code.code;
        let mut client =
            DiscordService::new(&state.client_id, &state.client_secret, &state.redirect_uri);

        let access = client.get_token(&code).await?;

        let user = client.get_user().await?;

        UserService::insert_or_update(&state.db_pool, &user, &access).await?;

        let mut session = Session::new();
        session.expire_in(Duration::from_secs(access.expires_in as u64));
        session.insert(&access.access_token, &user)?;

        state.store.store_session(session).await?;

        Ok((StatusCode::OK, Json(access + user)))
    }

    pub async fn discord_callback(
        Query(access_token): Query<String>,
    ) -> Result<impl IntoResponse, Errors> {
        let user = DiscordService::get_user_with_access_token(&access_token).await?;

        Ok((StatusCode::OK, Json(user)))
    }

}
