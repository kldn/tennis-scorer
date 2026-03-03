pub mod firebase;
pub mod handlers;
pub mod middleware;

use crate::AppState;
use axum::{Router, routing::put};

pub fn routes() -> Router<AppState> {
    Router::new().route("/auth/me", put(handlers::upsert_me))
}
