## 1. Core Types & Replay Engine

- [x] 1.1 Define `PointEndType` enum, `ScoreSnapshot`/`SetScore`/`GameScore` structs, and `PointContext` struct in new `crates/tennis-scorer/src/analysis/types.rs`
- [x] 1.2 Add `mod analysis` to `crates/tennis-scorer/src/lib.rs` and create module structure (`analysis/mod.rs`)
- [x] 1.3 Implement singles serving player detection (game parity modulo 2, tiebreak rotation) as helper function
- [x] 1.4 Implement `score_snapshot_from_state(state: &MatchState) -> ScoreSnapshot` to extract current score
- [x] 1.5 Implement break/game/set/match point detection from `MatchState` + `GameState` + `SetState`
- [x] 1.6 Implement `replay_with_context(config: &MatchConfig, events: &[(Player, SystemTime)]) -> Vec<PointContext>`
- [x] 1.7 Write tests for replay: empty events, simple game, break point detection, set point, match point, tiebreak, doubles serve rotation

## 2. Statistics Module

- [x] 2.1 Implement break point statistics (`compute_break_points`) in `analysis/stats.rs`
- [x] 2.2 Implement service & return statistics (`compute_service_stats`) including dominance ratio
- [x] 2.3 Implement deuce analysis (`compute_deuce_stats`)
- [x] 2.4 Implement conversion rate statistics (`compute_conversion_rates`) for game/set/match points
- [x] 2.5 Implement streak statistics (`compute_streaks`)
- [x] 2.6 Implement clutch performance statistics (`compute_clutch`)
- [x] 2.7 Implement tiebreak performance statistics (`compute_tiebreak_stats`)
- [x] 2.8 Implement total points statistics (`compute_total_points`)
- [x] 2.9 Define `MatchAnalysis` struct and implement `compute_analysis(points: &[PointContext]) -> MatchAnalysis` that aggregates all stats
- [x] 2.10 Write tests for each stats function with known match scenarios

## 3. Momentum Module

- [x] 3.1 Implement basic momentum calculation in `analysis/momentum.rs`
- [x] 3.2 Implement weighted momentum calculation with break ×3, set ×5, deuce ×1.5 stacking
- [x] 3.3 Implement per-set momentum partitioning
- [x] 3.4 Define `MomentumData` struct and implement `compute_momentum(points: &[PointContext]) -> MomentumData`
- [x] 3.5 Write tests for momentum: empty match, basic series, weighted amplification, per-set partitioning

## 4. Pace Module

- [x] 4.1 Implement pace/timing calculations in `analysis/pace.rs`: point intervals, game durations, set durations
- [x] 4.2 Define `PaceData` struct and implement `compute_pace(points: &[PointContext]) -> PaceData`
- [x] 4.3 Write tests for pace: single point, normal match, interval calculations

## 5. UniFFI Bindings

- [x] 5.1 Add `#[uniffi::export]` annotations to analysis types and functions
- [x] 5.2 Ensure all exported types derive necessary UniFFI traits (`uniffi::Record`, `uniffi::Enum`)
- [x] 5.3 Create `PointEvent` wrapper struct for UniFFI-compatible `(Player, SystemTime)` input
- [x] 5.4 Verify UniFFI binding generation compiles successfully

## 6. API Endpoints

- [x] 6.1 Add `point_events` column/table to database schema for storing per-match point events (JSON array of `{player, timestamp}`)
- [x] 6.2 Implement `GET /api/stats/match/:id/analysis` handler — load match, replay, return `MatchAnalysis` JSON
- [x] 6.3 Implement `GET /api/stats/match/:id/momentum` handler — return `MomentumData` JSON
- [x] 6.4 Implement `GET /api/stats/match/:id/pace` handler — return pace/timing JSON
- [x] 6.5 Register new routes in stats router alongside existing `/api/stats/summary`
- [x] 6.6 Add ownership check (match belongs to authenticated user) and 404 handling
- [x] 6.7 Write API integration tests for all 3 endpoints (valid, not found, not owned, no events)

## 7. Verification

- [x] 7.1 Run full test suite (`cargo test --workspace`) and verify all tests pass
- [x] 7.2 Run `cargo clippy --workspace` with no warnings
- [x] 7.3 Verify UniFFI Swift bindings generate correctly for watchOS target
