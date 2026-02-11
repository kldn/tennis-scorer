pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod matches;
pub mod stats;

use axum::Router;
use sqlx::PgPool;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn create_router(pool: PgPool, config: &AppConfig) -> Router {
    let state = AppState {
        pool,
        jwt_secret: config.jwt_secret.clone(),
    };

    let cors = if config.allowed_origins.is_empty() {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        let origins: Vec<_> = config
            .allowed_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(Any)
            .allow_headers(Any)
    };

    Router::new()
        .nest("/api", api_routes())
        .layer(cors)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(matches::routes())
        .merge(stats::routes())
        .route("/health", axum::routing::get(health_check))
}

async fn health_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<axum::Json<serde_json::Value>, error::AppError> {
    sqlx::query("SELECT 1").execute(&state.pool).await?;
    Ok(axum::Json(serde_json::json!({"status": "ok"})))
}
