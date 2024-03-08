use std::fmt::Debug;
use sqlx::types::time::PrimitiveDateTime;

use crate::models::adapters::question_files::{choice_option::ChoiceOption, survey_sections::SurveySections};

#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub id: i16,
    pub question_type: ChoiceOption,
    pub section_type: SurveySections,
    pub allow_comment: Option<bool>,
    pub options_available: i32,
    pub survey_id: i16,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub created_at: PrimitiveDateTime,
}
