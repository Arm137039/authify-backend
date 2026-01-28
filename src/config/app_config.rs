use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub database_url: String,
    pub firebase_project_id: String,
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

        let database_url = env::var("DATABASE_URL")?;
        let firebase_project_id = env::var("FIREBASE_PROJECT_ID")?;

        Ok(Self {
            port,
            allowed_origins,
            database_url,
            firebase_project_id,
        })
    }
}
