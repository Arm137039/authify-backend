use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::presentation::handlers;
use crate::presentation::middleware::auth_middleware;
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    // Routes publiques
    let public_routes = Router::new()
        .route("/health", get(handlers::health_check));

    // Routes protégées par authentification
    let protected_routes = Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/me", get(handlers::get_me))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Configuration CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(public_routes)
        .nest("/api/v1", protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
