## ADDED Requirements

### Requirement: Apple Sign-In API endpoint
The API SHALL provide `POST /api/auth/apple` that accepts an Apple identity token, verifies it, and returns JWT tokens.

#### Scenario: New Apple user signs in
- **WHEN** a valid Apple identity token is submitted and no user with that Apple user ID exists
- **THEN** the system SHALL create a new user record with the Apple user ID (and email if provided), and return `access_token` and `refresh_token` in the response body with status 200

#### Scenario: Existing Apple user signs in
- **WHEN** a valid Apple identity token is submitted and a user with that Apple user ID already exists
- **THEN** the system SHALL return `access_token` and `refresh_token` for the existing user with status 200

#### Scenario: Invalid identity token
- **WHEN** an invalid, expired, or malformed Apple identity token is submitted
- **THEN** the system SHALL return 401 Unauthorized

### Requirement: Apple identity token verification via JWKS
The API SHALL verify Apple identity tokens using Apple's public JWKS endpoint (`https://appleid.apple.com/auth/keys`).

#### Scenario: Token signature verification
- **WHEN** an Apple identity token is received
- **THEN** the system SHALL fetch (or use cached) Apple public keys and verify the token signature, issuer (`https://appleid.apple.com`), and audience (app bundle ID)

#### Scenario: JWKS cache refresh
- **WHEN** token verification fails with a cached key set
- **THEN** the system SHALL refresh the JWKS from Apple and retry verification once

### Requirement: Users table supports Apple authentication
The `users` table SHALL support Apple-authenticated users with an `apple_user_id` column.

#### Scenario: Schema migration
- **WHEN** the migration runs
- **THEN** the system SHALL add `apple_user_id TEXT UNIQUE` to the `users` table and make `email` and `password_hash` nullable
