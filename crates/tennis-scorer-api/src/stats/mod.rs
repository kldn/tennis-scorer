pub mod handlers;

use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stats/summary", get(handlers::summary))
        .route("/stats/match/{id}/analysis", get(handlers::match_analysis))
        .route("/stats/match/{id}/momentum", get(handlers::match_momentum))
        .route("/stats/match/{id}/pace", get(handlers::match_pace))
}
