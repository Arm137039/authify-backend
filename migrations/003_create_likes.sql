-- Create likes table for tracking user likes on posts
CREATE TABLE IF NOT EXISTS likes (
    user_uid VARCHAR(128) NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_uid, post_id)
);

-- Index for checking if user liked a post
CREATE INDEX IF NOT EXISTS idx_likes_post ON likes(post_id);
