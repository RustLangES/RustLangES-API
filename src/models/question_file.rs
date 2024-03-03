use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "choice_option", rename_all = "lowercase")]
#[serde(rename_all = "kebab-case")]
pub enum ChoiceOption {
    Single,
    Multiple,
    #[serde(rename = "limited-2")]
    Limited2,
    #[serde(rename = "limited-3")]
    Limited3,
    #[serde(rename = "limited-4")]
    Limited4,
    Numeric,
    TextMultiple,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "survey_section", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SurveySections {
    Features,
    Use,
    Resources,
    About,
}

impl Display for ChoiceOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Multiple => write!(f, "multiple"),
            Self::Limited2 => write!(f, "limited-2"),
            Self::Limited3 => write!(f, "limited-3"),
            Self::Limited4 => write!(f, "limited-4"),
            Self::Numeric => write!(f, "numeric"),
            Self::TextMultiple => write!(f, "text-multiple"),
        }
    }
}

impl Display for SurveySections {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::About => write!(f, "about"),
            Self::Features => write!(f, "features"),
            Self::Resources => write!(f, "resources"),
            Self::Use => write!(f, "use"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileQuestion {
    pub allow_comment: bool,
    #[serde(rename = "type")]
    pub type_field: ChoiceOption,
    pub section: SurveySections,
    pub translations: Translations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translations {
    pub es: Lang,
    pub en: Lang,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lang {
    pub title: String,
    pub note: Option<String>,
    pub options: Option<Vec<String>>,
}
