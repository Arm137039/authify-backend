use std::sync::Arc;
use uuid::Uuid;

use crate::domain::post::{Post, PostRepository};
use crate::error::AppError;

pub struct PostService {
    post_repository: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn create_post(&self, author_uid: String, content: String) -> Result<Post, AppError> {
        // Valider le contenu
        let content = content.trim().to_string();
        if content.is_empty() {
            return Err(AppError::Validation("Le contenu ne peut pas être vide".into()));
        }
        if content.len() > 280 {
            return Err(AppError::Validation("Le contenu ne peut pas dépasser 280 caractères".into()));
        }

        let post = Post::new(author_uid, content);
        self.post_repository.create(&post).await
    }

    pub async fn create_reply(
        &self,
        author_uid: String,
        content: String,
        parent_id: Uuid,
    ) -> Result<Post, AppError> {
        // Vérifier que le post parent existe
        self.post_repository
            .find_by_id(parent_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Post parent non trouvé".into()))?;

        // Valider le contenu
        let content = content.trim().to_string();
        if content.is_empty() {
            return Err(AppError::Validation("Le contenu ne peut pas être vide".into()));
        }
        if content.len() > 280 {
            return Err(AppError::Validation("Le contenu ne peut pas dépasser 280 caractères".into()));
        }

        let post = Post::reply(author_uid, content, parent_id);
        self.post_repository.create(&post).await
    }

    pub async fn get_post(&self, id: Uuid) -> Result<Option<Post>, AppError> {
        self.post_repository.find_by_id(id).await
    }

    pub async fn get_timeline(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let limit = limit.min(50).max(1);
        let offset = offset.max(0);
        self.post_repository.get_timeline(limit, offset).await
    }

    pub async fn get_user_posts(
        &self,
        author_uid: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, AppError> {
        let limit = limit.min(50).max(1);
        let offset = offset.max(0);
        self.post_repository.find_by_author(author_uid, limit, offset).await
    }

    pub async fn get_replies(&self, post_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let limit = limit.min(50).max(1);
        let offset = offset.max(0);
        self.post_repository.get_replies(post_id, limit, offset).await
    }

    pub async fn delete_post(&self, id: Uuid, requester_uid: &str) -> Result<(), AppError> {
        // Vérifier que le post existe et appartient à l'utilisateur
        let post = self
            .post_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Post non trouvé".into()))?;

        if post.author_uid != requester_uid {
            return Err(AppError::Forbidden("Vous ne pouvez pas supprimer ce post".into()));
        }

        self.post_repository.delete(id).await
    }
}
