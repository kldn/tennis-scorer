## ADDED Requirements

### Requirement: TennisMatch object provides match lifecycle
The `tennis-scorer-uniffi` crate SHALL expose a `TennisMatch` UniFFI Object with constructors for all match types.

#### Scenario: Default match creation
- **WHEN** Swift code calls `TennisMatch()`
- **THEN** a best-of-3, ad-scoring, 7-point tiebreak match is created

#### Scenario: Custom match creation
- **WHEN** Swift code calls `TennisMatch(config:)` with a `MatchConfig` record
- **THEN** the match uses the specified settings (sets_to_win, tiebreak_points, final_set_tiebreak, no_ad_scoring)

#### Scenario: Doubles match creation
- **WHEN** Swift code calls `TennisMatch(config:)` with `matchType: .doubles` and a `serveOrder` list
- **THEN** the match uses doubles serve rotation

#### Scenario: ARC memory management
- **WHEN** the Swift `TennisMatch` reference count drops to zero
- **THEN** the Rust-side match is freed automatically (no manual `free()` call needed)

### Requirement: Score point and undo
The `TennisMatch` object SHALL provide methods to score points and undo.

#### Scenario: Score a point
- **WHEN** `scorePoint(player:)` is called with `.player1` or `.player2`
- **THEN** the method returns an updated `MatchScore` record

#### Scenario: Undo last point
- **WHEN** `undo()` is called and history is non-empty
- **THEN** the match state reverts to before the last point and returns the previous `MatchScore`

#### Scenario: Check undo availability
- **WHEN** `canUndo()` is called
- **THEN** it returns `true` if there is history to undo, `false` otherwise

### Requirement: MatchScore record contains full match state
The `MatchScore` UniFFI Record SHALL contain all information needed to render the UI.

#### Scenario: Score fields present
- **WHEN** `getScore()` is called on a playing match
- **THEN** the returned `MatchScore` contains: `player1Sets`, `player2Sets`, `player1Games` (all sets), `player2Games` (all sets), `currentGame` (GameScore enum), `winner` (Optional), `isTiebreak`, `deuceCount`, `currentServer`

#### Scenario: Completed match
- **WHEN** the match has a winner
- **THEN** `winner` is non-nil and `currentGame` is `.completed(winner:)`

### Requirement: Player enum maps to Swift enum
The UniFFI `Player` enum SHALL generate an idiomatic Swift enum.

#### Scenario: Enum values
- **WHEN** Swift code references `Player`
- **THEN** `.player1` and `.player2` are available as enum cases

### Requirement: GameScore enum with associated values
The UniFFI `GameScore` enum SHALL represent all game states with associated values.

#### Scenario: Points state
- **WHEN** the game is in regular play
- **THEN** `GameScore.points(player1:player2:)` contains string scores ("0", "15", "30", "40")

#### Scenario: Deuce state
- **WHEN** the game is at deuce
- **THEN** `GameScore.deuce` is returned

#### Scenario: Advantage state
- **WHEN** a player has advantage
- **THEN** `GameScore.advantage(player:)` indicates which player

### Requirement: Point events accessible
The `TennisMatch` object SHALL expose point event history for sync/export.

#### Scenario: Get point events
- **WHEN** `getPointEvents()` is called
- **THEN** it returns a list of `PointEvent` records with `player` and `timestamp` fields

#### Scenario: Point events reflect undo
- **WHEN** `undo()` is called
- **THEN** the most recent point event is removed from the list

### Requirement: New match reset
The `TennisMatch` object SHALL support starting a new match without creating a new object.

#### Scenario: Reset match
- **WHEN** `newMatch()` is called
- **THEN** the internal state resets to a fresh match with the same config
