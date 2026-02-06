## Why

The scoring engine currently only supports singles matches. In tennis, doubles is equally popular and follows the same game/set/match scoring rules, but with a critical difference: serve rotation cycles through all four players rather than alternating between two. Without doubles support, the engine cannot be used for doubles matches on the Apple Watch app.

Adding doubles is a natural extension because the core scoring logic (points, games, sets, tiebreaks) is identical. The main addition is tracking which of the four players is serving and rotating correctly across games and tiebreaks.

## What Changes

- Add a `MatchType` enum (Singles/Doubles) to `MatchConfig`
- Add serve order configuration for doubles (4-player rotation)
- Track current server position within `MatchState`
- Implement serve rotation on game boundaries (cycle through 4 players)
- Implement tiebreak serve rotation (changes every 2 points after the first)
- Expose serve order information through the FFI layer
- All existing scoring rules (game points, set wins, tiebreak logic) remain unchanged

## Capabilities

### New Capabilities

- `doubles-rules`: Serve rotation tracking and match type configuration for doubles matches

### Modified Capabilities

None. The core scoring logic (game, set, match, tiebreak) is unchanged for both singles and doubles.

## Impact

- **Rust core**: `src/config.rs` - Add `MatchType` enum and serve order fields to `MatchConfig`
- **Rust core**: `src/match_state.rs` - Add serve tracking state, rotate server on game/set transitions
- **Rust core**: `src/tiebreak.rs` or `src/match_state.rs` - Tiebreak-specific serve rotation (every 2 points)
- **FFI layer**: `src/ffi.rs` - New constructor for doubles, expose current server in `MatchScore`
- **Types**: `src/types.rs` - Player enum stays as Player1/Player2 (representing teams in doubles)
