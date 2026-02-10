## MODIFIED Requirements

### Requirement: Create a match record
The system SHALL accept POST /api/matches with authenticated user context.
When `client_id` is provided, the system SHALL check for existing match scoped to the authenticated user (`WHERE client_id = $1 AND user_id = $2`).
If a match with the same `client_id` exists for the same user, the system SHALL return the existing match ID (idempotent).

#### Scenario: Idempotent creation scoped to user
- **WHEN** User A creates a match with client_id "abc-123"
- **AND** User B creates a match with the same client_id "abc-123"
- **THEN** both users get their own separate match records
- **AND** User A's idempotency check does not return User B's match

#### Scenario: Idempotent creation same user
- **WHEN** User A creates a match with client_id "abc-123" twice
- **THEN** the second request returns the same match ID as the first

## ADDED Requirements

### Requirement: Debug endpoint restricted to debug builds
The debug endpoint POST /debug/matches SHALL only be available in debug builds (`#[cfg(debug_assertions)]`).
The endpoint and its route registration SHALL be excluded from release builds via conditional compilation.

#### Scenario: Debug endpoint available in debug build
- **WHEN** the API is compiled with debug assertions (default `cargo run`)
- **THEN** POST /debug/matches is available and creates a match without authentication

#### Scenario: Debug endpoint absent in release build
- **WHEN** the API is compiled in release mode (`cargo build --release`)
- **THEN** POST /debug/matches returns 404 Not Found
