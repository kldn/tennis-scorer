## MODIFIED Requirements

### Requirement: User registration with email and password
The system SHALL accept POST /api/auth/register with email and password.
The system SHALL validate email format using regex pattern `^[^\s@]+@[^\s@]+\.[^\s@]+$`.
The system SHALL require password length of at least 8 characters.
The system SHALL return 422 Unprocessable Entity with "Invalid email format" if email validation fails.

#### Scenario: Registration with valid email
- **WHEN** user sends POST /api/auth/register with email "user@example.com" and password "securepass"
- **THEN** system creates account and returns 201

#### Scenario: Registration with invalid email format
- **WHEN** user sends POST /api/auth/register with email "notanemail" and password "securepass"
- **THEN** system returns 422 with message "Invalid email format"

#### Scenario: Registration with email missing domain
- **WHEN** user sends POST /api/auth/register with email "user@" and password "securepass"
- **THEN** system returns 422 with message "Invalid email format"
