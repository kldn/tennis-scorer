## ADDED Requirements

### Requirement: UniFFI exports for match analysis types
The system SHALL export the following types via UniFFI proc-macro for Swift consumption:
- `PointContext`
- `ScoreSnapshot`, `SetScore`, `GameScore`
- `PointEndType`
- `MatchAnalysis` and all per-player stat structs
- `MomentumData`
- `PaceData` (containing durations and intervals)

#### Scenario: Swift can instantiate analysis types
- **WHEN** the UniFFI bindings are generated
- **THEN** Swift code SHALL be able to access all fields of `MatchAnalysis`, `MomentumData`, and `PaceData`

### Requirement: UniFFI exports for analysis functions
The system SHALL export the following functions via UniFFI:
- `replay_with_context(config: MatchConfig, events: Vec<PointEvent>) -> Vec<PointContext>` where `PointEvent` is a UniFFI-compatible wrapper for `(Player, SystemTime)`
- `compute_analysis(points: Vec<PointContext>) -> MatchAnalysis`
- `compute_momentum(points: Vec<PointContext>) -> MomentumData`
- `compute_pace(points: Vec<PointContext>) -> PaceData`

#### Scenario: watchOS app computes post-match analysis
- **WHEN** a match completes on the watchOS app
- **THEN** the app SHALL call `replay_with_context` with the stored config and point events, then pass the result to `compute_analysis` to display statistics

#### Scenario: watchOS app shows momentum chart
- **WHEN** the user views match details on the watchOS app
- **THEN** the app SHALL call `compute_momentum` and display the basic momentum series

### Requirement: PaceData struct for timing analysis
The system SHALL define a `PaceData` struct containing:
- `average_point_interval_seconds: f64`
- `per_game_durations_seconds: Vec<f64>`
- `per_set_durations_seconds: Vec<f64>`
- `total_duration_seconds: f64`

This is a simplified version of the API pace response, suitable for watch display.

#### Scenario: PaceData from a 30-minute match
- **WHEN** `compute_pace` is called for a match spanning 30 minutes
- **THEN** `total_duration_seconds ≈ 1800.0` and per-set durations SHALL sum to approximately 1800
