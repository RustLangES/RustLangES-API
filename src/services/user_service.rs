use sqlx::PgPool;

use crate::{
    errors::Errors,
    models::discord::{AccessToken, ChannelUser, UserData},
};

pub struct UserService {}

impl UserService {
    pub async fn insert_or_update(
        db_pool: &PgPool,
        user: &UserData,
        access: &AccessToken,
    ) -> Result<(), Errors> {
        sqlx::query!(
            "INSERT INTO users (username, discord_id, access_token, refresh_token, expires_in) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (discord_id) DO UPDATE SET access_token = $3, refresh_token = $4, expires_in = $5, username = $1",
            user.username,
            user.id,
            access.access_token,
            access.refresh_token,
            access.expires_in
        ).execute(db_pool).await?;

        Ok(())
    }

    pub async fn create_user(db_pool: &PgPool, user: ChannelUser) -> Result<(), Errors> {
        sqlx::query!(
            "INSERT INTO users (username, discord_id) VALUES ($1, $2)",
            user.username,
            user.discord_id
        )
        .execute(db_pool)
        .await?;

        Ok(())
    }
}
