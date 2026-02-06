## 1. Core Engine — Timestamp Storage

- [x] 1.1 Add `point_events: Vec<(Player, SystemTime)>` field to `MatchWithHistory` struct in `history.rs`
- [x] 1.2 Update `MatchWithHistory::new()` to initialize `point_events` as empty vec
- [x] 1.3 Update `score_point()` to push `(scorer, SystemTime::now())` onto `point_events` when a point is successfully scored
- [x] 1.4 Update `undo()` to pop the last entry from `point_events`
- [x] 1.5 Add debug assertion `assert_eq!(history.len(), point_events.len())` in both `score_point()` and `undo()`
- [x] 1.6 Add public accessor `pub fn point_events(&self) -> &[(Player, SystemTime)]`

## 2. Core Engine — Tests

- [x] 2.1 Test that `score_point()` adds a timestamp entry with the current time (assert within a time range)
- [x] 2.2 Test that `undo()` removes the corresponding timestamp entry
- [x] 2.3 Test that `point_events` length matches `history_len()` after a sequence of score/undo operations
- [x] 2.4 Test that re-scoring after undo records a new timestamp (not the old one)
- [x] 2.5 Test that scoring on a completed match does not add a timestamp entry
- [x] 2.6 Verify existing history tests still pass without modification

## 3. FFI — Expose Timestamps

- [x] 3.1 Add `#[repr(C)] struct PointEvent { player: u8, timestamp: f64 }` to `ffi.rs`
- [x] 3.2 Add FFI function `tennis_match_get_point_count(match_ptr) -> u32` that returns the number of point events
- [x] 3.3 Add FFI function `tennis_match_get_points(match_ptr, buffer, buffer_size) -> bool` that fills a caller-provided `PointEvent` buffer
- [x] 3.4 Add helper to convert `SystemTime` to `f64` Unix epoch seconds

## 4. FFI — Tests

- [x] 4.1 Test `get_point_count` returns 0 for new match
- [x] 4.2 Test `get_point_count` returns correct count after scoring points
- [x] 4.3 Test `get_points` fills buffer with correct player and valid timestamp values
- [x] 4.4 Test `get_points` reflects undo (count decreases, buffer shorter)
- [x] 4.5 Test null pointer safety for both new FFI functions

## 5. C Header — Regenerate

- [x] 5.1 Run `cbindgen` to regenerate `include/tennis_scorer.h`
- [x] 5.2 Verify new `PointEvent` struct and functions appear in the header
