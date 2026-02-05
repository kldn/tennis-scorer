## Why

目前系統只追蹤當前 game 是否處於 deuce 狀態，但不記錄一個 game 中進入 deuce 的次數。追蹤 deuce 次數可以讓使用者了解每個 game 的激烈程度，這是網球比賽中有趣的統計數據。

## What Changes

- 在 Rust 核心新增 deuce 計數邏輯
- 每當 game 進入 deuce 狀態時，計數器加 1
- 透過 FFI 暴露 deuce 計數
- 在 watchOS UI 顯示當前 game 的 deuce 次數

## Capabilities

### New Capabilities

- `deuce-tracking`: 追蹤每個 game 中進入 deuce 的次數

### Modified Capabilities

無。

## Impact

- **Rust 核心**：`src/game.rs` - 新增 deuce_count 欄位
- **FFI 層**：`src/ffi.rs` - MatchScore 結構新增 deuce_count 欄位
- **Swift 層**：`TennisMatch.swift` - Score 結構新增 deuceCount
- **UI 層**：`ContentView.swift` - 顯示 deuce 次數
