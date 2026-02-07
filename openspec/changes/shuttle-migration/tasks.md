## 1. Update API crate dependencies

- [x] 1.1 Add `shuttle-axum`, `shuttle-shared-db` (postgres feature), `shuttle-runtime` to `Cargo.toml`
- [x] 1.2 Remove `dotenvy` from `Cargo.toml` (moved to dev-dependencies for integration tests)
- [x] 1.3 Run `cargo check -p tennis-scorer-api` to verify dependency resolution

## 2. Rewrite main.rs for Shuttle entry point

- [x] 2.1 Replace `#[tokio::main] async fn main()` with `#[shuttle_runtime::main]` entry point
- [x] 2.2 Inject `PgPool` via `#[shuttle_shared_db::Postgres]` parameter
- [x] 2.3 Inject `SecretStore` via `#[shuttle_runtime::Secrets]` parameter
- [x] 2.4 Extract `JWT_SECRET` from `SecretStore` and construct `AppConfig`
- [x] 2.5 Remove `dotenvy::dotenv()` call and manual `TcpListener` binding
- [x] 2.6 Return `ShuttleAxum` with the router from `create_router()`

## 3. Simplify config.rs

- [x] 3.1 Remove `database_url`, `host`, `port` from `AppConfig` struct (keep `jwt_secret` only)
- [x] 3.2 Remove `from_env()` method (no longer needed in production)
- [x] 3.3 Make struct fields public (direct construction via `AppConfig { jwt_secret }`)
- [x] 3.4 Update integration tests to construct `AppConfig` directly instead of `from_env()`
- [x] 3.5 Remove `use dotenvy` from production source files

## 4. Handle db.rs

- [x] 4.1 Keep `db::create_pool()` for integration tests (still needed)
- [x] 4.2 Add `sqlx::migrate!()` in main.rs for Shuttle runtime

## 5. Create Secrets.toml for local dev

- [x] 5.1 Create `Secrets.toml` in workspace root and API crate directory with `JWT_SECRET` placeholder
- [x] 5.2 Verify `Secrets.toml` is gitignored

## 6. Update Watch app base URL

- [x] 6.1 Change `APIClient.swift` baseURL from `https://tennis-scorer-api.fly.dev/api` to `https://tennis-scorer-api.shuttle.app/api`

## 7. Verify

- [x] 7.1 Run `cargo test -p tennis-scorer-api` — all existing tests pass
- [x] 7.2 Run `cargo shuttle run` locally — server starts and health check returns 200
- [x] 7.3 Verify API endpoints work (auth, matches, stats) via curl
