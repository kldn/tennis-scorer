## Context

The tennis-scorer core crate implements an immutable state machine for scoring tennis matches. Each point scored produces a new `MatchState` via `score_point()`. The `MatchWithHistory` wrapper stores the complete sequence of `(Player, SystemTime)` point events alongside the state history.

Currently there is no way to extract contextual statistics from this data — e.g., whether a point was a break point, game point, or match point. The raw data exists (who won each point and when), but the scoring context is lost after each transition.

The API crate (`tennis-scorer-api`) provides a `GET /api/stats/summary` endpoint for aggregate win/loss stats. There are no per-match analysis endpoints.

### Constraints
- All statistics must be computable from **point-winner + timestamp** data only
- No serve speed, shot type, rally length, or tracking hardware data available
- Core crate has no `serde` on state types (only on `MatchConfig`) — new analysis types need their own serialization
- Immutable architecture: replay builds new states, never mutates existing ones
- Singles serve tracking uses game parity (odd/even); doubles uses `serve_order` + `serve_rotation_index`

## Goals / Non-Goals

**Goals:**
- Replay a completed match's point events to annotate each point with full scoring context
- Compute comprehensive statistics covering break points, service performance, deuce analysis, conversion rates, momentum, streaks, clutch performance, pace, and tiebreak analysis
- Expose statistics via 3 API endpoints (analysis, momentum, pace)
- Expose core statistics via UniFFI for watchOS post-match summary
- Design `PointContext` with a reserved `point_end_type` field for future Ace/DF/Winner/UE tagging

**Non-Goals:**
- Shot-level tagging UI (Ace/DF/Winner/UE) — deferred to future change
- Real-time live statistics during a match (analysis is post-match only)
- Historical cross-match trend analysis (this change is single-match scope)
- Serve speed or any hardware-dependent metrics

## Decisions

### 1. Replay via state machine re-execution

**Decision:** Replay by feeding point events through `MatchState::new() → score_point()` in sequence, capturing state before each point to determine context flags.

**Rationale:** The existing immutable state machine already handles all scoring rules (deuce, tiebreak, no-ad, doubles serve rotation). Re-using it guarantees correctness without duplicating logic.

**Alternative considered:** Parse the stored `Vec<MatchState>` history directly. Rejected because `MatchWithHistory.history` stores states but doesn't expose enough public API to extract game/set point status, and adding getters would bloat the core API for a single use case. Replay is cleaner.

### 2. Serving player detection for singles

**Decision:** For singles (empty `serve_order`), determine server by counting total completed games modulo 2. Game 0 = Player1 serves, Game 1 = Player2, etc. During tiebreaks, use the same offset formula as doubles (`tiebreak_server_offset`).

**Rationale:** The core crate doesn't track singles serving because it doesn't affect scoring. But for statistics (break points, hold %), we need to know who served each game. Game parity is the standard tennis rule.

**Alternative considered:** Add `serving_player` field to `MatchState`. Rejected — would change the core data model for a derived concern.

### 3. Statistics as pure functions over `Vec<PointContext>`

**Decision:** All statistics are computed as pure functions: `Vec<PointContext> → SomeStatsStruct`. No mutable accumulators or stateful builders.

**Rationale:** Matches the project's immutable architecture. Pure functions are trivially testable and composable. Each stat category gets its own function.

### 4. API endpoints read from database, replay in-memory

**Decision:** The 3 new API endpoints load `MatchConfig` + `point_events` from the database, call `replay_with_context()` in-memory, then compute requested statistics.

**Rationale:** Point event data is small (typically <200 points per match). In-memory replay is fast (<1ms). No need to pre-compute or cache statistics. This keeps the database schema simple — no new tables for stats.

**Alternative considered:** Pre-compute stats on match completion and store in a `match_stats` table. Rejected — adds schema complexity, migration burden, and staleness risk for negligible performance gain.

### 5. UniFFI exports for watchOS

**Decision:** Export `MatchAnalysis`, `MomentumData`, and `PaceData` structs via UniFFI proc-macro. The watchOS app calls these directly on the local `MatchWithHistory` data without going through the API.

**Rationale:** The watch app already has the full point history in memory. No network round-trip needed. UniFFI bindings are already set up from the `uniffi-migration` change.

### 6. Reserved PointEndType field

**Decision:** `PointContext` includes `point_end_type: Option<PointEndType>` where `PointEndType` is an enum with variants `Ace`, `DoubleFault`, `Winner`, `UnforcedError`, `ForcedError`, `Normal`. Default is `None`. Field uses `#[serde(default)]`.

**Rationale:** Allows future UI to tag points without changing the data model. Current replay always sets `None`. Backward compatible via `serde(default)`.

## Risks / Trade-offs

**[Serving player inference for singles may diverge from reality]** → If a user starts tracking mid-game or undoes across game boundaries, the game-parity heuristic could be wrong. Mitigation: replay always starts from point 0, and undo removes point events, so the parity stays correct for a complete replay.

**[Momentum weighted formula is subjective]** → The weights (break ×3, set ×5, deuce ×1.5) are arbitrary. Mitigation: return both basic and weighted values, let the client choose. Weights can be tuned later without API changes.

**[No pagination on momentum endpoint]** → A 5-set match could have ~300 points. Mitigation: 300 momentum entries at ~50 bytes each ≈ 15KB JSON. Acceptable without pagination.

**[Clutch score aggregation is novel]** → No standard "clutch score" exists in tennis analytics. Mitigation: expose individual critical-point rates alongside the aggregate. Users can ignore the aggregate if they prefer raw rates.
