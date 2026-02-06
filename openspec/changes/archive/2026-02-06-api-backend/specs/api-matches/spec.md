## ADDED Requirements

### Requirement: Create a match record
The system SHALL accept completed match data via `POST /api/matches` (authenticated).

#### Scenario: Successful match creation
- **WHEN** an authenticated user submits match config, final score, and point events array
- **THEN** the system SHALL store the match and all point events, returning 201 with the match ID

#### Scenario: Idempotent creation via client_id
- **WHEN** a request includes a `client_id` (UUID) that already exists in the database
- **THEN** the system SHALL return 200 with the existing match ID (not create a duplicate)

#### Scenario: Unauthenticated request
- **WHEN** the request has no valid auth token
- **THEN** the system SHALL return 401 Unauthorized

### Requirement: List user matches with pagination
The system SHALL return the authenticated user's matches via `GET /api/matches`.

#### Scenario: Default listing
- **WHEN** an authenticated user requests their matches without query parameters
- **THEN** the system SHALL return the most recent 20 matches ordered by `started_at` descending

#### Scenario: Paginated listing
- **WHEN** the request includes `?limit=10&offset=20`
- **THEN** the system SHALL return up to 10 matches starting from offset 20, plus a `total` count

#### Scenario: No matches
- **WHEN** the user has no matches
- **THEN** the system SHALL return an empty array with `total: 0`

### Requirement: Get match detail with point events
The system SHALL return a single match with all point events via `GET /api/matches/:id`.

#### Scenario: Match belongs to user
- **WHEN** an authenticated user requests a match they own
- **THEN** the system SHALL return match metadata, config, final score, and all point events ordered by point_number

#### Scenario: Match not found or not owned
- **WHEN** the match ID does not exist or belongs to another user
- **THEN** the system SHALL return 404 Not Found

### Requirement: Delete a match
The system SHALL allow deleting a match via `DELETE /api/matches/:id`.

#### Scenario: Successful deletion
- **WHEN** an authenticated user deletes a match they own
- **THEN** the system SHALL delete the match and all associated point events, returning 204

#### Scenario: Not owned
- **WHEN** the user tries to delete a match they don't own
- **THEN** the system SHALL return 404 Not Found
