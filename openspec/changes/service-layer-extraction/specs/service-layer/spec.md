## ADDED Requirements

### Requirement: AuthService encapsulates authentication logic
The system SHALL provide an `AuthService` struct that encapsulates all authentication-related business logic, separate from HTTP handler concerns.

#### Scenario: AuthService initialization
- **WHEN** the application starts
- **THEN** the system SHALL create an `AuthService` instance holding a `PgPool` reference and register it in `AppState`

#### Scenario: AuthService handles user lookup/creation
- **WHEN** a handler calls `AuthService::get_or_create_user(firebase_uid, email, display_name, avatar_url)`
- **THEN** the service SHALL find or create the user and return the user record, without any HTTP-specific logic

### Requirement: MatchService encapsulates match logic
The system SHALL provide a `MatchService` struct that encapsulates all match-related business logic.

#### Scenario: MatchService initialization
- **WHEN** the application starts
- **THEN** the system SHALL create a `MatchService` instance holding a `PgPool` reference and register it in `AppState`

#### Scenario: MatchService create
- **WHEN** a handler calls `MatchService::create(user_id, request)`
- **THEN** the service SHALL handle the database transaction (insert match + events), client_id idempotency check, and return the created match

#### Scenario: MatchService list
- **WHEN** a handler calls `MatchService::list(user_id, limit, offset)`
- **THEN** the service SHALL query the database and return paginated matches with total count

#### Scenario: MatchService get
- **WHEN** a handler calls `MatchService::get(user_id, match_id)`
- **THEN** the service SHALL return the match with point events if owned by the user, or an error if not found

#### Scenario: MatchService delete
- **WHEN** a handler calls `MatchService::delete(user_id, match_id)`
- **THEN** the service SHALL delete the match and associated events if owned by the user

### Requirement: StatsService encapsulates statistics logic
The system SHALL provide a `StatsService` struct that encapsulates all statistics calculation logic.

#### Scenario: StatsService initialization
- **WHEN** the application starts
- **THEN** the system SHALL create a `StatsService` instance holding a `PgPool` reference and register it in `AppState`

#### Scenario: StatsService summary
- **WHEN** a handler calls `StatsService::summary(user_id)`
- **THEN** the service SHALL calculate and return the user's overall statistics

#### Scenario: StatsService match_analysis
- **WHEN** a handler calls `StatsService::match_analysis(user_id, match_id)`
- **THEN** the service SHALL return detailed analysis for the specified match

#### Scenario: StatsService match_momentum
- **WHEN** a handler calls `StatsService::match_momentum(user_id, match_id)`
- **THEN** the service SHALL return point-by-point momentum data for the specified match

#### Scenario: StatsService match_pace
- **WHEN** a handler calls `StatsService::match_pace(user_id, match_id)`
- **THEN** the service SHALL return pace/timing analysis for the specified match

### Requirement: Handlers are thin HTTP adapters
After service extraction, handlers SHALL only parse HTTP requests, call the appropriate service method, and map service results to HTTP responses.

#### Scenario: Handler delegates to service
- **WHEN** an HTTP request reaches a handler
- **THEN** the handler SHALL extract request parameters, call the corresponding service method, and return the result as an HTTP response

#### Scenario: Service error mapping
- **WHEN** a service method returns an error
- **THEN** the handler SHALL map the service error to the appropriate HTTP status code (e.g., NotFound → 404, Conflict → 409)

### Requirement: External API behavior unchanged after extraction
All existing API endpoints SHALL maintain identical external behavior after service layer extraction.

#### Scenario: All integration tests pass
- **WHEN** the service layer extraction is complete
- **THEN** all existing integration tests SHALL pass without modification (except import changes)
