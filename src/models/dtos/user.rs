use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub discord_id: String,
}
