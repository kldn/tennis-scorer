## ADDED Requirements

### Requirement: User registration with email and password
The system SHALL allow new users to register by providing an email and password via `POST /api/auth/register`.

#### Scenario: Successful registration
- **WHEN** a valid email and password (min 8 characters) are submitted
- **THEN** the system SHALL create a user record with argon2-hashed password and return a 201 response with user ID

#### Scenario: Duplicate email
- **WHEN** a registration request uses an email that already exists
- **THEN** the system SHALL return a 409 Conflict error

#### Scenario: Invalid input
- **WHEN** the email is malformed or password is shorter than 8 characters
- **THEN** the system SHALL return a 422 Unprocessable Entity error with field-level messages

### Requirement: User login returns JWT tokens
The system SHALL authenticate users via `POST /api/auth/login` and return access + refresh tokens.

#### Scenario: Successful login
- **WHEN** valid email and password are submitted
- **THEN** the system SHALL return a JSON body with `access_token` (1h expiry) and `refresh_token` (30d expiry)

#### Scenario: Invalid credentials
- **WHEN** email does not exist or password is wrong
- **THEN** the system SHALL return a 401 Unauthorized error (without revealing which field was wrong)

### Requirement: Token refresh
The system SHALL allow refreshing an expired access token via `POST /api/auth/refresh`.

#### Scenario: Valid refresh token
- **WHEN** a valid, non-expired refresh token is submitted
- **THEN** the system SHALL return a new access token

#### Scenario: Expired or invalid refresh token
- **WHEN** an expired or malformed refresh token is submitted
- **THEN** the system SHALL return a 401 Unauthorized error

### Requirement: Auth middleware extracts user from JWT
The system SHALL provide an Axum extractor that validates the `Authorization: Bearer <token>` header and injects the authenticated user ID into handlers.

#### Scenario: Valid access token
- **WHEN** a request includes a valid Bearer token in the Authorization header
- **THEN** the handler SHALL receive the authenticated user's UUID

#### Scenario: Missing or invalid token
- **WHEN** the Authorization header is missing, malformed, or contains an expired token
- **THEN** the system SHALL return a 401 Unauthorized error before reaching the handler
