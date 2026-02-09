## MODIFIED Requirements

### Requirement: Application configuration from environment
The system SHALL read all configuration from environment variables: `DATABASE_URL`, `JWT_SECRET`, `HOST`, `PORT`. For local development, the system SHALL load variables from a `.env` file using `dotenvy` if present.

#### Scenario: All variables set
- **WHEN** all required environment variables are present
- **THEN** the server SHALL start successfully

#### Scenario: Missing required variable
- **WHEN** `DATABASE_URL` or `JWT_SECRET` is not set
- **THEN** the server SHALL exit with a clear error message indicating the missing variable

#### Scenario: Local development with .env file
- **WHEN** a `.env` file exists in the working directory
- **THEN** the system SHALL load variables from it before reading environment

### Requirement: Standard tokio runtime entry point
The system SHALL use `#[tokio::main]` as the application entry point, creating a PgPool from `DATABASE_URL`, running migrations, and starting the Axum server on `HOST:PORT`.

#### Scenario: Server startup
- **WHEN** the binary is executed with valid environment variables
- **THEN** the system SHALL connect to Postgres, run pending migrations, and listen on the configured host and port

#### Scenario: Default host and port
- **WHEN** `HOST` and `PORT` are not set
- **THEN** the system SHALL default to `0.0.0.0:8000`

## REMOVED Requirements

### Requirement: Shuttle.rs for local development and deployment
**Reason**: Shuttle.rs is shutting down. Replaced by standard tokio runtime with Railway deployment.
**Migration**: Use `cargo run` with a `.env` file for local development. Deploy via Railway CLI or GitHub Actions.
