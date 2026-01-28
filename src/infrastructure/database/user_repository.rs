use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::user::{User, UserRepository};
use crate::error::AppError;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_uid(&self, uid: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT uid, email, username, display_name, bio, avatar_url,
                followers_count, following_count, posts_count,
                is_verified, is_private, created_at, updated_at
            FROM users
            WHERE uid = $1
            "#,
        )
        .bind(uid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT uid, email, username, display_name, bio, avatar_url,
                followers_count, following_count, posts_count,
                is_verified, is_private, created_at, updated_at
            FROM users
            WHERE LOWER(username) = LOWER($1)
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<User, AppError> {
        let created = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (uid, email, username, display_name, bio, avatar_url,
                followers_count, following_count, posts_count, is_verified, is_private,
                created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING uid, email, username, display_name, bio, avatar_url,
                followers_count, following_count, posts_count,
                is_verified, is_private, created_at, updated_at
            "#,
        )
        .bind(&user.uid)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.display_name)
        .bind(&user.bio)
        .bind(&user.avatar_url)
        .bind(user.followers_count)
        .bind(user.following_count)
        .bind(user.posts_count)
        .bind(user.is_verified)
        .bind(user.is_private)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("duplicate key") || err_str.contains("unique constraint") {
                if err_str.contains("username") {
                    AppError::Conflict("Ce nom d'utilisateur est déjà pris".into())
                } else if err_str.contains("email") {
                    AppError::Conflict("Cet email est déjà utilisé".into())
                } else {
                    AppError::Conflict("Un utilisateur avec cet ID existe déjà".into())
                }
            } else {
                AppError::Internal(format!("Erreur DB: {}", e))
            }
        })?;

        tracing::info!("Utilisateur créé: {} ({})", created.username, created.uid);

        Ok(created)
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let updated = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET display_name = $2, bio = $3, avatar_url = $4, updated_at = NOW()
            WHERE uid = $1
            RETURNING uid, email, username, display_name, bio, avatar_url,
                followers_count, following_count, posts_count,
                is_verified, is_private, created_at, updated_at
            "#,
        )
        .bind(&user.uid)
        .bind(&user.display_name)
        .bind(&user.bio)
        .bind(&user.avatar_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(updated)
    }

    async fn delete(&self, uid: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE uid = $1")
            .bind(uid)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        tracing::info!("Utilisateur supprimé: {}", uid);

        Ok(())
    }

    async fn is_username_taken(&self, username: &str) -> Result<bool, AppError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM users WHERE LOWER(username) = LOWER($1))",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur DB: {}", e)))?;

        Ok(result.0)
    }
}
