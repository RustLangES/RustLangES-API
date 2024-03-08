use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lang {
    pub title: String,
    pub note: Option<String>,
    pub options: Option<Vec<String>>,
}
