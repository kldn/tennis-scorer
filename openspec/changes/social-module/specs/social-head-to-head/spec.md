## ADDED Requirements

### Requirement: Head-to-head statistics
The system SHALL provide head-to-head statistics between two friends via `GET /api/social/head-to-head/{id}`.

#### Scenario: Friends with match history
- **WHEN** an authenticated user requests head-to-head stats with a friend who they have played matches against (via opponent_user_id)
- **THEN** the system SHALL return wins, losses, total_matches, and an array of recent_matches

#### Scenario: Friends with no shared matches
- **WHEN** an authenticated user requests head-to-head stats with a friend but no matches have opponent_user_id linking them
- **THEN** the system SHALL return wins: 0, losses: 0, total_matches: 0, recent_matches: []

#### Scenario: Not a friend
- **WHEN** an authenticated user requests head-to-head stats with someone who is NOT their friend
- **THEN** the system SHALL return 403 Forbidden

#### Scenario: Head-to-head calculation logic
- **WHEN** calculating head-to-head statistics
- **THEN** the system SHALL count matches where (user_id = current_user AND opponent_user_id = friend) as the current user's matches, and determine wins/losses based on match results
