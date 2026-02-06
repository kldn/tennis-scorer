## ADDED Requirements

### Requirement: Database migrations manage schema
The system SHALL use sqlx migrations to create and manage the PostgreSQL schema (users, matches, match_events tables).

#### Scenario: Fresh database setup
- **WHEN** `sqlx migrate run` is executed against an empty database
- **THEN** all tables (users, matches, match_events) SHALL be created with correct columns, types, and constraints

#### Scenario: Migrations are idempotent
- **WHEN** `sqlx migrate run` is executed against an already-migrated database
- **THEN** no errors SHALL occur and no data SHALL be lost

### Requirement: Application configuration from environment
The system SHALL read all configuration from environment variables: `DATABASE_URL`, `JWT_SECRET`, `HOST`, `PORT`.

#### Scenario: All variables set
- **WHEN** all required environment variables are present
- **THEN** the server SHALL start successfully

#### Scenario: Missing required variable
- **WHEN** `DATABASE_URL` or `JWT_SECRET` is not set
- **THEN** the server SHALL exit with a clear error message indicating the missing variable

### Requirement: Health check endpoint
The system SHALL expose `GET /api/health` (unauthenticated) that verifies database connectivity.

#### Scenario: Database reachable
- **WHEN** the database is connected
- **THEN** the system SHALL return 200 with `{"status": "ok"}`

#### Scenario: Database unreachable
- **WHEN** the database connection fails
- **THEN** the system SHALL return 503 with `{"status": "error"}`

### Requirement: Docker Compose for local development
The project SHALL include a `docker-compose.yml` that starts PostgreSQL with a preconfigured database.

#### Scenario: Local development startup
- **WHEN** `docker compose up -d` is run
- **THEN** a PostgreSQL instance SHALL be available on localhost:5432 with the configured database name

### Requirement: CORS configuration
The system SHALL enable CORS headers to allow requests from web frontends.

#### Scenario: Cross-origin request
- **WHEN** a request arrives with an `Origin` header
- **THEN** the system SHALL include appropriate `Access-Control-Allow-*` headers in the response
