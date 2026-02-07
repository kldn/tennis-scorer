## MODIFIED Requirements

### Requirement: Player labels display in first-person perspective

UI SHALL display player labels from the user's perspective:
- Player 1 (self) SHALL be labeled as「我」
- Player 2 (opponent) SHALL be labeled as「對手」

#### Scenario: Score buttons show correct labels
- **WHEN** the match is in progress
- **THEN** the Player 1 button displays「我」
- **AND** the Player 2 button displays「對手」

#### Scenario: Win message shows correct label for self
- **WHEN** Player 1 wins the match
- **THEN** the victory message displays「我贏了！」

#### Scenario: Win message shows correct label for opponent
- **WHEN** Player 2 wins the match
- **THEN** the victory message displays「對手贏了」

Note: Player mapping changes from C FFI `PLAYER_1`/`PLAYER_2` constants to UniFFI `Player.player1`/`Player.player2` enum. Label logic remains identical.
