use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelUser {
    pub username: String,
    pub discord_id: String,
}