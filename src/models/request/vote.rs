use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub question_id: i16,
    pub option_id: i16,
}
