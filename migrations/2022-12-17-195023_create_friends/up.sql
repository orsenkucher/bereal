CREATE TABLE friends (
    id UUID PRIMARY KEY,
    user_id UUID references users(id) NOT NULL,
    friend_id UUID references users(id) NOT NULL
)