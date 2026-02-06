## Why

The scoring engine currently records point history as a sequence of `MatchState` snapshots for undo support, but lacks any notion of when each point occurred. Without timestamps, time-based analytics (pace of play, per-set duration, performance over time) are impossible. Adding timestamps at the core engine level ensures all frontends (Watch App, future Flutter app, future API) automatically inherit this data without duplicating logic.

## What Changes

- Add a `timestamp` field (`std::time::SystemTime`) to each history entry in `MatchWithHistory`
- Record `SystemTime::now()` automatically when `score_point()` is called
- Expose timestamp data through the C FFI layer so Watch App can access point timing
- Add a new FFI function to retrieve the list of point events with timestamps
- **BREAKING**: `MatchWithHistory::score_point()` signature behavior changes — history entries now carry timestamps. Existing tests will need updating.

## Capabilities

### New Capabilities
- `point-timestamps`: Core engine records timestamps for each scored point, exposes them via Rust API and C FFI

### Modified Capabilities
- `game-history`: History entries gain timestamp data; undo preserves timestamp consistency

## Impact

- **Rust core**: `history.rs` — `MatchWithHistory` struct and `score_point()`/`undo()` methods
- **FFI**: `ffi.rs` — new struct and function to expose timestamped point list
- **C header**: `include/tennis_scorer.h` — new types and functions (cbindgen regeneration)
- **Watch App**: `TennisMatch.swift` — minor update to consume new FFI if needed
- **Tests**: All history-related tests need timestamp awareness
- **Dependencies**: None (uses `std::time::SystemTime`)
