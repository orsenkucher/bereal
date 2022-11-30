ALTER TABLE posts 
    ADD draft BOOLEAN NOT NULL DEFAULT TRUE,
    ADD published_at TIMESTAMP NOT NULL,
    ADD visit_count INTEGER NOT NULL,
    DROP COLUMN published
