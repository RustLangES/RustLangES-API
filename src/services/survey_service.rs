use std::{
    fs::{self, DirEntry},
    io::Result as IOResult,
};

use sqlx::{query, PgPool};

use crate::{errors::Errors, models::adapters::question_files::file_question::FileQuestion};

pub struct SurveyService;

impl SurveyService {
    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn insert_a_question(dbpool: &PgPool, file_entry: IOResult<DirEntry>) -> Result<(), Errors> {
        let file_entry = file_entry?;

        let question_content = fs::read_to_string(file_entry.path())?;
        let question: FileQuestion = serde_json::from_str(&question_content)?;

        query!(
            r#"
                INSERT INTO questions (question_type, section_type, allow_comment, options_available, survey_id)
                VALUES (($1::text)::choice_option, ($2::text)::survey_section, $3, $4, $5);
                "#,
            &question.type_field.to_string(),
            &question.section.to_string(),
            question.allow_comment,
            question.translations.es.options.unwrap_or(vec![]).len() as i32,
            1
        )
        .execute(dbpool)
        .await?;

        Ok(())
    }

    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn insert_a_survey(dbpool: &PgPool, year: i16) -> Result<(), Errors> {
        query!("INSERT INTO surveys (year) VALUES ($1);", year)
            .execute(dbpool)
            .await?;

        Ok(())
    }
}
