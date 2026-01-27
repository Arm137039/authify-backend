use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Charger les variables d'environnement
    dotenv::dotenv().ok();
    
    // Configuration des logs
    tracing_subscriber::fmt::init();
    
    // Port depuis .env ou 8080 par défaut
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    
    // Router minimal
    let app = Router::new()
        .route("/", get(|| async { "Authify API is running!" }))
        .route("/health", get(|| async { "OK" }));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    tracing::info!("🚀 Server starting on {}", addr);
    
    // Axum 0.7 utilise tokio::net::TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
