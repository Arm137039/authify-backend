use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::error::AppError;
use crate::presentation::middleware::AuthenticatedUser;

pub struct AuthUser(pub AuthenticatedUser);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .map(AuthUser)
            .ok_or_else(|| AppError::Unauthorized("Non authentifié".into()))
    }
}
