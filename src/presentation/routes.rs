use axum::{
    middleware,
    routing::{delete, get, post},
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
        .route("/health", get(handlers::health_check))
        // Timeline et lecture de posts (public)
        .route("/api/v1/posts", get(handlers::get_posts))
        .route("/api/v1/posts/{id}", get(handlers::get_post))
        .route("/api/v1/posts/{id}/replies", get(handlers::get_post_replies))
        .route("/api/v1/users/{uid}/posts", get(handlers::get_user_posts));

    // Routes protégées par authentification
    let protected_routes = Router::new()
        // Auth
        .route("/auth/register", post(handlers::register))
        .route("/auth/me", get(handlers::get_me))
        // Posts (écriture)
        .route("/posts", post(handlers::create_post))
        .route("/posts/{id}", delete(handlers::delete_post))
        .route("/posts/{id}/replies", post(handlers::create_reply))
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
