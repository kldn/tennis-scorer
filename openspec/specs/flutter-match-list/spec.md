## ADDED Requirements

### Requirement: Paginated match list screen
The app SHALL display a scrollable list of the user's matches fetched from `GET /api/matches`.

#### Scenario: Display match list
- **WHEN** the user navigates to the match list screen
- **THEN** the app SHALL display matches sorted by date (newest first), showing date, set scores, and win/loss indicator for each match

#### Scenario: Pagination
- **WHEN** the user scrolls to the bottom of the list
- **THEN** the app SHALL load the next page of matches (20 per page)

#### Scenario: Empty state
- **WHEN** the user has no matches
- **THEN** the app SHALL display a message indicating no matches have been recorded yet

#### Scenario: Pull to refresh
- **WHEN** the user pulls down on the match list
- **THEN** the app SHALL reload the match list from the API

### Requirement: Navigate to match detail
The app SHALL allow the user to tap a match to view its details.

#### Scenario: Tap match item
- **WHEN** the user taps a match in the list
- **THEN** the app SHALL navigate to the match detail screen for that match
