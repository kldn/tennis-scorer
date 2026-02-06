## Why

The tennis-scorer engine runs locally on the Apple Watch with no persistence beyond the device. To enable match history, cross-device access, statistics, and future Flutter/Python clients, we need a backend API that receives and stores completed match data.

## What Changes

- Build a REST API using Axum inside the existing `tennis-scorer-api` crate
- Add PostgreSQL persistence via sqlx with compile-time checked queries
- Add JWT-based authentication (register/login) for multi-user support
- Expose endpoints: auth, match CRUD, basic statistics
- Add database migrations for users, matches, and match events
- Add Docker / docker-compose configuration for local development
- Configure for deployment on Fly.io

## Capabilities

### New Capabilities
- `api-auth`: JWT-based user registration and login (email/password with argon2 hashing)
- `api-matches`: CRUD endpoints for match data — create match, list matches, get match detail with point events
- `api-stats`: Aggregated statistics endpoint — win/loss record, recent form, per-opponent breakdown
- `api-infra`: Database schema (migrations), configuration, Docker setup, health check

### Modified Capabilities
- `workspace-structure`: The `tennis-scorer-api` crate gains real dependencies (axum, sqlx, etc.) and becomes a binary crate with `main.rs`

## Impact

- **Code**: `crates/tennis-scorer-api/` transforms from placeholder to full API server
- **Dependencies**: axum, sqlx, tokio, jsonwebtoken, argon2, serde, tower-http (CORS)
- **Infrastructure**: PostgreSQL database required; Docker Compose for local dev
- **CI/CD**: `deploy.yml` workflow will need real build + deploy steps (Fly.io)
- **Existing code**: No changes to `tennis-scorer` core crate
