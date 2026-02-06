## Context

The tennis-scorer project currently has a core scoring engine (`tennis-scorer` crate) and an empty placeholder API crate (`tennis-scorer-api`). The Watch app stores match data only locally. To enable cross-device access, historical statistics, and future Flutter/Python clients, we need a backend API.

The design doc at `docs/plans/2026-02-06-backend-and-cicd-design.md` established the high-level architecture: Rust (Axum) API + PostgreSQL + Fly.io deployment. This design fills in the implementation details.

## Goals / Non-Goals

**Goals:**
- REST API with JWT auth that clients can use to store and retrieve match data
- PostgreSQL schema for users, matches, and point events
- Local development setup with Docker Compose
- Deployable to Fly.io
- Share types from `tennis-scorer` core crate (MatchConfig, Player, etc.)

**Non-Goals:**
- Real-time features (WebSockets, live scoring) — future work
- Two-way sync — Watch is write-only source of truth
- Flutter or Python clients — separate changes
- Advanced statistics (momentum, break points) — belongs in `api-stats` or `python-analysis`
- Admin dashboard or user management UI

## Decisions

### 1. Framework: Axum
**Choice**: Axum 0.8+
**Why**: Native tokio integration, tower middleware ecosystem, strong typing with extractors. Preferred over Actix (less ergonomic) and Rocket (heavier macro usage).

### 2. Database: sqlx with compile-time checked queries
**Choice**: sqlx with offline mode for CI
**Why**: No ORM overhead, queries checked at compile time, supports migrations natively. Preferred over Diesel (heavy macro DSL, schema.rs management).

### 3. Auth: JWT with argon2 password hashing
**Choice**: `jsonwebtoken` crate + `argon2` crate
**Why**: Stateless auth suitable for mobile clients. Access token (short-lived, 1h) + refresh token (long-lived, 30d). Preferred over session-based auth (needs server state) or OAuth (overkill for v1).

### 4. API crate becomes a binary
**Choice**: Convert `tennis-scorer-api` from lib to binary crate (`main.rs` + `lib.rs`)
**Why**: Needs to be an executable server. Keep `lib.rs` for shared types/logic that tests can import.

### 5. Match data format
**Choice**: Store `MatchConfig` as JSONB, point events as a separate table with timestamps
**Why**: JSONB for config is flexible (new fields don't need migrations). Separate point events table enables per-point queries and time-series analysis.

### 6. Configuration
**Choice**: Environment variables via `dotenvy` + typed config struct
**Why**: 12-factor app compatible, works with Docker and Fly.io secrets. No config file needed.

## API Routes

```
POST   /api/auth/register     — Create account (email, password)
POST   /api/auth/login         — Get JWT tokens
POST   /api/auth/refresh       — Refresh access token

GET    /api/matches            — List user's matches (paginated)
POST   /api/matches            — Create match (config + point events)
GET    /api/matches/:id        — Get match detail with all point events
DELETE /api/matches/:id        — Delete a match

GET    /api/stats/summary      — Win/loss record, recent form
GET    /api/health             — Health check (DB connectivity)
```

## Database Schema

```sql
-- users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- matches
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL,
    client_id UUID UNIQUE,  -- idempotency key from Watch
    match_type TEXT NOT NULL DEFAULT 'singles',
    config JSONB NOT NULL,
    winner u8 NOT NULL,  -- 1 or 2
    player1_sets u8 NOT NULL,
    player2_sets u8 NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- match_events (point-by-point)
CREATE TABLE match_events (
    id BIGSERIAL PRIMARY KEY,
    match_id UUID REFERENCES matches(id) ON DELETE CASCADE NOT NULL,
    point_number INT NOT NULL,
    player SMALLINT NOT NULL,  -- 1 or 2
    timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(match_id, point_number)
);
```

## Project Structure

```
crates/tennis-scorer-api/
├── Cargo.toml
├── src/
│   ├── main.rs          — Entry point, server startup
│   ├── lib.rs           — Re-exports for tests
│   ├── config.rs        — App config from env vars
│   ├── db.rs            — Database pool setup
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── handlers.rs  — Register, login, refresh
│   │   ├── jwt.rs       — Token creation/validation
│   │   └── middleware.rs — Auth extractor
│   ├── matches/
│   │   ├── mod.rs
│   │   ├── handlers.rs  — CRUD handlers
│   │   └── models.rs    — Request/response types
│   ├── stats/
│   │   ├── mod.rs
│   │   └── handlers.rs  — Summary stats
│   └── error.rs         — Unified error type → JSON responses
├── migrations/
│   ├── 001_create_users.sql
│   ├── 002_create_matches.sql
│   └── 003_create_match_events.sql
└── .env.example
```

## Risks / Trade-offs

- **[Risk] sqlx compile-time checks require DB connection** → Use `sqlx prepare` to generate offline query data for CI builds
- **[Risk] JWT secret management** → Use Fly.io secrets in prod; `.env` locally; never commit secrets
- **[Risk] Schema evolution** → JSONB for config provides flexibility; sqlx migrations for structural changes
- **[Risk] No rate limiting in v1** → Accept for now; add tower-governor middleware later if needed

## Migration Plan

1. Add dependencies to `tennis-scorer-api/Cargo.toml`
2. Create migration SQL files
3. Implement auth module → matches module → stats module
4. Add Docker Compose for local PostgreSQL
5. Test locally, then configure Fly.io deployment
6. Update `deploy.yml` CI workflow

**Rollback**: API is a new service with no existing users. Rollback = don't deploy.
