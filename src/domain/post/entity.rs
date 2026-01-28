use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub author_uid: String,
    pub content: String,
    pub likes_count: i64,
    pub replies_count: i64,
    pub reposts_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
}

impl Post {
    pub fn new(author_uid: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            author_uid,
            content,
            likes_count: 0,
            replies_count: 0,
            reposts_count: 0,
            created_at: now,
            updated_at: now,
            parent_id: None,
        }
    }

    pub fn reply(author_uid: String, content: String, parent_id: Uuid) -> Self {
        let mut post = Self::new(author_uid, content);
        post.parent_id = Some(parent_id);
        post
    }
}
