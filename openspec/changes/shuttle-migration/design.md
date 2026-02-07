## Context

The `tennis-scorer-api` crate currently uses a manual Axum setup with `dotenvy` for environment variables and was designed for Fly.io/Docker deployment. The Docker and Fly.io config files have already been removed. The API needs to be migrated to Shuttle.rs, which provides:

- Automatic PostgreSQL provisioning (no docker-compose needed for local dev)
- Rust-native deployment (no Dockerfile)
- Secrets management via `Secrets.toml` (local) and Shuttle dashboard (prod)

Current entry point (`main.rs`) manually reads env vars, creates a PgPool, binds a TCP listener. Shuttle replaces all of this with its `#[shuttle_runtime::main]` macro and resource injection.

Current files to modify:
- `crates/tennis-scorer-api/Cargo.toml` — add shuttle deps, remove dotenvy
- `crates/tennis-scorer-api/src/main.rs` — rewrite to Shuttle entry point
- `crates/tennis-scorer-api/src/config.rs` — simplify (JWT_SECRET from Shuttle secrets)
- `crates/tennis-scorer-api/src/db.rs` — may become unused (Shuttle provisions PgPool)
- `WatchApp/.../APIClient.swift` — update base URL

## Goals / Non-Goals

**Goals:**
- Replace manual Axum startup with `#[shuttle_runtime::main]` entry point
- Use `shuttle-shared-db` for automatic PostgreSQL provisioning
- Use `shuttle_runtime::SecretStore` for JWT_SECRET and other secrets
- Maintain all existing API functionality (auth, matches, stats, health)
- Keep `cargo test` working (tests don't use Shuttle runtime)
- Update Watch app base URL to Shuttle.rs deployment URL
- Local development via `cargo shuttle run`

**Non-Goals:**
- Changing API endpoints or business logic
- Modifying database schema or migrations
- Adding new features to the API
- Migrating existing production data (no production data yet)

## Decisions

### 1. Shuttle entry point pattern

Use `#[shuttle_runtime::main]` with `ShuttleAxum` return type. Shuttle injects `PgPool` via `#[shuttle_shared_db::Postgres]` and secrets via `#[shuttle_runtime::Secrets]`.

```rust
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleAxum { ... }
```

**Why:** This is the standard Shuttle pattern. Removes all manual connection setup. Shuttle handles connection pooling and provisioning.

### 2. Config simplification

Remove `dotenvy` dependency entirely. `AppConfig` struct simplified to only hold `jwt_secret` (no more `database_url`, `host`, `port` — Shuttle manages these).

**Alternative considered:** Keep `AppConfig::from_env()` for test compatibility. Rejected because tests can construct `AppConfig` directly.

### 3. Keep `create_router()` in lib.rs unchanged

The router construction in `lib.rs` stays the same — it takes `PgPool` and `AppConfig` and returns a `Router`. This keeps `lib.rs` testable without Shuttle runtime.

**Why:** Separation of concerns. Only `main.rs` depends on Shuttle; the library remains framework-agnostic.

### 4. db.rs becomes optional

`db::create_pool()` is no longer needed for production (Shuttle provides the pool). Keep it for tests that need a manual pool, or remove it if tests use Shuttle's test utilities.

### 5. Shuttle.toml at workspace root

`Shuttle.toml` already exists at workspace root with `name = "tennis-scorer-api"`. The deploy command uses `--working-directory crates/tennis-scorer-api`.

## Risks / Trade-offs

- **[Risk] Shuttle.rs service availability** → Shuttle is a managed platform; if it goes down, API is unreachable. Mitigation: offline-first Watch app design means matches are never lost.
- **[Risk] sqlx migrations** → Shuttle's shared-db auto-runs migrations from the `migrations/` directory. Verify migration path resolves correctly from the API crate working directory.
- **[Risk] Base URL change** → Watch apps already deployed with old URL will fail to sync. Mitigation: no production users yet; hardcoded URL update is acceptable.
- **[Trade-off] Vendor lock-in** → Shuttle-specific macros in main.rs. Mitigation: only main.rs depends on Shuttle; lib.rs is portable. Migration to another platform only requires a new main.rs.
