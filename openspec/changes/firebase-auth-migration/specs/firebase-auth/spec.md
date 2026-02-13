## ADDED Requirements

### Requirement: Firebase ID Token verification via Google JWKS
The system SHALL verify Firebase ID Tokens by fetching Google's public JWKS and validating the RS256 JWT signature locally.

#### Scenario: Valid Firebase ID Token
- **WHEN** a valid, non-expired Firebase ID Token is received
- **THEN** the system SHALL verify the signature using cached Google JWKS public keys, validate the issuer is `https://securetoken.google.com/<project_id>`, validate the audience matches `FIREBASE_PROJECT_ID`, and return the decoded claims (sub, email, name, picture)

#### Scenario: Expired Firebase ID Token
- **WHEN** an expired Firebase ID Token is received
- **THEN** the system SHALL return a 401 Unauthorized error

#### Scenario: Invalid signature
- **WHEN** a Firebase ID Token with an invalid or tampered signature is received
- **THEN** the system SHALL return a 401 Unauthorized error

#### Scenario: Wrong issuer or audience
- **WHEN** a Firebase ID Token has an incorrect issuer or audience claim
- **THEN** the system SHALL return a 401 Unauthorized error

### Requirement: JWKS public key caching
The system SHALL cache Google JWKS public keys to avoid fetching them on every request.

#### Scenario: First request fetches JWKS
- **WHEN** no cached JWKS keys exist
- **THEN** the system SHALL fetch keys from `https://www.googleapis.com/service_account/v1/metadata/x509/securetoken@system.gserviceaccount.com` and cache them

#### Scenario: Cache refresh after TTL
- **WHEN** the cached JWKS keys are older than 1 hour
- **THEN** the system SHALL fetch fresh keys from Google and update the cache

#### Scenario: Verification failure triggers cache refresh
- **WHEN** token verification fails with cached keys and the cache is older than 5 minutes
- **THEN** the system SHALL refresh the JWKS cache and retry verification once

### Requirement: FirebaseClaims extraction
The system SHALL extract user information from verified Firebase ID Token claims.

#### Scenario: Claims with all fields
- **WHEN** a verified token contains sub, email, name, and picture claims
- **THEN** the system SHALL return a FirebaseClaims struct with firebase_uid (from sub), email, display_name (from name), and avatar_url (from picture)

#### Scenario: Claims with minimal fields
- **WHEN** a verified token contains only sub and email (name and picture are absent)
- **THEN** the system SHALL return FirebaseClaims with display_name and avatar_url as None

### Requirement: Users table supports Firebase authentication
The `users` table SHALL store Firebase-specific user attributes.

#### Scenario: Migration adds Firebase columns
- **WHEN** the migration runs
- **THEN** the system SHALL add `firebase_uid TEXT UNIQUE`, `display_name TEXT`, and `avatar_url TEXT` columns to the users table

#### Scenario: Migration removes password requirement
- **WHEN** the migration runs
- **THEN** the system SHALL alter `password_hash` to be nullable (DROP NOT NULL)

### Requirement: Get or create user from Firebase token
The system SHALL provide a `GET /api/auth/me` endpoint that finds or creates a user based on the Firebase ID Token.

#### Scenario: Existing user by firebase_uid
- **WHEN** an authenticated request hits `GET /api/auth/me` and a user with the token's firebase_uid exists
- **THEN** the system SHALL return the existing user's data (id, email, display_name, avatar_url) with status 200

#### Scenario: New user creation
- **WHEN** an authenticated request hits `GET /api/auth/me` and no user with the token's firebase_uid exists
- **THEN** the system SHALL create a new user record with firebase_uid, email, display_name, and avatar_url from the token claims, and return the user data with status 200

#### Scenario: User data update on login
- **WHEN** an existing user's display_name or avatar_url in the token differs from the stored values
- **THEN** the system SHALL update the stored display_name and avatar_url to match the token claims
