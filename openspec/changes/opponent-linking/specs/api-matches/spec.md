## MODIFIED Requirements

### Requirement: Create a match record
The system SHALL accept completed match data via `POST /api/matches` (authenticated), including optional opponent information.

#### Scenario: Successful match creation
- **WHEN** an authenticated user submits match config, final score, and point events array
- **THEN** the system SHALL store the match and all point events, returning 201 with the match ID

#### Scenario: Match creation with opponent info
- **WHEN** an authenticated user submits a match with `opponent_email` and/or `opponent_name`
- **THEN** the system SHALL store the opponent fields, generate a `opponent_claim_token` (UUID v4), and if opponent_email matches a registered user's email, automatically set `opponent_user_id`

#### Scenario: Match creation without opponent info
- **WHEN** an authenticated user submits a match without opponent_email or opponent_name
- **THEN** the system SHALL create the match with all opponent fields as NULL

#### Scenario: Idempotent creation via client_id
- **WHEN** a request includes a `client_id` (UUID) that already exists in the database
- **THEN** the system SHALL return 200 with the existing match ID (not create a duplicate)

#### Scenario: Unauthenticated request
- **WHEN** the request has no valid auth token
- **THEN** the system SHALL return 401 Unauthorized

### Requirement: List user matches with pagination
The system SHALL return the authenticated user's matches via `GET /api/matches`, including opponent information.

#### Scenario: Default listing
- **WHEN** an authenticated user requests their matches without query parameters
- **THEN** the system SHALL return the most recent 20 matches ordered by `started_at` descending, each including opponent_user_id, opponent_email, opponent_name fields

#### Scenario: Paginated listing
- **WHEN** the request includes `?limit=10&offset=20`
- **THEN** the system SHALL return up to 10 matches starting from offset 20, plus a `total` count

#### Scenario: No matches
- **WHEN** the user has no matches
- **THEN** the system SHALL return an empty array with `total: 0`

### Requirement: Get match detail with point events
The system SHALL return a single match with all point events and opponent information via `GET /api/matches/:id`.

#### Scenario: Match belongs to user
- **WHEN** an authenticated user requests a match they own
- **THEN** the system SHALL return match metadata, config, final score, opponent info (opponent_user_id, opponent_email, opponent_name, opponent_claim_token), and all point events ordered by point_number

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

## ADDED Requirements

### Requirement: Claim a match as opponent
The system SHALL allow a user to claim a match record as the opponent via `POST /api/matches/claim`.

#### Scenario: Successful claim
- **WHEN** an authenticated user submits a valid `opponent_claim_token` and the match has no `opponent_user_id` set
- **THEN** the system SHALL set `opponent_user_id` to the current user's ID, clear the `opponent_claim_token`, and return the match info with status 200

#### Scenario: Already claimed
- **WHEN** an authenticated user submits a claim token for a match that already has an `opponent_user_id`
- **THEN** the system SHALL return 409 Conflict

#### Scenario: Invalid claim token
- **WHEN** an authenticated user submits a claim token that does not exist
- **THEN** the system SHALL return 404 Not Found

#### Scenario: Claiming own match
- **WHEN** an authenticated user submits a claim token for a match they created (user_id matches)
- **THEN** the system SHALL return 400 Bad Request (cannot be opponent of own match)

### Requirement: Auto-match opponent on user creation
The system SHALL automatically link matches to a newly registered user if their email matches `opponent_email` on existing matches.

#### Scenario: Email matches existing matches
- **WHEN** a new user is created via `GET /api/auth/me` with an email that matches `opponent_email` on one or more matches where `opponent_user_id` IS NULL
- **THEN** the system SHALL update all matching records to set `opponent_user_id` to the new user's ID

#### Scenario: No matching matches
- **WHEN** a new user is created and no matches have a matching `opponent_email`
- **THEN** the system SHALL proceed normally without any updates
