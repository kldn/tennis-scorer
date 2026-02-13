## ADDED Requirements

### Requirement: Dashboard screen displays user summary
The phone app SHALL provide a Dashboard screen as the home page showing the user's match summary and quick access to key features.

#### Scenario: User with match history
- **WHEN** the user opens the Dashboard screen and has recorded matches
- **THEN** the app SHALL display total matches, win rate, recent streak, and a list of the most recent 5 matches

#### Scenario: User with no matches
- **WHEN** the user opens the Dashboard screen and has no recorded matches
- **THEN** the app SHALL display an empty state with a message encouraging them to record matches on their watch

#### Scenario: Pull to refresh
- **WHEN** the user pulls down on the Dashboard screen
- **THEN** the app SHALL refresh the summary data from `GET /api/stats/summary` and recent matches from `GET /api/matches`

### Requirement: Dashboard quick actions
The Dashboard SHALL provide quick navigation to key features.

#### Scenario: Navigate to match history
- **WHEN** the user taps "View All Matches" on the Dashboard
- **THEN** the app SHALL navigate to the full match history screen

#### Scenario: Navigate to social
- **WHEN** the user taps the friends/social section on the Dashboard
- **THEN** the app SHALL navigate to the Social screen

### Requirement: Dashboard data loading
The Dashboard SHALL load data from the backend API using Riverpod state management.

#### Scenario: Initial load
- **WHEN** the Dashboard screen mounts
- **THEN** the app SHALL show a loading indicator while fetching data from the API

#### Scenario: API error
- **WHEN** the API request fails (network error, server error)
- **THEN** the app SHALL display an error state with a retry button
