## MODIFIED Requirements

### Requirement: Match history list view
The app SHALL provide a `MatchHistoryView` that displays past matches from SwiftData, ordered by date descending.

#### Scenario: Matches displayed
- **WHEN** the user navigates to match history
- **THEN** the app SHALL show each match with: date, result (W/L), final set score, and sync status indicator

#### Scenario: No matches
- **WHEN** the user has no saved matches
- **THEN** the app SHALL display an empty state message

Note: The underlying match data source changes from C FFI point events to UniFFI `getPointEvents()` return value, but the SwiftData model and UI remain identical.
