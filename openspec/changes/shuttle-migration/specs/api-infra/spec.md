## MODIFIED Requirements

### Requirement: Application configuration from environment
The system SHALL read `JWT_SECRET` from Shuttle's `SecretStore`. The `database_url`, `host`, and `port` are no longer needed as configuration — Shuttle manages these automatically.

#### Scenario: JWT secret available
- **WHEN** the Shuttle runtime provides a `SecretStore`
- **THEN** the system SHALL extract `JWT_SECRET` and use it for token signing/verification

#### Scenario: JWT secret missing
- **WHEN** `JWT_SECRET` is not present in the `SecretStore`
- **THEN** the system SHALL fail to start with a clear error message

### Requirement: Shuttle.rs for local development and deployment
The project SHALL use Shuttle.rs (`cargo shuttle run`) for local development with automatic PostgreSQL provisioning.

#### Scenario: Local development startup
- **WHEN** `cargo shuttle run` is executed in the API crate directory
- **THEN** a PostgreSQL instance SHALL be provisioned automatically and the server SHALL start

### Requirement: Health check endpoint
The system SHALL expose `GET /api/health` (unauthenticated) that verifies database connectivity.

#### Scenario: Database reachable
- **WHEN** the database is connected
- **THEN** the system SHALL return 200 with `{"status": "ok"}`

#### Scenario: Database unreachable
- **WHEN** the database connection fails
- **THEN** the system SHALL return 503 with `{"status": "error"}`

## REMOVED Requirements

### Requirement: Docker Compose for local development
**Reason**: Replaced by Shuttle.rs automatic PostgreSQL provisioning
**Migration**: Use `cargo shuttle run` instead of `docker compose up`
