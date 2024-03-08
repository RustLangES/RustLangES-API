use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize, Clone, Type, PartialEq)]
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
