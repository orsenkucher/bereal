ALTER TABLE posts
    DROP COLUMN draft,
    DROP COLUMN published_at,
    DROP COLUMN visit_count,
    ADD published BOOLEAN NOT NULL DEFAULT FALSE    