## ADDED Requirements

### Requirement: Auto-save completed match on new match

The system SHALL automatically save a completed match when the user starts a new match.

#### Scenario: Save match when starting new match after completion
- **WHEN** the current match is completed (has a winner)
- **AND** the user taps "New Match"
- **THEN** the system SHALL save the completed match to history
- **AND** the system SHALL start a fresh new match

#### Scenario: No save when starting new match during play
- **WHEN** the current match is in progress (no winner yet)
- **AND** the user starts a new match
- **THEN** the system SHALL NOT save the in-progress match
- **AND** the system SHALL start a fresh new match

### Requirement: Store match history persistently

The system SHALL persist match history across app restarts.

#### Scenario: History survives app restart
- **WHEN** matches have been saved to history
- **AND** the app is terminated and relaunched
- **THEN** the saved matches SHALL still be available in history

#### Scenario: History limited to 100 matches
- **WHEN** history contains 100 matches
- **AND** a new match is saved
- **THEN** the oldest match SHALL be removed
- **AND** the new match SHALL be added

### Requirement: Match record contains essential information

Each saved match record SHALL contain the following information.

#### Scenario: Match record data
- **WHEN** a match is saved
- **THEN** the record SHALL include:
  - Unique identifier (UUID)
  - Completion date/time
  - Winner (player 1 or player 2)
  - Final set score (e.g., 2-1)

### Requirement: Retrieve match history

The system SHALL provide an API to retrieve saved match history.

#### Scenario: Get all history
- **WHEN** the app requests match history
- **THEN** the system SHALL return all saved matches
- **AND** matches SHALL be ordered by date (most recent first)

#### Scenario: Empty history
- **WHEN** no matches have been completed yet
- **THEN** the system SHALL return an empty list
