use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::error::AppError;
use crate::AppState;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub uid: String,
    pub email: Option<String>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Header Authorization manquant".into()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Format d'autorisation invalide".into()))?;

    let claims = state.firebase.verify_id_token(token).await?;

    request.extensions_mut().insert(AuthenticatedUser {
        uid: claims.uid,
        email: claims.email,
    });

    Ok(next.run(request).await)
}
