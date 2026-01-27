use std::sync::Arc;

use crate::application::dto::RegisterRequest;
use crate::domain::user::{User, UserRepository};
use crate::error::AppError;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn register(
        &self,
        uid: String,
        email: String,
        request: RegisterRequest,
    ) -> Result<User, AppError> {
        // Vérifier si le username est déjà pris
        if self.user_repository.is_username_taken(&request.username).await? {
            return Err(AppError::Conflict("Ce nom d'utilisateur est déjà pris".into()));
        }

        // Vérifier si l'utilisateur existe déjà
        if self.user_repository.find_by_uid(&uid).await?.is_some() {
            return Err(AppError::Conflict("Un profil existe déjà pour cet utilisateur".into()));
        }

        // Créer le nouvel utilisateur
        let mut user = User::new(uid, email, request.username, request.display_name);
        user.bio = request.bio;

        self.user_repository.create(&user).await
    }

    pub async fn get_by_uid(&self, uid: &str) -> Result<Option<User>, AppError> {
        self.user_repository.find_by_uid(uid).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.user_repository.find_by_username(username).await
    }

    pub async fn update_profile(
        &self,
        uid: &str,
        display_name: Option<String>,
        bio: Option<String>,
    ) -> Result<User, AppError> {
        let mut user = self
            .user_repository
            .find_by_uid(uid)
            .await?
            .ok_or_else(|| AppError::NotFound("Utilisateur non trouvé".into()))?;

        if let Some(name) = display_name {
            user.display_name = name;
        }

        if let Some(bio_text) = bio {
            user.bio = Some(bio_text);
        }

        user.updated_at = chrono::Utc::now();

        self.user_repository.update(&user).await
    }
}
