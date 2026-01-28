use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::application::dto::{
    ApiResponse, CreatePostRequest, PaginationQuery, PostResponse, PostsListResponse,
};
use crate::error::AppError;
use crate::presentation::extractors::{AuthUser, ValidatedJson};
use crate::AppState;

/// GET /api/v1/posts - Timeline des posts
pub async fn get_posts(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PostsListResponse>>, AppError> {
    let posts = state
        .post_service
        .get_timeline(pagination.limit, pagination.offset)
        .await?;

    Ok(Json(ApiResponse::success(PostsListResponse::from_posts(posts))))
}

/// POST /api/v1/posts - Créer un post
pub async fn create_post(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    ValidatedJson(payload): ValidatedJson<CreatePostRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PostResponse>>), AppError> {
    let post = state
        .post_service
        .create_post(auth_user.uid, payload.content)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::with_message(
            PostResponse::from(post),
            "Post créé avec succès",
        )),
    ))
}

/// GET /api/v1/posts/:id - Obtenir un post
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    let post = state
        .post_service
        .get_post(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Post non trouvé".into()))?;

    Ok(Json(ApiResponse::success(PostResponse::from(post))))
}

/// DELETE /api/v1/posts/:id - Supprimer un post
pub async fn delete_post(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.post_service.delete_post(id, &auth_user.uid).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/posts/:id/replies - Obtenir les réponses à un post
pub async fn get_post_replies(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PostsListResponse>>, AppError> {
    let posts = state
        .post_service
        .get_replies(id, pagination.limit, pagination.offset)
        .await?;

    Ok(Json(ApiResponse::success(PostsListResponse::from_posts(posts))))
}

/// POST /api/v1/posts/:id/replies - Répondre à un post
pub async fn create_reply(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(parent_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreatePostRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PostResponse>>), AppError> {
    let post = state
        .post_service
        .create_reply(auth_user.uid, payload.content, parent_id)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::with_message(
            PostResponse::from(post),
            "Réponse créée avec succès",
        )),
    ))
}

/// GET /api/v1/users/:uid/posts - Obtenir les posts d'un utilisateur
pub async fn get_user_posts(
    State(state): State<AppState>,
    Path(uid): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PostsListResponse>>, AppError> {
    let posts = state
        .post_service
        .get_user_posts(&uid, pagination.limit, pagination.offset)
        .await?;

    Ok(Json(ApiResponse::success(PostsListResponse::from_posts(posts))))
}
