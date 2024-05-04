-- Add up migration script here
CREATE TABLE IF NOT EXISTS ingredients (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    diet_friendly VARCHAR(50) ARRAY NOT NULL DEFAULT '{}'
);
