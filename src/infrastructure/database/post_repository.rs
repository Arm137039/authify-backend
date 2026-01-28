use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::post::{Post, PostRepository};
use crate::error::AppError;

pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: &Post) -> Result<Post, AppError> {
        let created = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at
            "#,
        )
        .bind(post.id)
        .bind(&post.author_uid)
        .bind(&post.content)
        .bind(post.likes_count)
        .bind(post.replies_count)
        .bind(post.reposts_count)
        .bind(post.parent_id)
        .bind(post.created_at)
        .bind(post.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur création post: {}", e)))?;

        // Incrémenter le compteur de posts de l'utilisateur
        sqlx::query("UPDATE users SET posts_count = posts_count + 1 WHERE uid = $1")
            .bind(&post.author_uid)
            .execute(&self.pool)
            .await
            .ok();

        // Si c'est une réponse, incrémenter le compteur de réponses du parent
        if let Some(parent_id) = post.parent_id {
            self.increment_replies(parent_id).await.ok();
        }

        tracing::info!("Post créé: {} par {}", created.id, created.author_uid);

        Ok(created)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(post)
    }

    async fn find_by_author(&self, author_uid: &str, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at
            FROM posts
            WHERE author_uid = $1 AND parent_id IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(author_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(posts)
    }

    async fn get_timeline(&self, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at
            FROM posts
            WHERE parent_id IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(posts)
    }

    async fn get_replies(&self, parent_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, author_uid, content, likes_count, replies_count,
                reposts_count, parent_id, created_at, updated_at
            FROM posts
            WHERE parent_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(parent_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(posts)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        // Récupérer le post pour décrémenter le compteur de l'utilisateur
        if let Some(post) = self.find_by_id(id).await? {
            sqlx::query("DELETE FROM posts WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

            // Décrémenter le compteur de posts de l'utilisateur
            sqlx::query("UPDATE users SET posts_count = GREATEST(posts_count - 1, 0) WHERE uid = $1")
                .bind(&post.author_uid)
                .execute(&self.pool)
                .await
                .ok();

            tracing::info!("Post supprimé: {}", id);
        }

        Ok(())
    }

    async fn increment_likes(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE posts SET likes_count = likes_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(())
    }

    async fn decrement_likes(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE posts SET likes_count = GREATEST(likes_count - 1, 0) WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(())
    }

    async fn increment_replies(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE posts SET replies_count = replies_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(())
    }
}
