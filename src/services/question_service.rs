use sqlx::{query_as, Error, PgPool};

use crate::models::dtos::question::Question;

pub struct QuestionService;

impl QuestionService {
    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn get_a_question(db_pool: &PgPool, question_id: i16) -> Result<Option<Question>, Error> {
        query_as!(Question, r#"
            SELECT id, question_type as "question_type: _", section_type as "section_type: _", allow_comment, options_available, survey_id, deleted_at, created_at FROM questions WHERE id = $1
            "#, question_id).fetch_optional(&db_pool.clone()).await
    }
}
