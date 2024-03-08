use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SurveyYear {
    #[validate(range(min = 2024, message = "Invalid year"))]
    pub year: i16,
}
