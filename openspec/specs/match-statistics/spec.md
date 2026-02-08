## ADDED Requirements

### Requirement: Break point statistics
The system SHALL compute per-player break point statistics from `Vec<PointContext>`:
- `break_points_created: u32` — points where this player was returning and had a break point opportunity
- `break_points_converted: u32` — break points where the returner won the point
- `break_points_faced: u32` — points where this player was serving and faced a break point
- `break_points_saved: u32` — break points where the server saved (won the point)
- `break_point_conversion_rate: f64` — converted / created (0.0 if none created)

#### Scenario: Player breaks serve once in a set
- **WHEN** Player2 has 3 break point opportunities against Player1 and converts 1
- **THEN** Player2's `break_points_created = 3`, `break_points_converted = 1`, `break_point_conversion_rate ≈ 0.333`

#### Scenario: No break points in match
- **WHEN** no break points occur (e.g., every game is won to love by the server)
- **THEN** both players SHALL have `break_points_created = 0`, `break_point_conversion_rate = 0.0`

### Requirement: Service and return statistics
The system SHALL compute per-player service/return stats:
- `service_games_played: u32` — total games served
- `service_games_held: u32` — games where server won
- `hold_percentage: f64` — held / played
- `return_games_played: u32` — games where this player was returning
- `return_games_won: u32` — return games where returner broke serve
- `break_percentage: f64` — won / played
- `service_points_won: u32` — points won while serving
- `service_points_total: u32` — total points played while serving
- `return_points_won: u32` — points won while returning
- `return_points_total: u32` — total points played while returning
- `dominance_ratio: f64` — (return_points_won / return_points_total) / (1.0 - service_points_won / service_points_total), or 0.0 if denominator is 0

#### Scenario: Player holds all service games
- **WHEN** Player1 serves 6 games and holds all 6
- **THEN** Player1's `hold_percentage = 1.0` and `service_games_held = 6`

#### Scenario: Dominance ratio calculation
- **WHEN** Player1 wins 60% of return points and loses 30% of service points
- **THEN** Player1's `dominance_ratio ≈ 2.0` (0.60 / 0.30)

### Requirement: Deuce analysis
The system SHALL compute deuce statistics:
- `deuce_games_count: u32` — number of games that reached deuce
- `deuce_games_won: u32` — deuce games won (per player)
- `deuce_game_win_rate: f64` — won / count per player
- `total_deuce_count: u32` — sum of all deuce counts across all deuce games
- `average_deuces_per_deuce_game: f64` — total_deuce_count / deuce_games_count

A "deuce game" is identified when a `PointContext` has `score_before` showing 40-40 (Deuce state) at any point during the game.

#### Scenario: Match with multiple deuce games
- **WHEN** 3 games reach deuce with deuce counts of 1, 2, and 3
- **THEN** `deuce_games_count = 3`, `total_deuce_count = 6`, `average_deuces_per_deuce_game = 2.0`

#### Scenario: Match with no deuce games
- **WHEN** no game reaches deuce
- **THEN** `deuce_games_count = 0`, `average_deuces_per_deuce_game = 0.0`

### Requirement: Conversion rate statistics
The system SHALL compute conversion rates for critical points:
- `game_points_total: u32` — total game point opportunities (per player)
- `game_points_converted: u32` — game points where the player won
- `game_point_conversion_rate: f64`
- `set_points_total: u32` — total set point opportunities
- `set_points_converted: u32`
- `set_point_conversion_rate: f64`
- `match_points_total: u32` — total match point opportunities
- `match_points_converted: u32`
- `match_point_conversion_rate: f64`

#### Scenario: Player converts first match point
- **WHEN** Player1 has 1 match point and wins it
- **THEN** `match_points_total = 1`, `match_points_converted = 1`, `match_point_conversion_rate = 1.0`

#### Scenario: Player needs multiple set points
- **WHEN** Player1 has 3 set point opportunities and converts on the 3rd
- **THEN** `set_points_total = 3`, `set_points_converted = 1`, `set_point_conversion_rate ≈ 0.333`

### Requirement: Streak statistics
The system SHALL compute streak stats per player:
- `longest_point_streak: u32` — most consecutive points won
- `longest_point_drought: u32` — most consecutive points lost
- `longest_service_hold_streak: u32` — most consecutive service games held
- `max_games_in_a_row: u32` — most consecutive games won

#### Scenario: Player wins 8 consecutive points
- **WHEN** Player1 wins points 15-22 consecutively
- **THEN** Player1's `longest_point_streak >= 8`

#### Scenario: No streaks beyond 1
- **WHEN** players alternate winning every point
- **THEN** both players' `longest_point_streak = 1`

### Requirement: Clutch performance statistics
The system SHALL compute clutch stats per player:
- `break_point_win_rate: f64` — win rate on break point opportunities (as returner)
- `set_point_win_rate: f64` — win rate on set point opportunities
- `match_point_win_rate: f64` — win rate on match point opportunities
- `normal_point_win_rate: f64` — win rate on non-critical points
- `clutch_score: f64` — weighted aggregate: `0.4 × break_point_win_rate + 0.35 × set_point_win_rate + 0.25 × match_point_win_rate`

A "critical point" is any point where `is_break_point`, `is_set_point`, or `is_match_point` is true.

#### Scenario: Player performs better under pressure
- **WHEN** Player1 wins 80% of critical points and 50% of normal points
- **THEN** Player1's clutch rates SHALL reflect the higher win rate on critical points

#### Scenario: No critical points for a player
- **WHEN** Player1 never faces a break/set/match point
- **THEN** the corresponding rates SHALL be `0.0` and `clutch_score = 0.0`

### Requirement: Tiebreak performance statistics
The system SHALL compute tiebreak stats per player:
- `tiebreaks_played: u32`
- `tiebreaks_won: u32`
- `tiebreak_win_rate: f64`
- `average_tiebreak_margin: f64` — average absolute point difference in tiebreaks played

#### Scenario: Player wins a tiebreak 7-4
- **WHEN** Player1 wins a tiebreak 7-4
- **THEN** the margin for this tiebreak SHALL be 3, and `tiebreaks_won` for Player1 SHALL increment

#### Scenario: No tiebreaks in match
- **WHEN** no set goes to a tiebreak
- **THEN** `tiebreaks_played = 0`, `tiebreak_win_rate = 0.0`, `average_tiebreak_margin = 0.0`

### Requirement: Total points statistics
The system SHALL compute total points stats:
- `points_won: u32` per player
- `total_points: u32` — total points in the match
- `points_won_percentage: f64` per player

#### Scenario: Even match
- **WHEN** a match has 150 total points with Player1 winning 78
- **THEN** Player1's `points_won = 78`, `points_won_percentage ≈ 0.52`

### Requirement: MatchAnalysis aggregates all statistics
The system SHALL define a `MatchAnalysis` struct containing all of the above stats per player, returned from a single function `compute_analysis(points: &[PointContext]) -> MatchAnalysis`.

#### Scenario: Complete analysis from replay data
- **WHEN** `compute_analysis` is called with a valid `Vec<PointContext>` from a completed match
- **THEN** the returned `MatchAnalysis` SHALL contain all break point, service, deuce, conversion, streak, clutch, tiebreak, and total point statistics for both players
