use axum::{extract::State, http::StatusCode, Json};

use crate::application::dto::{ApiResponse, RegisterRequest, UserResponse};
use crate::error::AppError;
use crate::presentation::extractors::{AuthUser, ValidatedJson};
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), AppError> {
    let email = auth_user.email.unwrap_or_default();

    let user = state
        .user_service
        .register(auth_user.uid, email, payload)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::with_message(
            UserResponse::from(user),
            "Profil créé avec succès",
        )),
    ))
}

pub async fn get_me(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = state
        .user_service
        .get_by_uid(&auth_user.uid)
        .await?
        .ok_or_else(|| AppError::NotFound("Profil non trouvé. Veuillez d'abord créer votre profil.".into()))?;

    Ok(Json(ApiResponse::success(UserResponse::from(user))))
}
