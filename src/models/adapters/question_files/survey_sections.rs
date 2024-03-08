use std::fmt::Display;
use core::fmt;

use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Deserialize, Serialize, Clone, Type, PartialEq)]
#[sqlx(type_name = "survey_section", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SurveySections {
    Features,
    Use,
    Resources,
    About,
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