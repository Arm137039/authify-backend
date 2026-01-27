use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub firebase_project_id: String,
    pub google_credentials_path: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8081".to_string())
            .parse()
            .expect("PORT doit être un nombre valide");

        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let firebase_project_id = env::var("FIREBASE_PROJECT_ID")?;
        let google_credentials_path = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;

        Ok(Self {
            port,
            allowed_origins,
            firebase_project_id,
            google_credentials_path,
        })
    }
}
