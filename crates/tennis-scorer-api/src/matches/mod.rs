pub mod handlers;
pub mod models;

use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/matches",
            post(handlers::create_match).get(handlers::list_matches),
        )
        .route(
            "/matches/{id}",
            get(handlers::get_match).delete(handlers::delete_match),
        )
        // Debug: no-auth endpoint for local testing
        .merge(debug_routes())
}

#[cfg(debug_assertions)]
fn debug_routes() -> Router<AppState> {
    Router::new().route("/debug/matches", post(handlers::create_match_debug))
}

#[cfg(not(debug_assertions))]
fn debug_routes() -> Router<AppState> {
    Router::new()
}
