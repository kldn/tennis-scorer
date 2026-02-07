use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use tennis_scorer_api::config::AppConfig;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Note: Shuttle runtime already initializes a tracing subscriber.

    let jwt_secret = secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET must be set in Secrets.toml");

    let config = AppConfig { jwt_secret };

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = tennis_scorer_api::create_router(pool, &config);

    Ok(app.into())
}
