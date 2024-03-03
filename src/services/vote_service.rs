use sqlx::PgPool;

use crate::errors::Errors;
use crate::models::dtos::question::Question;
use crate::models::question_file::ChoiceOption;
use crate::models::request::vote::Vote;

use super::answer_comments_service::AnswerCommentsService;
use super::answer_service::AnswerService;
use super::question_service::QuestionService;

pub struct VoteService {}

impl VoteService {
    pub async fn insert_vote(db_pool: &PgPool, discord_id: &str, vote: &Vote) -> Result<(), Errors> {
        let question: Option<Question> = QuestionService::get_a_question(db_pool, vote.question_id).await?;

        let Some(question) = question else {
            return Err(Errors::UNAUTHORIZED);
        };

        if let Some(option) = vote.option_id {
            if option as i32 > question.options_available {
                return Err(Errors::UNAUTHORIZED);
            }
        }

        if let (None | Some(false), Some(_)) = (question.allow_comment, vote.comment.clone()) {
            return Err(Errors::UNAUTHORIZED);
        }

        let answer_comments_count =
            AnswerCommentsService::count_by_question_and_user(db_pool, vote.question_id, discord_id).await?;

        if let Some(count) = answer_comments_count {
            println!("{}, {}", count, question.question_type);
            if question.question_type != ChoiceOption::TextMultiple && count >= 1 {
                return Err(Errors::UnavailableAnswer);
            }
        }

        AnswerService::add_answer(db_pool, vote, question.id, discord_id).await?;

        Ok(())
    }
}
