CREATE TABLE users (
    id UUID PRIMARY KEY,
    telegram_id TEXT NOT NULL,
    name TEXT,
    joined_at TIMESTAMP NOT NULL
)
