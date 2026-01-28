use std::net::SocketAddr;

use authify_api::{config::AppConfig, presentation::routes::create_router, AppState};

#[tokio::main]
async fn main() {
    // Charger les variables d'environnement
    dotenvy::dotenv().ok();

    // Initialiser le système de logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Charger la configuration
    let config = AppConfig::from_env().expect("Erreur de configuration");
    let port = config.port;

    // Initialiser l'état de l'application
    let state = AppState::new(config)
        .await
        .expect("Erreur d'initialisation");

    // Exécuter les migrations
    state
        .run_migrations()
        .await
        .expect("Erreur lors des migrations");

    // Créer le routeur
    let app = create_router(state);

    // Démarrer le serveur
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Serveur démarré sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
