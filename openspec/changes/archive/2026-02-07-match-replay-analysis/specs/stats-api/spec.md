## ADDED Requirements

### Requirement: Match analysis endpoint
The system SHALL provide `GET /api/stats/match/:id/analysis` (authenticated) that returns the full `MatchAnalysis` for a specific match.

#### Scenario: Valid match with point events
- **WHEN** an authenticated user requests analysis for a match they own that has point events stored
- **THEN** the system SHALL load the match config and point events from the database, call `replay_with_context`, compute analysis, and return JSON containing all break point, service, deuce, conversion, streak, clutch, tiebreak, and total point stats

#### Scenario: Match not found
- **WHEN** the match ID does not exist
- **THEN** the system SHALL return HTTP 404

#### Scenario: Match belongs to another user
- **WHEN** an authenticated user requests analysis for a match they do not own
- **THEN** the system SHALL return HTTP 404

#### Scenario: Match with no point events
- **WHEN** a match exists but has no stored point events (e.g., manually entered result only)
- **THEN** the system SHALL return HTTP 200 with zeroed statistics

### Requirement: Match momentum endpoint
The system SHALL provide `GET /api/stats/match/:id/momentum` (authenticated) that returns the `MomentumData` for a specific match.

#### Scenario: Valid match momentum
- **WHEN** an authenticated user requests momentum for their match
- **THEN** the system SHALL return JSON with `basic`, `weighted`, `per_set_basic`, and `per_set_weighted` arrays

#### Scenario: Match not found or not owned
- **WHEN** the match ID does not exist or belongs to another user
- **THEN** the system SHALL return HTTP 404

### Requirement: Match pace endpoint
The system SHALL provide `GET /api/stats/match/:id/pace` (authenticated) that returns timing/pace data.

The response SHALL include:
- `average_point_interval_seconds: f64` — mean seconds between consecutive points
- `per_game_durations: Vec<GameDuration>` — each game's duration in seconds, set number, game number
- `per_set_durations: Vec<SetDuration>` — each set's total duration in seconds
- `total_duration_seconds: f64` — match total duration (first to last point timestamp)
- `point_intervals: Vec<f64>` — interval in seconds between each consecutive pair of points

#### Scenario: Valid match pace data
- **WHEN** an authenticated user requests pace for their match with 100 points
- **THEN** `point_intervals` SHALL have 99 entries, and `average_point_interval_seconds` SHALL be their mean

#### Scenario: Match with single point
- **WHEN** a match has only 1 point event
- **THEN** `point_intervals` SHALL be empty, `average_point_interval_seconds = 0.0`, `total_duration_seconds = 0.0`

#### Scenario: Match not found or not owned
- **WHEN** the match ID does not exist or belongs to another user
- **THEN** the system SHALL return HTTP 404

### Requirement: API routes registered under stats module
The 3 new endpoints SHALL be registered in the existing stats router alongside the existing `/api/stats/summary` endpoint.

#### Scenario: Routes are accessible
- **WHEN** the API server starts
- **THEN** all 3 new routes SHALL be registered and respond to GET requests with valid auth tokens

## MODIFIED Requirements

### Requirement: Summary statistics for authenticated user
The system SHALL return aggregated match statistics via `GET /api/stats/summary` (authenticated).

#### Scenario: User with match history
- **WHEN** an authenticated user requests their summary
- **THEN** the system SHALL return: total matches played, wins, losses, win rate (percentage), current streak (type and count), and recent form (last 10 results as array of "W"/"L")

#### Scenario: User with no matches
- **WHEN** an authenticated user with no matches requests their summary
- **THEN** the system SHALL return zeroed statistics (0 matches, 0 wins, 0 losses, 0.0 win rate, empty recent form)

#### Scenario: Statistics scoped to user
- **WHEN** user A requests stats
- **THEN** the response SHALL only include matches belonging to user A, not other users' matches
