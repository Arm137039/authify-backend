use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub followers_count: i64,
    pub following_count: i64,
    pub posts_count: i64,
    pub is_verified: bool,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(uid: String, email: String, username: String, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            uid,
            email,
            username,
            display_name,
            bio: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
            followers_count: 0,
            following_count: 0,
            posts_count: 0,
            is_verified: false,
            is_private: false,
        }
    }
}
