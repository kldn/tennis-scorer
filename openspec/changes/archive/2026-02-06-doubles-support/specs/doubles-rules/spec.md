## ADDED Requirements

### Requirement: Match type configuration

The system SHALL support configuring a match as Singles or Doubles.

#### Scenario: Default match type is Singles
- **WHEN** a `MatchConfig` is created with default settings
- **THEN** the match type SHALL be Singles

#### Scenario: Configure a doubles match
- **WHEN** a `MatchConfig` is created with match type set to Doubles
- **AND** a serve order of 4 players is provided (e.g., [A, C, B, D])
- **THEN** the system SHALL accept the configuration and create a valid match

#### Scenario: Singles match ignores serve order
- **WHEN** the match type is Singles
- **THEN** the system SHALL NOT require or use a doubles serve order

### Requirement: Teams map to existing Player enum

The system SHALL use the existing `Player::Player1` and `Player::Player2` enum to represent Team 1 and Team 2 respectively. No new team type is needed.

#### Scenario: Scoring a point in doubles
- **WHEN** a point is scored by Team 1 in a doubles match
- **THEN** the scorer SHALL be `Player::Player1`
- **AND** all existing scoring logic (game, set, tiebreak) SHALL apply unchanged

### Requirement: Serve rotation in regular games

In a doubles match, the system SHALL rotate the server through all 4 players in a fixed order across games.

#### Scenario: Initial server
- **GIVEN** a doubles match with serve order [A, C, B, D]
- **WHEN** the match begins
- **THEN** player A SHALL serve the first game

#### Scenario: Server rotates after each game
- **GIVEN** a doubles match with serve order [A, C, B, D]
- **WHEN** the first game completes
- **THEN** player C (from the other team) SHALL serve the second game
- **AND** player B SHALL serve the third game
- **AND** player D SHALL serve the fourth game
- **AND** the rotation SHALL cycle back to player A for the fifth game

#### Scenario: Serve rotation continues across sets
- **GIVEN** a doubles match where the serve order position is at index 2 when a set ends
- **WHEN** the next set begins
- **THEN** the server SHALL be the next player in rotation (index 3)

### Requirement: Serve rotation in tiebreaks

In a doubles tiebreak, the serve SHALL rotate every 2 points after the initial point.

#### Scenario: Tiebreak first point
- **GIVEN** a doubles tiebreak begins
- **AND** the next server in rotation is player X
- **THEN** player X SHALL serve the first point of the tiebreak

#### Scenario: Tiebreak serve changes every 2 points
- **GIVEN** a doubles tiebreak where player X served the first point
- **WHEN** the first point is completed
- **THEN** the next player in rotation SHALL serve points 2 and 3
- **AND** the following player SHALL serve points 4 and 5
- **AND** this pattern SHALL continue (2 points per server) for the remainder of the tiebreak

#### Scenario: Serve order after tiebreak
- **WHEN** a tiebreak completes in a doubles match
- **THEN** the serve rotation SHALL continue from where it left off in the regular rotation
- **AND** the next game's server SHALL be determined by the post-tiebreak rotation position

### Requirement: Query current server

The system SHALL expose which player/position is currently serving.

#### Scenario: Query server during regular game
- **WHEN** the match is in progress during a regular game
- **THEN** the system SHALL report the current server's position in the rotation (0-3)

#### Scenario: Query server during tiebreak
- **WHEN** the match is in a tiebreak
- **THEN** the system SHALL report the current tiebreak server's position

### Requirement: Unchanged scoring rules

The core scoring rules SHALL remain identical for singles and doubles.

#### Scenario: Game scoring in doubles
- **WHEN** a point is scored in a doubles match
- **THEN** game scoring SHALL follow the same rules as singles (Love, 15, 30, 40, Deuce, Advantage)

#### Scenario: Set scoring in doubles
- **WHEN** games are won in a doubles match
- **THEN** set scoring SHALL follow the same rules as singles (first to 6 with 2-game lead, tiebreak at 6-6)

#### Scenario: Match scoring in doubles
- **WHEN** sets are won in a doubles match
- **THEN** match scoring SHALL follow the configured sets_to_win (best-of-3 or best-of-5)
