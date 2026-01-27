use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::error::AppError;

const GOOGLE_CERTS_URL: &str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

#[derive(Clone)]
pub struct FirebaseClient {
    project_id: String,
    http_client: reqwest::Client,
    cached_keys: Arc<RwLock<HashMap<String, DecodingKey>>>,
}

#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub uid: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseTokenClaims {
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub exp: usize,
    pub iat: usize,
}

impl FirebaseClient {
    pub async fn new(config: &AppConfig) -> Result<Self, AppError> {
        let http_client = reqwest::Client::new();

        let client = Self {
            project_id: config.firebase_project_id.clone(),
            http_client,
            cached_keys: Arc::new(RwLock::new(HashMap::new())),
        };

        // Pre-fetch keys
        client.refresh_keys().await?;

        tracing::info!("Firebase client initialisé pour le projet: {}", config.firebase_project_id);

        Ok(client)
    }

    async fn refresh_keys(&self) -> Result<(), AppError> {
        let response: HashMap<String, String> = self
            .http_client
            .get(GOOGLE_CERTS_URL)
            .send()
            .await
            .map_err(|e| AppError::Firebase(format!("Erreur téléchargement clés Google: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::Firebase(format!("Erreur parsing clés Google: {}", e)))?;

        let mut keys = self.cached_keys.write().await;
        keys.clear();

        for (kid, cert_pem) in response {
            match DecodingKey::from_rsa_pem(cert_pem.as_bytes()) {
                Ok(key) => {
                    keys.insert(kid, key);
                }
                Err(e) => {
                    tracing::warn!("Erreur parsing certificat {}: {}", kid, e);
                }
            }
        }

        tracing::debug!("Clés Firebase actualisées: {} clés chargées", keys.len());

        Ok(())
    }

    pub async fn verify_id_token(&self, token: &str) -> Result<TokenClaims, AppError> {
        // Decode header to get the key ID
        let header = decode_header(token)
            .map_err(|e| AppError::Unauthorized(format!("Token invalide: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::Unauthorized("Token sans key ID".into()))?;

        // Get the decoding key
        let keys = self.cached_keys.read().await;
        let decoding_key = keys
            .get(&kid)
            .ok_or_else(|| AppError::Unauthorized("Clé de signature inconnue".into()))?;

        // Set up validation
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.project_id]);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]);

        // Decode and verify the token
        let token_data = decode::<FirebaseTokenClaims>(token, decoding_key, &validation)
            .map_err(|e| AppError::Unauthorized(format!("Token invalide: {}", e)))?;

        Ok(TokenClaims {
            uid: token_data.claims.sub,
            email: token_data.claims.email,
        })
    }
}
