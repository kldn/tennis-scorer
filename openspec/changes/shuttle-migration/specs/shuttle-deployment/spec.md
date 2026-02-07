## ADDED Requirements

### Requirement: Shuttle.rs entry point
The API binary SHALL use `#[shuttle_runtime::main]` as its entry point, receiving a `PgPool` from `#[shuttle_shared_db::Postgres]` and a `SecretStore` from `#[shuttle_runtime::Secrets]`.

#### Scenario: Server starts via Shuttle runtime
- **WHEN** `cargo shuttle run` is executed in the API crate directory
- **THEN** the server SHALL start with an auto-provisioned PostgreSQL database and serve the existing API routes

#### Scenario: Secrets are available
- **WHEN** the Shuttle runtime starts
- **THEN** `JWT_SECRET` SHALL be read from the `SecretStore` and passed to the router

### Requirement: Shuttle dependencies in Cargo.toml
The API crate SHALL depend on `shuttle-axum`, `shuttle-shared-db` (with postgres feature), and `shuttle-runtime`.

#### Scenario: Build with Shuttle deps
- **WHEN** `cargo build -p tennis-scorer-api` is executed
- **THEN** the build SHALL succeed with Shuttle dependencies resolved

### Requirement: dotenvy removal
The API crate SHALL NOT depend on `dotenvy`. Configuration SHALL come from Shuttle's `SecretStore` instead of environment variables.

#### Scenario: No dotenvy in dependency tree
- **WHEN** the API crate's Cargo.toml is inspected
- **THEN** `dotenvy` SHALL NOT be listed as a dependency

### Requirement: Secrets.toml for local development
A `Secrets.toml` file SHALL exist locally (gitignored) containing `JWT_SECRET` for local development with `cargo shuttle run`.

#### Scenario: Local development secrets
- **WHEN** `cargo shuttle run` is executed with a valid `Secrets.toml`
- **THEN** the API SHALL read `JWT_SECRET` from the secrets file

### Requirement: Watch app base URL update
The Watch app's `APIClient.swift` SHALL use the Shuttle.rs deployment URL instead of the Fly.io URL.

#### Scenario: API requests target Shuttle deployment
- **WHEN** the Watch app makes an API request
- **THEN** the request SHALL be sent to `https://tennis-scorer-api.shuttle.app/api`
