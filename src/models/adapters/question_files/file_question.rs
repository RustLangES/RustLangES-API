use serde::{Deserialize, Serialize};

use super::{choice_option::ChoiceOption, survey_sections::SurveySections, translations::Translations};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileQuestion {
    pub allow_comment: bool,
    #[serde(rename = "type")]
    pub type_field: ChoiceOption,
    pub section: SurveySections,
    pub translations: Translations,
}
