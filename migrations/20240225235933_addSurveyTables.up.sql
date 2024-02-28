-- Add up migration script here
DROP TYPE IF EXISTS question_type;
CREATE TYPE choice_option AS ENUM ('single', 'multiple', 'limited-2', 'limited-3', 'limited-4');

CREATE TABLE IF NOT EXISTS surveys(
    id SMALLSERIAL PRIMARY KEY,
    year SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS questions (
    id SMALLSERIAL PRIMARY KEY,
    label_text VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    question_type choice_option NOT NULL,
    survey_id SMALLINT NOT NULL,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (survey_id) REFERENCES surveys (id)
);


CREATE TABLE IF NOT EXISTS possible_options (
    id SMALLSERIAL PRIMARY KEY,
    label_text VARCHAR(255) NOT NULL,
    question_id SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (question_id) REFERENCES questions (id)
);

ALTER TABLE users DROP COLUMN discord_id;
ALTER TABLE users ADD COLUMN discord_id VARCHAR(255) UNIQUE;

CREATE TABLE IF NOT EXISTS answers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id SMALLINT NOT NULL,
    option_id SMALLINT NOT NULL,
    discord_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (question_id) REFERENCES questions (id),
    FOREIGN KEY (option_id) REFERENCES possible_options (id),
    FOREIGN KEY (discord_id) REFERENCES users (discord_id),
    CONSTRAINT unique_answer_combination UNIQUE (question_id, option_id, discord_id)
);
