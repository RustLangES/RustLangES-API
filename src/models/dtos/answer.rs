use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};
use super::answer_comment::AnswerComment;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: i16,
    pub option_id: Option<i16>,
    pub answer_comment_id: Option<i16>,
    pub answer_comment: Option<AnswerComment>,
    pub discord_id: String,
    pub created_at: NaiveDateTime,
}

impl FromRow<'_, PgRow> for Answer {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let id = row.try_get("id")?;

        let answer_comment_id: Option<i16> = row.try_get("answer_comment_id").unwrap_or_default();
        let discord_id: String = row.get("discord_id");
        let question_id: i16 = row.get("question_id");
        let answer_comment = match answer_comment_id.clone() {
            Some(id) => Some(AnswerComment {
                id,
                question_id,
                comment: row.get("comment"),
                discord_id: discord_id.clone()
            }),
            None => None
        };

        Ok(
            Answer {
                id,
                question_id,
                discord_id,
                answer_comment_id,
                answer_comment,
                option_id: row.get("option_id"),
                created_at: row.get("created_at")
            }
        )
    }
}