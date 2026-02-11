## MODIFIED Requirements

### Requirement: CORS configuration
The system SHALL read allowed origins from the `ALLOWED_ORIGINS` environment variable as a comma-separated list.
When `ALLOWED_ORIGINS` is set, the system SHALL only allow requests from those specific origins.
When `ALLOWED_ORIGINS` is not set, the system SHALL fall back to allowing any origin (development mode).

#### Scenario: CORS with configured origins
- **WHEN** `ALLOWED_ORIGINS` is set to "https://app.example.com,https://staging.example.com"
- **THEN** only requests from those two origins are allowed
- **AND** requests from other origins receive CORS rejection

#### Scenario: CORS fallback for development
- **WHEN** `ALLOWED_ORIGINS` is not set
- **THEN** the system allows any origin (equivalent to current wildcard behavior)

### Requirement: Application configuration from environment
The system SHALL read configuration from environment variables: DATABASE_URL, JWT_SECRET, HOST, PORT, and ALLOWED_ORIGINS.
ALLOWED_ORIGINS is optional and defaults to permissive mode when unset.

#### Scenario: Configuration with ALLOWED_ORIGINS
- **WHEN** the environment includes ALLOWED_ORIGINS="https://app.example.com"
- **THEN** AppConfig.allowed_origins contains ["https://app.example.com"]

#### Scenario: Configuration without ALLOWED_ORIGINS
- **WHEN** the environment does not include ALLOWED_ORIGINS
- **THEN** AppConfig.allowed_origins is empty and CORS defaults to Any
