## ADDED Requirements

### Requirement: PointContext struct captures full scoring context per point
The system SHALL define a `PointContext` struct containing:
- `point_number: u32` — 1-indexed position in the match
- `scorer: Player` — who won this point
- `timestamp: SystemTime` — when the point was scored
- `serving_player: Player` — who was serving during this point
- `score_before: ScoreSnapshot` — full score state before this point was played
- `is_break_point: bool` — true if the returner could win the game by winning this point
- `is_game_point: bool` — true if either player could win the game by winning this point
- `is_set_point: bool` — true if either player could win the set by winning this game
- `is_match_point: bool` — true if either player could win the match by winning this set
- `game_number_in_set: u32` — 1-indexed game number within the current set
- `set_number: u32` — 1-indexed set number
- `is_tiebreak: bool` — true if this point is played during a tiebreak
- `point_end_type: Option<PointEndType>` — reserved for future use, always `None` in this change

#### Scenario: PointContext for a normal point
- **WHEN** a point is replayed at score 15-0 in game 3 of set 1 with Player1 serving
- **THEN** the `PointContext` SHALL have `point_number` reflecting its position, `serving_player = Player1`, `is_break_point = false`, `is_game_point = false`, `game_number_in_set = 3`, `set_number = 1`, `is_tiebreak = false`

#### Scenario: PointContext for a break point
- **WHEN** a point is replayed at 30-40 with Player1 serving
- **THEN** `is_break_point` SHALL be `true` and `is_game_point` SHALL be `true`

#### Scenario: PointContext for a set point
- **WHEN** a point is replayed at 40-0 with Player1 serving and Player1 leads 5-3 in games
- **THEN** `is_set_point` SHALL be `true`

#### Scenario: PointContext for a match point
- **WHEN** a point is replayed where winning this game would win the set, and winning this set would win the match
- **THEN** `is_match_point` SHALL be `true`

### Requirement: ScoreSnapshot captures the full score state
The system SHALL define a `ScoreSnapshot` struct containing:
- `sets: Vec<SetScore>` — completed and current set scores
- `current_game: GameScore` — point score in the current game
- `player1_sets: u8`, `player2_sets: u8` — sets won

#### Scenario: ScoreSnapshot at match start
- **WHEN** the first point is about to be played
- **THEN** `ScoreSnapshot` SHALL show 0-0 in sets, 0-0 in games, Love-Love in points

### Requirement: replay_with_context reconstructs annotated point sequence
The system SHALL provide a function `replay_with_context(config: &MatchConfig, events: &[(Player, SystemTime)]) -> Vec<PointContext>` that:
1. Creates a fresh `MatchState` from the config
2. For each event, captures the current state as a `PointContext` before calling `score_point()`
3. Determines `serving_player` using game parity (singles) or `serve_order` + rotation index (doubles)
4. Determines break/game/set/match point flags from the current `MatchState`
5. Returns a `Vec<PointContext>` with length equal to the input events

#### Scenario: Replay a complete singles match
- **WHEN** `replay_with_context` is called with a valid config and point events from a completed singles match
- **THEN** the returned `Vec<PointContext>` SHALL have the same length as the input events, and every `serving_player` SHALL alternate correctly per game

#### Scenario: Replay a doubles match with serve rotation
- **WHEN** `replay_with_context` is called with a doubles config containing a 4-player `serve_order`
- **THEN** `serving_player` SHALL follow the configured rotation order, including tiebreak rotation rules

#### Scenario: Replay with empty events
- **WHEN** `replay_with_context` is called with an empty event list
- **THEN** it SHALL return an empty `Vec<PointContext>`

### Requirement: Serving player detection for singles
For singles matches (empty `serve_order`), the system SHALL determine the serving player by counting total completed games (across all sets) modulo 2. Game 0 = Player1 serves. During tiebreaks, the system SHALL use the standard tiebreak serving pattern (first server serves 1 point, then alternating 2 points each).

#### Scenario: Singles serving alternates each game
- **WHEN** replaying a singles match where Player1 wins the first game (4 points)
- **THEN** points 1-4 SHALL have `serving_player = Player1` and points in the next game SHALL have `serving_player = Player2`

#### Scenario: Singles tiebreak serving
- **WHEN** a tiebreak begins at 6-6 in a singles match
- **THEN** the first point SHALL be served by the player whose turn it is (game 12 = even = Player1), then the opponent serves 2, then alternating 2 each

### Requirement: PointEndType enum reserved for future extension
The system SHALL define a `PointEndType` enum with variants: `Ace`, `DoubleFault`, `Winner`, `UnforcedError`, `ForcedError`, `Normal`. The `point_end_type` field on `PointContext` SHALL default to `None` via `#[serde(default)]`.

#### Scenario: Current replay always sets None
- **WHEN** `replay_with_context` generates `PointContext` entries
- **THEN** every entry SHALL have `point_end_type = None`
