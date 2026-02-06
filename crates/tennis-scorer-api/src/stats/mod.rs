pub mod handlers;

use crate::AppState;
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new().route("/stats/summary", get(handlers::summary))
}
