## Why

The API backend was originally designed for Fly.io deployment with Docker. We've decided to switch to Shuttle.rs — a Rust-native deployment platform that doesn't require Docker, simplifies configuration, and auto-provisions PostgreSQL.

## What Changes

- Add `shuttle-axum` and `shuttle-shared-db` dependencies to `tennis-scorer-api/Cargo.toml`
- Rewrite `main.rs` to use Shuttle entry point (`#[shuttle_runtime::main]`) instead of manual Axum setup
- Simplify `config.rs` — remove `dotenvy`, use Shuttle secrets for `JWT_SECRET` and other config
- Create `Shuttle.toml` at workspace root (done)
- Create `Secrets.toml` for local development (in .gitignore, done)
- Update `APIClient.swift` base URL to point to Shuttle.rs deployment
- Test locally with `cargo shuttle run`
- Deploy with `cargo shuttle deploy`

## Capabilities

### New Capabilities
- `shuttle-deployment`: Shuttle.rs-based deployment pipeline replacing Docker/Fly.io

### Modified Capabilities
- `api-backend`: Entry point and config rewritten for Shuttle.rs
- `cicd-pipelines`: deploy.yml already updated for `cargo shuttle deploy`

## Impact

- **API crate**: main.rs, config.rs, Cargo.toml modified
- **Watch App**: APIClient.swift base URL change
- **Dependencies**: `shuttle-axum`, `shuttle-shared-db` added; `dotenvy` removed
- **Removed**: Dockerfile, fly.toml, .dockerignore, docker-compose.yml (already deleted)
- **Depends on**: `api-backend` (#5, completed)
