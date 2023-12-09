use crate::models::discord::{AccessToken, UserData};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SessionData {
    pub access: AccessToken,
    pub user: UserData,
}

impl Add<UserData> for AccessToken {
    type Output = SessionData;

    fn add(self, other: UserData) -> Self::Output {
        SessionData {
            access: self,
            user: other,
        }
    }
}
