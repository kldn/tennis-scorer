## Why

Shuttle.rs is shutting down — Community tier accounts will be deleted in early 2026, and Pro tier projects were shut down on 2026-01-16. The tennis-scorer API currently depends on Shuttle for both runtime (`shuttle-runtime`, `shuttle-axum`) and database provisioning (`shuttle-shared-db`). We need to migrate to a maintained platform before service is terminated. Railway was chosen for its strong DX (git-push deploy), recent $100M Series B funding, and built-in Postgres plugin.

## What Changes

- **BREAKING**: Remove all Shuttle dependencies (`shuttle-axum`, `shuttle-shared-db`, `shuttle-runtime`)
- Replace `#[shuttle_runtime::main]` entry point with standard `#[tokio::main]` that reads `DATABASE_URL` and `JWT_SECRET` from environment variables
- Add `Dockerfile` for Railway deployment (multi-stage Rust build)
- Add `railway.toml` configuration file
- Update GitHub Actions CI/CD to deploy to Railway instead of Shuttle
- Move `dotenvy` from dev-dependencies to dependencies for local development
- Remove `Secrets.toml` usage in favor of standard `.env` / environment variables

## Capabilities

### New Capabilities

None — this is a platform migration, not a feature change.

### Modified Capabilities

- `api-infra`: Replace Shuttle.rs runtime requirement with standard tokio runtime + Railway deployment. Change local development from `cargo shuttle run` to `cargo run` with `.env` file. Configuration via environment variables instead of Shuttle SecretStore.

## Impact

- **Code**: `crates/tennis-scorer-api/src/main.rs` — rewrite entry point
- **Dependencies**: `crates/tennis-scorer-api/Cargo.toml` — remove 3 shuttle crates, promote dotenvy
- **CI/CD**: `.github/workflows/deploy.yml` — replace Shuttle CLI with Railway CLI
- **New files**: `Dockerfile`, `railway.toml` at API crate level
- **Deployment**: Railway project with Postgres plugin replaces Shuttle managed infra
