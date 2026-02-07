## ADDED Requirements

### Requirement: Track deuce count within a game

The system SHALL track the number of times a game enters deuce state.

#### Scenario: First deuce in a game
- **WHEN** the score reaches 40-40 for the first time in a game
- **THEN** the deuce count SHALL be 1

#### Scenario: Deuce after advantage is broken
- **WHEN** the player with advantage loses the next point
- **AND** the game returns to deuce
- **THEN** the deuce count SHALL increment by 1

#### Scenario: Deuce count resets on new game
- **WHEN** a game completes and a new game begins
- **THEN** the deuce count SHALL be 0

#### Scenario: Deuce count during normal play
- **WHEN** the game is not in deuce state (score is not 40-40 or advantage)
- **THEN** the deuce count SHALL be 0

### Requirement: Expose deuce count via FFI

The FFI layer SHALL expose the current game's deuce count.

#### Scenario: Query deuce count during deuce
- **WHEN** the current game is in deuce with count N
- **THEN** `tennis_match_get_score()` SHALL return a MatchScore with `deuce_count = N`

#### Scenario: Query deuce count during advantage
- **WHEN** the current game is in advantage state
- **THEN** `tennis_match_get_score()` SHALL return the deuce count from the preceding deuce

### Requirement: Display deuce count in UI

The watchOS UI SHALL display the deuce count when relevant.

#### Scenario: Show deuce count during deuce
- **WHEN** the game is in deuce state with count N > 0
- **THEN** the UI SHALL display the deuce count (e.g., "Deuce (2)")

#### Scenario: Hide deuce count when not in deuce
- **WHEN** the game is in normal play or advantage
- **THEN** the UI SHALL NOT display a deuce count indicator
