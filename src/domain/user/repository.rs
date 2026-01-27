use async_trait::async_trait;

use super::User;
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_uid(&self, uid: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn create(&self, user: &User) -> Result<User, AppError>;
    async fn update(&self, user: &User) -> Result<User, AppError>;
    async fn delete(&self, uid: &str) -> Result<(), AppError>;
    async fn is_username_taken(&self, username: &str) -> Result<bool, AppError>;
}
