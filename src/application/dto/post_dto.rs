use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::post::Post;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 280, message = "Le contenu doit avoir entre 1 et 280 caractères"))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReplyRequest {
    #[validate(length(min = 1, max = 280, message = "Le contenu doit avoir entre 1 et 280 caractères"))]
    pub content: String,
    pub parent_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_uid: String,
    pub content: String,
    pub likes_count: i64,
    pub replies_count: i64,
    pub reposts_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
    pub created_at: String,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_uid: post.author_uid,
            content: post.content,
            likes_count: post.likes_count,
            replies_count: post.replies_count,
            reposts_count: post.reposts_count,
            parent_id: post.parent_id,
            created_at: post.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostsListResponse {
    pub posts: Vec<PostResponse>,
    pub count: usize,
}

impl PostsListResponse {
    pub fn from_posts(posts: Vec<Post>) -> Self {
        let count = posts.len();
        Self {
            posts: posts.into_iter().map(PostResponse::from).collect(),
            count,
        }
    }
}
