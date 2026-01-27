pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod presentation;

use std::sync::Arc;

use application::services::UserService;
use config::AppConfig;
use infrastructure::firebase::{FirebaseClient, InMemoryUserRepository};

#[derive(Clone)]
pub struct AppState {
    pub firebase: Arc<FirebaseClient>,
    pub user_service: Arc<UserService>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, error::AppError> {
        let firebase = FirebaseClient::new(&config).await?;
        let firebase = Arc::new(firebase);

        // Note: InMemoryUserRepository pour le développement
        // TODO: Remplacer par PostgreSQL ou Firestore pour la production
        let user_repository = Arc::new(InMemoryUserRepository::new());
        let user_service = Arc::new(UserService::new(user_repository));

        Ok(Self {
            firebase,
            user_service,
            config: Arc::new(config),
        })
    }
}
