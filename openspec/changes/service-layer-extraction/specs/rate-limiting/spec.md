## ADDED Requirements

### Requirement: Global API rate limiting
The system SHALL enforce a global rate limit on all API requests using Tower middleware.

#### Scenario: Request within rate limit
- **WHEN** a request arrives and the current request rate is below 100 requests per second
- **THEN** the system SHALL process the request normally

#### Scenario: Request exceeds rate limit
- **WHEN** a request arrives and the current request rate exceeds 100 requests per second
- **THEN** the system SHALL return 429 Too Many Requests without processing the request

### Requirement: Rate limit applies to all routes
The rate limiting middleware SHALL be applied globally to all API routes.

#### Scenario: Rate limit on authenticated routes
- **WHEN** rate limit is exceeded on an authenticated endpoint (e.g., POST /api/matches)
- **THEN** the system SHALL return 429 Too Many Requests

#### Scenario: Rate limit on public routes
- **WHEN** rate limit is exceeded on any public endpoint
- **THEN** the system SHALL return 429 Too Many Requests
