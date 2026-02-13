## REMOVED Requirements

### Requirement: User registration with email and password
**Reason**: Replaced by Firebase Auth — user accounts are created via Firebase, not custom registration.
**Migration**: Use Firebase Auth SDK on client side; backend receives Firebase ID Token via `GET /api/auth/me`.

### Requirement: User login returns JWT tokens
**Reason**: Replaced by Firebase Auth — login is handled by Firebase SDK on client; backend validates Firebase ID Token.
**Migration**: Client authenticates via Firebase SDK, sends Firebase ID Token in Authorization header.

### Requirement: Token refresh
**Reason**: Replaced by Firebase Auth — token refresh is handled by Firebase SDK on client side.
**Migration**: Firebase SDK automatically refreshes ID Tokens; no backend refresh endpoint needed.

## MODIFIED Requirements

### Requirement: Auth middleware extracts user from JWT
The system SHALL provide an Axum extractor that validates the `Authorization: Bearer <token>` header containing a Firebase ID Token and injects the authenticated user ID into handlers.

#### Scenario: Valid Firebase ID Token
- **WHEN** a request includes a valid Firebase ID Token in the Authorization header
- **THEN** the handler SHALL receive the authenticated user's UUID (looked up via firebase_uid from token claims)

#### Scenario: Missing or invalid token
- **WHEN** the Authorization header is missing, malformed, or contains an expired/invalid Firebase ID Token
- **THEN** the system SHALL return a 401 Unauthorized error before reaching the handler

#### Scenario: Valid token but user not found
- **WHEN** the Authorization header contains a valid Firebase ID Token but no user exists with the corresponding firebase_uid
- **THEN** the system SHALL return a 401 Unauthorized error (user must call GET /api/auth/me first)
