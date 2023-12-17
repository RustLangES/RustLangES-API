use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData {
    pub accent_color: Option<u32>,
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<String>,
    pub banner: Option<String>,
    pub banner_color: Option<u32>,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub global_name: String,
    pub id: String,
    pub locale: String,
    pub mfa_enabled: Option<bool>,
    pub premium_type: Option<u32>,
    pub public_flags: Option<u32>,
    pub username: String,
    pub verified: Option<bool>,
}
