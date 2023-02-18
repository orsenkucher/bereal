CREATE TABLE users (
    id UUID PRIMARY KEY,
    chat_id TEXT NOT NULL,
    phone_number TEXT,
    joined_at TIMESTAMP NOT NULL,
    language TEXT
)
