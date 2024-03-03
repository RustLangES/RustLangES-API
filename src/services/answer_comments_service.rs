use sqlx::{query, Error, PgPool};

pub struct AnswerCommentsService;

impl AnswerCommentsService {
    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn count_by_question_and_user(
        db_pool: &PgPool,
        question_id: i16,
        discord_id: &str,
    ) -> Result<Option<i64>, Error> {
        let count_result = query!(
            "SELECT COUNT(*) as count FROM answer_comments WHERE question_id = $1 AND discord_id = $2",
            question_id,
            discord_id
        )
        .fetch_one(&db_pool.clone())
        .await?;

        Ok(count_result.count)
    }

    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn add_answer(db_pool: &PgPool, comment: &str, question_id: i16, discord_id: &str) -> Result<i16, Error> {
        let count_result = query!(
            r#"
            INSERT INTO answer_comments (comment, question_id, discord_id)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            comment,
            question_id,
            discord_id
        )
        .fetch_one(&db_pool.clone())
        .await?;

        Ok(count_result.id)
    }
}
