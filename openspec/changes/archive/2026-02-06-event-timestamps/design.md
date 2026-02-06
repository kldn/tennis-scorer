## Context

The tennis-scorer engine uses an immutable state machine pattern. `MatchWithHistory` wraps `MatchState` and maintains a `Vec<MatchState>` for undo support. Each `score_point()` call pushes the current state onto the history stack and returns a new `MatchWithHistory` with the updated state.

Currently, history entries are bare `MatchState` snapshots — they record what the state was before each point, but not when each point was scored. The Watch App and future frontends need timestamp data for analytics (pace of play, per-set duration, time-based performance).

## Goals / Non-Goals

**Goals:**
- Record `SystemTime::now()` with each scored point in the core engine
- Expose timestamped point event list via C FFI for Watch App consumption
- Maintain the immutable, functional style of the scoring engine
- Zero impact on scoring logic

**Non-Goals:**
- Time-based analytics computation (belongs in `match-replay-analysis` change)
- Persisting timestamps to disk (belongs in `offline-sync` change)
- Accepting externally-provided timestamps (engine always uses `SystemTime::now()`)

## Decisions

### 1. Store timestamps in a parallel Vec, not inside MatchState history

**Decision**: Add a `Vec<(Player, SystemTime)>` field to `MatchWithHistory` alongside the existing `Vec<MatchState>` history.

**Rationale**: The existing `history` stores `MatchState` snapshots for undo. Embedding timestamps into `MatchState` would pollute the scoring state machine with observational metadata. A parallel vec keeps concerns separated — `history` is for undo, `timestamps` is for analytics.

**Alternative considered**: Wrapping each history entry in a `struct HistoryEntry { state: MatchState, player: Player, timestamp: SystemTime }`. This is cleaner in theory but requires changing the undo logic and all existing tests. The parallel vec is a minimal change.

### 2. Use `std::time::SystemTime` (not `Instant`)

**Decision**: Use `SystemTime` for timestamps.

**Rationale**: `SystemTime` maps to wall-clock time (Unix epoch), which is meaningful for analytics and serialization. `Instant` is monotonic but opaque — it can't be serialized or compared across processes.

### 3. FFI exposes timestamps as `f64` Unix epoch seconds

**Decision**: In the C FFI, represent timestamps as `f64` (seconds since Unix epoch, with sub-second precision).

**Rationale**: `f64` is universally consumable from C/Swift/any language. It avoids the complexity of passing `timespec` structs or separate seconds/nanoseconds fields. Millisecond precision is sufficient (tennis points are seconds apart).

**FFI struct**:
```c
typedef struct {
    uint8_t player;     // PLAYER_1 or PLAYER_2
    double timestamp;   // Unix epoch seconds (e.g., 1738857015.123)
} PointEvent;
```

**FFI functions**:
```c
uint32_t tennis_match_get_point_count(const TennisMatch *match);
bool tennis_match_get_points(const TennisMatch *match, PointEvent *buffer, uint32_t buffer_size);
```

The caller allocates the buffer (size from `get_point_count`), then calls `get_points` to fill it. This avoids heap allocation in FFI.

### 4. No changes to `score_point()` signature

**Decision**: `score_point(&self, scorer: Player) -> MatchWithHistory` signature stays the same. Timestamp is captured internally.

**Rationale**: All existing callers (tests, FFI) continue to work unchanged. The timestamp is an implementation detail of `MatchWithHistory`, not a parameter.

## Risks / Trade-offs

- **[Risk] SystemTime::now() in tests makes assertions non-deterministic** → Tests that check timestamps will use `SystemTime::now()` before/after calls and assert the recorded timestamp falls within that range. For unit tests that don't care about time, timestamps are simply ignored.

- **[Risk] Parallel vec can drift from history vec** → Both vecs are only modified in `score_point()` and `undo()`, which always push/pop in lockstep. A debug assertion `assert_eq!(history.len(), timestamps.len())` guards against bugs.

- **[Trade-off] No injectable clock** → We always use `SystemTime::now()`. This makes the engine slightly less testable for time-dependent scenarios, but avoids over-engineering. If needed later, a clock trait can be added.
