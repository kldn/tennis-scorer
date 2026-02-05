## 1. Rust Core Changes

- [x] 1.1 Modify `GameState::Deuce` to `Deuce { count: u8 }` in src/game.rs
- [x] 1.2 Update `GameState::score_point()` to handle deuce count logic
- [x] 1.3 Update all pattern matches on `GameState::Deuce` throughout codebase
- [x] 1.4 Add unit tests for deuce counting in src/game.rs

## 2. FFI Layer Changes

- [x] 2.1 Add `deuce_count: u8` field to `MatchScore` struct in src/ffi.rs
- [x] 2.2 Update `extract_game_info()` to extract deuce count
- [x] 2.3 Add FFI tests for deuce count

## 3. Swift Layer Changes

- [x] 3.1 Add `deuceCount: Int` to `Score` struct in TennisMatch.swift
- [x] 3.2 Update `updateScore()` to read deuce count from FFI

## 4. UI Changes

- [x] 4.1 Update points display to show deuce count when in deuce state in ContentView.swift
- [x] 4.2 Format as "Deuce (N)" where N is the count

## 5. Build & Verify

- [x] 5.1 Run `cargo test` to verify Rust changes
- [x] 5.2 Build watchOS app to verify Swift changes compile
- [x] 5.3 Manual test: verify deuce count increments correctly
