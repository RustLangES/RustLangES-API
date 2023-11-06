-- Add migration script here
CREATE TABLE IF NOT EXISTS domains (
    id UUID PRIMARY KEY default gen_random_uuid(),
    domain VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS visits (
    id UUID PRIMARY KEY default gen_random_uuid(),
    domain_id UUID NOT NULL,
    visited_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (domain_id) REFERENCES domains (id)
);