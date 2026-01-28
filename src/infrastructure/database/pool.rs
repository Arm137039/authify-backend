use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::error::AppError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|e| AppError::Internal(format!("Erreur connexion DB: {}", e)))?;

    tracing::info!("Connexion PostgreSQL établie");

    Ok(pool)
}
