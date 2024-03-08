use serde::{Deserialize, Serialize};

use super::lang::Lang;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translations {
    pub es: Lang,
    pub en: Lang,
}
