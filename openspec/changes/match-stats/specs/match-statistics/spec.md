## ADDED Requirements

### Requirement: Calculate match statistics from history

The system SHALL calculate statistics based on saved match history.

#### Scenario: Calculate wins and losses
- **WHEN** the history contains matches with different winners
- **THEN** the system SHALL count wins as matches where Player 1 (我) won
- **AND** the system SHALL count losses as matches where Player 2 (對手) won

#### Scenario: Calculate win rate
- **WHEN** the history contains N total matches with W wins
- **THEN** the win rate SHALL be W / N expressed as a percentage

#### Scenario: Empty history statistics
- **WHEN** no matches exist in history
- **THEN** wins SHALL be 0
- **AND** losses SHALL be 0
- **AND** win rate SHALL be 0%

### Requirement: Display statistics in UI

The system SHALL display match statistics in the user interface.

#### Scenario: Show statistics after match completion
- **WHEN** a match is completed
- **THEN** the UI SHALL display the current statistics
- **AND** the format SHALL be "戰績: N勝 M敗 (P%)"

#### Scenario: Statistics update after new match saved
- **WHEN** a new completed match is saved to history
- **THEN** the displayed statistics SHALL reflect the updated totals

### Requirement: Statistics API

The system SHALL provide an API to retrieve current statistics.

#### Scenario: Get statistics
- **WHEN** the app requests match statistics
- **THEN** the system SHALL return a MatchStats object containing:
  - Total number of matches
  - Number of wins (Player 1)
  - Number of losses (Player 2)
  - Win rate as a decimal (0.0 to 1.0)
