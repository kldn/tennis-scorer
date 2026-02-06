# Backend & CI/CD Design

## Overview

Extend tennis-scorer with a backend API to persist match results, enable statistical analysis, and introduce CI/CD pipelines.

## Architecture

```
┌─────────────────┐     ┌──────────────────────┐     ┌────────────┐
│  Apple Watch     │────▶│  Rust API (Axum)      │────▶│ PostgreSQL │
│  (Swift + FFI)   │ HTTP│                      │     │            │
└─────────────────┘     │  tennis-scorer crate  │     └─────┬──────┘
                        │  (shared types)       │           │
┌─────────────────┐     └──────────────────────┘           │
│  Future iOS App  │────▶                                   │
│  (optional)      │ HTTP                                   ▼
└─────────────────┘                            ┌──────────────────┐
                                               │  Python Analysis  │
                                               │  (pandas, etc.)  │
                                               └──────────────────┘
```

**Key decisions:**
- **API backend:** Rust (Axum) — shares types with core scoring engine
- **Analysis:** Python — leverages pandas/numpy ecosystem for statistics
- **Database:** PostgreSQL — robust, well-supported by both Rust (sqlx) and Python
- **Deployment:** Fly.io (or Railway) — simple deployment, free tier sufficient for early stage
- **Auth:** JWT-based authentication

## Project Structure

```
tennis-scorer/
├── Cargo.toml                  # workspace root
├── crates/
│   ├── tennis-scorer/          # core scoring engine (existing code)
│   │   ├── Cargo.toml
│   │   └── src/
│   └── tennis-scorer-api/      # Axum backend
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── routes/         # API endpoints
│           ├── models/         # DB models (sqlx)
│           ├── auth/           # JWT authentication
│           └── error.rs
├── python/                     # Python analysis service
│   ├── pyproject.toml
│   └── analysis/
├── WatchApp/                   # existing Watch App
├── include/                    # FFI headers
├── migrations/                 # SQL migrations (sqlx)
└── .github/workflows/          # CI/CD
```

## Rust Dependencies

| Crate          | Purpose                              |
|----------------|--------------------------------------|
| `axum`         | Web framework                        |
| `sqlx`         | Async PostgreSQL driver (compile-time SQL verification) |
| `jsonwebtoken` | JWT auth                             |
| `serde`        | Serialization (already in use)       |
| `tokio`        | Async runtime                        |

## API Endpoints

```
POST   /api/auth/register       # Register new user
POST   /api/auth/login          # Login, returns JWT

POST   /api/matches             # Upload match result
GET    /api/matches             # List my matches
GET    /api/matches/:id         # Match detail (with point-by-point record)

GET    /api/stats/summary       # Personal win rate, match count
GET    /api/stats/vs/:player    # Head-to-head record vs specific opponent
GET    /api/stats/trends        # Recent performance trends
```

## Type Sharing Example

The core `tennis-scorer` crate types are directly reusable in the API:

```rust
use tennis_scorer::{MatchConfig, Player, MatchState};

#[derive(Deserialize)]
struct CreateMatch {
    config: MatchConfig,        // from core crate
    points: Vec<Player>,        // point-by-point record
    match_type: MatchType,      // singles or doubles
    players: Vec<PlayerInfo>,   // participants
}
```

## Database Schema

```sql
CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username      VARCHAR(50) UNIQUE NOT NULL,
    email         VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE matches (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id          UUID REFERENCES users(id) NOT NULL,
    match_type       SMALLINT NOT NULL,       -- 1 = singles, 2 = doubles
    config           JSONB NOT NULL,          -- MatchConfig serialized
    winner           SMALLINT,                -- 1 = my side, 2 = opponent side
    final_score      JSONB NOT NULL,          -- { sets: [6-4, 3-6, 7-5], ... }
    points           JSONB NOT NULL,          -- point-by-point: [1, 2, 1, 1, 2, ...]
    duration_seconds INT,
    played_at        TIMESTAMPTZ NOT NULL,
    created_at       TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_matches_user_id ON matches(user_id);
CREATE INDEX idx_matches_played_at ON matches(played_at);

CREATE TABLE match_players (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_id    UUID REFERENCES matches(id) ON DELETE CASCADE NOT NULL,
    team        SMALLINT NOT NULL,           -- 1 = my side, 2 = opponent side
    player_name VARCHAR(100) NOT NULL,
    is_self     BOOLEAN DEFAULT false
);

CREATE INDEX idx_match_players_match ON match_players(match_id);
CREATE INDEX idx_match_players_name ON match_players(player_name);
```

**Design notes:**
- Singles: 1 player per team. Doubles: 2 players per team.
- `config` stored as JSONB — directly serialized from Rust `MatchConfig`
- `points` stores full point-by-point record — enables Python to replay matches for deep analysis (serve game win %, break points, etc.)
- `final_score` stored separately for fast queries without replaying points

## CI/CD (GitHub Actions)

### 1. Rust CI (on every push/PR)

```yaml
# .github/workflows/rust.yml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test
- cargo build --release
```

### 2. watchOS Build (on tag or manual trigger)

```yaml
# .github/workflows/watchos.yml
- Run build-watchos.sh
- Verify FFI header is up-to-date (cbindgen diff check)
```

### 3. API Deploy (push to main)

```yaml
# .github/workflows/deploy.yml
- Run tests
- Build Docker image
- Push to container registry
- Deploy to Fly.io / Railway
```

## Implementation Phases

### Phase 1: Project restructuring
- Convert to Cargo workspace
- Move existing code to `crates/tennis-scorer/`
- Ensure Watch App still builds

### Phase 2: API backend
- Set up `tennis-scorer-api` crate with Axum
- Implement auth (register/login)
- Implement match CRUD endpoints
- Database migrations with sqlx

### Phase 3: Watch App integration
- Add HTTP client to Watch App
- Upload completed matches to backend
- Handle offline/online sync

### Phase 4: CI/CD
- GitHub Actions for Rust CI
- watchOS build verification
- API deployment pipeline

### Phase 5: Python analysis
- Set up Python project
- Connect to PostgreSQL
- Implement stats endpoints or reports

### Phase 6: Doubles support
- Extend core engine with serve rotation tracking
- Update API and schema (already designed for doubles)
- Update Watch App UI for doubles mode
