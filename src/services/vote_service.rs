use sqlx::{query, PgPool};

use crate::models::request::vote::Vote;


pub struct VoteService {}

impl VoteService {
    pub async fn insert_vote(
        db_pool: &PgPool,
        discord_id: &str,
        vote: &Vote,
    ) -> Result<(), sqlx::Error> {
        query!(
            r#"
            INSERT INTO answers (question_id, option_id, discord_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (question_id, option_id, discord_id) DO NOTHING
            "#,
            vote.question_id,
            vote.option_id,
            discord_id
        )
        .execute(db_pool)
        .await?;

        Ok(())
    }
}