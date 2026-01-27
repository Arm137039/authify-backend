use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::domain::user::{User, UserRepository};
use crate::error::AppError;

/// Repository utilisateur en mémoire pour le développement.
/// TODO: Remplacer par une implémentation PostgreSQL ou Firestore pour la production.
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
    usernames: Arc<RwLock<HashMap<String, String>>>, // username -> uid
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            usernames: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_uid(&self, uid: &str) -> Result<Option<User>, AppError> {
        let users = self.users.read().await;
        Ok(users.get(uid).cloned())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let usernames = self.usernames.read().await;
        let username_lower = username.to_lowercase();

        if let Some(uid) = usernames.get(&username_lower) {
            let users = self.users.read().await;
            Ok(users.get(uid).cloned())
        } else {
            Ok(None)
        }
    }

    async fn create(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.write().await;
        let mut usernames = self.usernames.write().await;

        // Vérifier que l'uid n'existe pas déjà
        if users.contains_key(&user.uid) {
            return Err(AppError::Conflict("Un utilisateur avec cet ID existe déjà".into()));
        }

        // Vérifier que le username n'est pas pris
        let username_lower = user.username.to_lowercase();
        if usernames.contains_key(&username_lower) {
            return Err(AppError::Conflict("Ce nom d'utilisateur est déjà pris".into()));
        }

        // Insérer l'utilisateur
        users.insert(user.uid.clone(), user.clone());
        usernames.insert(username_lower, user.uid.clone());

        tracing::info!("Utilisateur créé: {} ({})", user.username, user.uid);

        Ok(user.clone())
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.write().await;

        if !users.contains_key(&user.uid) {
            return Err(AppError::NotFound("Utilisateur non trouvé".into()));
        }

        users.insert(user.uid.clone(), user.clone());

        Ok(user.clone())
    }

    async fn delete(&self, uid: &str) -> Result<(), AppError> {
        let mut users = self.users.write().await;
        let mut usernames = self.usernames.write().await;

        if let Some(user) = users.remove(uid) {
            usernames.remove(&user.username.to_lowercase());
            tracing::info!("Utilisateur supprimé: {}", uid);
        }

        Ok(())
    }

    async fn is_username_taken(&self, username: &str) -> Result<bool, AppError> {
        let usernames = self.usernames.read().await;
        Ok(usernames.contains_key(&username.to_lowercase()))
    }
}
