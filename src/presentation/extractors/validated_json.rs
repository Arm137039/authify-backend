use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Validation(format!("JSON invalide: {}", e)))?;

        value
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation échouée: {}", e)))?;

        Ok(ValidatedJson(value))
    }
}
