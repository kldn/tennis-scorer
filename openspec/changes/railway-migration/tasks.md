## 1. Update Dependencies

- [x] 1.1 Remove `shuttle-axum`, `shuttle-shared-db`, `shuttle-runtime` from `crates/tennis-scorer-api/Cargo.toml`
- [x] 1.2 Move `dotenvy` from `[dev-dependencies]` to `[dependencies]`

## 2. Rewrite Entry Point

- [x] 2.1 Rewrite `crates/tennis-scorer-api/src/main.rs` to use `#[tokio::main]` — load `.env` via dotenvy, read `DATABASE_URL` and `JWT_SECRET` from env, create PgPool, run migrations, bind Axum server to `HOST:PORT` (default `0.0.0.0:8000`)

## 3. Deployment Configuration

- [x] 3.1 Create `Dockerfile` at repo root (workspace build context) — multi-stage build with cargo-chef for caching, `rust:1-slim` builder, `debian:bookworm-slim` runtime
- [x] 3.2 Create `railway.toml` at repo root with build and deploy config

## 4. CI/CD Pipeline

- [x] 4.1 Update `.github/workflows/deploy.yml` — replace Shuttle CLI install + deploy with Railway CLI install + `railway up`

## 5. Local Development

- [x] 5.1 Add `crates/tennis-scorer-api/.env.example` with `DATABASE_URL` and `JWT_SECRET` placeholders

## 6. Verification

- [x] 6.1 Run `cargo build -p tennis-scorer-api` to verify compilation
- [x] 6.2 Run `cargo test -p tennis-scorer-api` to verify tests pass
