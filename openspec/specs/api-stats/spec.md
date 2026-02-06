## ADDED Requirements

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
