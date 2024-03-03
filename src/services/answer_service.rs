use sqlx::{query, Error, PgPool};

use crate::models::request::vote::Vote;

use super::answer_comments_service::AnswerCommentsService;

pub struct AnswerService;

impl AnswerService {
    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn add_answer(db_pool: &PgPool, vote: &Vote, question_id: i16, discord_id: &str) -> Result<(), Error> {
        let comment_id = 'bloque: {
            let Some(comment) = vote.comment.clone() else {
                break 'bloque None;
            };
            let answer_id = AnswerCommentsService::add_answer(db_pool, &comment, question_id, discord_id).await?;

            Some(answer_id)
        };

        query!(
            r#"
            INSERT INTO answers (question_id, option_id, answer_comment_id, discord_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (question_id, option_id, discord_id) DO NOTHING;
            "#,
            vote.question_id,
            vote.option_id,
            comment_id,
            discord_id
        )
        .execute(db_pool)
        .await?;

        Ok(())
    }
}
