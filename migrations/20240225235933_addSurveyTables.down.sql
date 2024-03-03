-- Add down migration script here
DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS answer_comments;
DROP TABLE IF EXISTS possible_options;
DROP TABLE IF EXISTS questions;
DROP TYPE IF EXISTS choice_option;
DROP TYPE IF EXISTS survey_section;
DROP TABLE IF EXISTS surveys;


