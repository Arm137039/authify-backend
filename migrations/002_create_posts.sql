-- Create posts table
CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_uid VARCHAR(128) NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
    content TEXT NOT NULL,
    likes_count BIGINT NOT NULL DEFAULT 0,
    replies_count BIGINT NOT NULL DEFAULT 0,
    reposts_count BIGINT NOT NULL DEFAULT 0,
    parent_id UUID REFERENCES posts(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for author lookups
CREATE INDEX IF NOT EXISTS idx_posts_author ON posts(author_uid);

-- Index for timeline (most recent first)
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);

-- Index for replies
CREATE INDEX IF NOT EXISTS idx_posts_parent ON posts(parent_id) WHERE parent_id IS NOT NULL;
