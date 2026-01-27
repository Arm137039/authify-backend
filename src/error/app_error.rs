use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Non autorisé: {0}")]
    Unauthorized(String),

    #[error("Accès interdit: {0}")]
    Forbidden(String),

    #[error("Non trouvé: {0}")]
    NotFound(String),

    #[error("Erreur de validation: {0}")]
    Validation(String),

    #[error("Conflit: {0}")]
    Conflict(String),

    #[error("Erreur interne: {0}")]
    Internal(String),

    #[error("Erreur Firebase: {0}")]
    Firebase(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("Erreur interne: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Une erreur interne s'est produite".into(),
                )
            }
            AppError::Firebase(msg) => {
                tracing::error!("Erreur Firebase: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "FIREBASE_ERROR",
                    "Erreur du service d'authentification".into(),
                )
            }
        };

        let body = Json(ErrorResponse {
            error: ErrorBody { code, message },
        });

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
