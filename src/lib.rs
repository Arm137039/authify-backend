pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod presentation;

use std::sync::Arc;

use application::services::{PostService, UserService};
use config::AppConfig;
use infrastructure::database::{create_pool, PostgresPostRepository, PostgresUserRepository};
use infrastructure::firebase::FirebaseClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub firebase: Arc<FirebaseClient>,
    pub user_service: Arc<UserService>,
    pub post_service: Arc<PostService>,
    pub config: Arc<AppConfig>,
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, error::AppError> {
        // Connexion à la base de données
        let db_pool = create_pool(&config.database_url).await?;

        // Client Firebase pour la vérification des tokens
        let firebase = FirebaseClient::new(&config).await?;
        let firebase = Arc::new(firebase);

        // Repositories
        let user_repository = Arc::new(PostgresUserRepository::new(db_pool.clone()));
        let post_repository = Arc::new(PostgresPostRepository::new(db_pool.clone()));

        // Services
        let user_service = Arc::new(UserService::new(user_repository));
        let post_service = Arc::new(PostService::new(post_repository));

        Ok(Self {
            firebase,
            user_service,
            post_service,
            config: Arc::new(config),
            db_pool,
        })
    }

    /// Helper pour exécuter un fichier de migration avec plusieurs statements
    async fn execute_migration_file(
        &self,
        sql: &str,
        name: &str,
    ) -> Result<(), error::AppError> {
        for statement in sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("--") {
                sqlx::query(statement)
                    .execute(&self.db_pool)
                    .await
                    .map_err(|e| {
                        error::AppError::Internal(format!("Migration {} échouée: {}", name, e))
                    })?;
            }
        }
        Ok(())
    }

    /// Exécute les migrations SQL
    pub async fn run_migrations(&self) -> Result<(), error::AppError> {
        tracing::info!("Exécution des migrations...");

        // Migration 1: users
        self.execute_migration_file(
            include_str!("../migrations/001_create_users.sql"),
            "users",
        )
        .await?;

        // Migration 2: posts
        self.execute_migration_file(
            include_str!("../migrations/002_create_posts.sql"),
            "posts",
        )
        .await?;

        // Migration 3: likes
        self.execute_migration_file(
            include_str!("../migrations/003_create_likes.sql"),
            "likes",
        )
        .await?;

        tracing::info!("Migrations terminées avec succès");

        Ok(())
    }
}
