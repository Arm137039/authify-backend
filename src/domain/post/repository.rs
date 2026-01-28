use async_trait::async_trait;
use uuid::Uuid;

use super::Post;
use crate::error::AppError;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: &Post) -> Result<Post, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError>;
    async fn find_by_author(&self, author_uid: &str, limit: i64, offset: i64) -> Result<Vec<Post>, AppError>;
    async fn get_timeline(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError>;
    async fn get_replies(&self, parent_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn increment_likes(&self, id: Uuid) -> Result<(), AppError>;
    async fn decrement_likes(&self, id: Uuid) -> Result<(), AppError>;
    async fn increment_replies(&self, id: Uuid) -> Result<(), AppError>;
}
