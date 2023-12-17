-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY default gen_random_uuid(),
    username VARCHAR(255) NOT NULL,
    discord_id VARCHAR(255) NOT NULL,
    access_token VARCHAR(255) NULL,
    refresh_token VARCHAR(255) NULL,
    expires_in INT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (discord_id)
);
