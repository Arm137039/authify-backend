use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::user::User;

fn validate_username(username: &str) -> Result<(), validator::ValidationError> {
    let is_valid = username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_');

    if is_valid {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_username"))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 30, message = "Le username doit avoir entre 3 et 30 caractères"))]
    #[validate(custom(function = "validate_username", message = "Le username ne peut contenir que des lettres, chiffres et underscores"))]
    pub username: String,

    #[validate(length(min = 1, max = 100, message = "Le nom d'affichage doit avoir entre 1 et 100 caractères"))]
    pub display_name: String,

    #[validate(length(max = 500, message = "La bio ne peut pas dépasser 500 caractères"))]
    pub bio: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub uid: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub followers_count: u64,
    pub following_count: u64,
    pub posts_count: u64,
    pub is_verified: bool,
    pub is_private: bool,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            uid: user.uid,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            followers_count: user.followers_count,
            following_count: user.following_count,
            posts_count: user.posts_count,
            is_verified: user.is_verified,
            is_private: user.is_private,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
        }
    }
}
