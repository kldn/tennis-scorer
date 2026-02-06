## 1. Project Setup & Infrastructure

- [x] 1.1 Update `crates/tennis-scorer-api/Cargo.toml` with dependencies (axum, sqlx, tokio, jsonwebtoken, argon2, serde, tower-http, dotenvy, uuid)
- [x] 1.2 Create `src/main.rs` entry point with tokio runtime and Axum server startup
- [x] 1.3 Create `src/config.rs` for typed env-var configuration (DATABASE_URL, JWT_SECRET, HOST, PORT)
- [x] 1.4 Create `src/db.rs` for sqlx PgPool setup
- [x] 1.5 Create `src/error.rs` unified error type with IntoResponse impl
- [x] 1.6 Create `docker-compose.yml` with PostgreSQL service
- [x] 1.7 Create `.env.example` with all required variables

## 2. Database Migrations

- [x] 2.1 Create migration 001: users table (id UUID, email, password_hash, created_at)
- [x] 2.2 Create migration 002: matches table (id, user_id, client_id, match_type, config JSONB, winner, sets, timestamps)
- [x] 2.3 Create migration 003: match_events table (id, match_id, point_number, player, timestamp)

## 3. Authentication

- [x] 3.1 Create `src/auth/jwt.rs` — token creation (access 1h, refresh 30d) and validation
- [x] 3.2 Create `src/auth/handlers.rs` — register endpoint (argon2 hash, 201/409/422)
- [x] 3.3 Create `src/auth/handlers.rs` — login endpoint (verify password, return tokens)
- [x] 3.4 Create `src/auth/handlers.rs` — refresh endpoint (validate refresh token, return new access token)
- [x] 3.5 Create `src/auth/middleware.rs` — AuthUser extractor from Bearer token

## 4. Match Endpoints

- [x] 4.1 Create `src/matches/models.rs` — request/response types (CreateMatch, MatchResponse, MatchListResponse)
- [x] 4.2 Create `src/matches/handlers.rs` — POST /api/matches (create with idempotency via client_id)
- [x] 4.3 Create `src/matches/handlers.rs` — GET /api/matches (list with pagination: limit, offset, total)
- [x] 4.4 Create `src/matches/handlers.rs` — GET /api/matches/:id (detail with point events)
- [x] 4.5 Create `src/matches/handlers.rs` — DELETE /api/matches/:id (cascade delete)

## 5. Statistics

- [x] 5.1 Create `src/stats/handlers.rs` — GET /api/stats/summary (wins, losses, win rate, streak, recent form)

## 6. Health & CORS

- [x] 6.1 Add GET /api/health endpoint (DB connectivity check)
- [x] 6.2 Add tower-http CORS layer to router

## 7. Router Assembly & Integration

- [x] 7.1 Wire all routes in main.rs with proper middleware (auth on protected routes)
- [x] 7.2 Create `src/lib.rs` re-exporting app builder for test use
- [x] 7.3 Add integration tests (auth flow, match CRUD, stats)

## 8. Deployment Prep

- [x] 8.1 Create `Dockerfile` for the API crate
- [x] 8.2 Create `fly.toml` configuration
- [x] 8.3 ~~Run `sqlx prepare`~~ N/A — using string queries (not `query!` macro), no offline metadata needed
