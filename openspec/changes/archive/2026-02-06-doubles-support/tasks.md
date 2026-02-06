## 1. Config Changes

- [x] 1.1 Add `MatchType` enum (`Singles`, `Doubles`) with Serialize/Deserialize to `src/config.rs`
- [x] 1.2 Add `match_type: MatchType` field to `MatchConfig` (default: Singles)
- [x] 1.3 Add `serve_order: Vec<(Player, u8)>` field to `MatchConfig` (default: empty)
- [x] 1.4 Add unit tests for MatchConfig with doubles settings

## 2. Serve Tracking State

- [x] 2.1 Add `serve_rotation_index: usize` field to `MatchState::Playing`
- [x] 2.2 Add `tiebreak_serve_index: usize` and `tiebreak_points_served: u8` fields to `MatchState::Playing`
- [x] 2.3 Update `MatchState::new()` to initialize serve tracking fields (index 0)
- [x] 2.4 Update all existing `MatchState::Playing` pattern matches to include new fields

## 3. Serve Rotation Logic

- [x] 3.1 Implement server advancement on game completion in `MatchState::score_point()`
- [x] 3.2 Implement tiebreak serve rotation (1 point for first server, then 2 points each)
- [x] 3.3 Implement serve index restoration after tiebreak ends
- [x] 3.4 Add `current_server()` query method to `MatchState`
- [x] 3.5 Add unit tests for serve rotation across games within a set
- [x] 3.6 Add unit tests for serve rotation across set boundaries
- [x] 3.7 Add unit tests for tiebreak serve rotation pattern
- [x] 3.8 Add unit tests verifying singles matches are unaffected by serve tracking

## 4. FFI Changes

- [x] 4.1 Add `current_server: u8` field to `MatchScore` struct in `src/ffi.rs`
- [x] 4.2 Add `tennis_match_new_doubles()` extern function
- [x] 4.3 Update `build_match_score()` to populate `current_server` from serve rotation state
- [x] 4.4 Add FFI tests for doubles match creation and server querying

## 5. Integration Tests

- [x] 5.1 Add integration test: full doubles match with serve rotation verification
- [x] 5.2 Add integration test: doubles tiebreak serve rotation
- [x] 5.3 Add integration test: undo preserves/restores serve rotation state
- [x] 5.4 Run `cargo test` to verify all existing tests still pass
