-- Add up migration script here
DROP TYPE IF EXISTS question_type;
CREATE TYPE choice_option AS ENUM ('single', 'multiple', 'limited-2', 'limited-3', 'limited-4', 'numeric', 'text-multiple');
CREATE TYPE survey_section AS ENUM ('features', 'use', 'resources', 'about');

CREATE TABLE IF NOT EXISTS surveys(
    id SMALLSERIAL PRIMARY KEY,
    year SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS questions (
    id SMALLSERIAL PRIMARY KEY,
    question_type choice_option NOT NULL,
    section_type survey_section NOT NULL,
    allow_comment BOOLEAN,
    options_available INT NOT NULL,
    survey_id SMALLINT NOT NULL,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (survey_id) REFERENCES surveys (id)
);

ALTER TABLE users DROP COLUMN discord_id;
ALTER TABLE users ADD COLUMN discord_id VARCHAR(255) UNIQUE;

CREATE TABLE IF NOT EXISTS answer_comments (
    id SMALLSERIAL PRIMARY KEY,
    comment VARCHAR(255),
    question_id SMALLINT NOT NULL,
    discord_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (question_id) REFERENCES questions (id),
    FOREIGN KEY (discord_id) REFERENCES users (discord_id)
);

CREATE TABLE IF NOT EXISTS answers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id SMALLINT NOT NULL,
    option_id SMALLINT,
    answer_comment_id SMALLINT,
    discord_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (question_id) REFERENCES questions (id),
    FOREIGN KEY (answer_comment_id) REFERENCES answer_comments (id),
    FOREIGN KEY (discord_id) REFERENCES users (discord_id),
    CONSTRAINT unique_answer_combination UNIQUE (question_id, option_id, discord_id),
    CONSTRAINT unique_comment_combination UNIQUE (question_id, answer_comment_id, discord_id)
);

-- INSERT INTO questions (question_type, section_type, allow_comment, options_available, survey_id)
-- VALUES (
--     'multiple'::public."choice_option",
--     'features'::public."survey_section",
--     false,
--     4,
--     1
-- );
