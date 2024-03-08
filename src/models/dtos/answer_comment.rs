use serde::Serialize;
use sqlx_macros::Type;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Type, Serialize)]
pub struct AnswerComment {
    pub id: i16,
    pub comment: String,
    pub question_id: i16,
    pub discord_id: String,
}
