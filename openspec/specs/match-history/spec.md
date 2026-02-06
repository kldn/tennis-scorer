## ADDED Requirements

### Requirement: Match history list view
The app SHALL provide a `MatchHistoryView` that displays past matches from SwiftData, ordered by date descending.

#### Scenario: Matches displayed
- **WHEN** the user navigates to match history
- **THEN** the app SHALL show each match with: date, result (W/L), final set score, and sync status indicator

#### Scenario: No matches
- **WHEN** the user has no saved matches
- **THEN** the app SHALL display an empty state message

### Requirement: Sync status visible
Each match in the history list SHALL show whether it has been synced to the backend.

#### Scenario: Synced match
- **WHEN** a match has `isSynced == true`
- **THEN** the app SHALL show a sync-complete indicator (e.g., checkmark)

#### Scenario: Unsynced match
- **WHEN** a match has `isSynced == false`
- **THEN** the app SHALL show a pending-sync indicator (e.g., cloud with arrow)
