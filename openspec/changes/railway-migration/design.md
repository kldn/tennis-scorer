## Context

The tennis-scorer API is an Axum + SQLx + Postgres backend deployed on Shuttle.rs. Shuttle is shutting down, so we must migrate to a new platform. The core application logic (`create_router`, handlers, database queries) is cleanly separated in `lib.rs` — only `main.rs` depends on Shuttle. CI/CD in `.github/workflows/deploy.yml` uses Shuttle CLI for deployment.

Railway was selected after comparing with Fly.io. Railway offers git-push deploys, a Postgres plugin, and strong financial health ($100M Series B, Jan 2026).

## Goals / Non-Goals

**Goals:**
- Replace Shuttle runtime with standard `#[tokio::main]` entry point
- Deploy on Railway with Postgres plugin
- Update CI/CD pipeline for Railway deployment
- Zero downtime for API consumers (same endpoints, same behavior)
- Local development works with `cargo run` + `.env` file

**Non-Goals:**
- Changing API endpoints or behavior
- Multi-region deployment
- Custom domain setup (can be done later)
- Database migration from Shuttle Postgres to Railway Postgres (manual one-time data export/import, out of scope for code changes)

## Decisions

### 1. Standard `#[tokio::main]` with `dotenvy`

Replace Shuttle's macro-based entry point with a plain tokio main function. Use `dotenvy` (already a dev-dependency) to load `.env` files for local development. Read `DATABASE_URL` and `JWT_SECRET` from `std::env`.

**Why not** keep an abstraction layer: The Shuttle macros hid 5 lines of boilerplate. A plain main.rs is ~20 lines, trivially readable, and has zero platform lock-in.

### 2. Multi-stage Dockerfile

Railway can build from Dockerfile. Use a multi-stage build:
- Stage 1: `rust:1-slim` — build the binary with `--release`
- Stage 2: `debian:bookworm-slim` — copy binary + run

**Why not** Nixpacks (Railway default): Nixpacks auto-detects Rust but offers less control over build caching and final image size. A Dockerfile gives deterministic builds.

### 3. Railway CLI in GitHub Actions

Replace `cargo-shuttle` with `railway` CLI. Use `railway up` for deployment triggered on push to master.

**Why not** Railway's built-in git integration: GitHub Actions gives us the test step before deploy. Railway's git integration would deploy on every push without running tests first.

### 4. Connection pooling via DATABASE_URL

Railway Postgres plugin provides a `DATABASE_URL` environment variable automatically. SQLx connects directly via this URL. No connection pooler needed at current scale.

## Risks / Trade-offs

- **[Data migration]** → Out of scope. User must manually export data from Shuttle Postgres and import to Railway Postgres before switching DNS/clients. The schema will be recreated by sqlx migrations.
- **[Build time]** → Rust Docker builds are slow (~5-10 min). Mitigated by cargo chef for layer caching in Dockerfile.
- **[Railway free tier limits]** → Hobby plan includes $5/month credits. Sufficient for a personal project with low traffic.

## Migration Plan

1. Create Railway project + Postgres plugin
2. Update code (main.rs, Cargo.toml, Dockerfile, railway.toml)
3. Test locally with `cargo run` + `.env`
4. Deploy to Railway, verify health check
5. Update GitHub Actions workflow
6. Export/import data from Shuttle Postgres (manual)
7. Point watchOS app to new Railway URL
