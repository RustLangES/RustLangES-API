use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct DiscordCode {
    #[validate(length(equal=30, message="Invalid code length"))]
    pub code: String,
}
