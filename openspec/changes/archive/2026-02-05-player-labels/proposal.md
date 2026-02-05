## Why

目前 UI 顯示「P1」和「P2」作為玩家標籤，對於個人使用場景不夠直覺。將標籤改為「我」和「對手」可以更清楚地區分自己的得分，提升使用體驗。

## What Changes

- 將所有 UI 中的「P1」標籤改為「我」
- 將所有 UI 中的「P2」標籤改為「對手」
- 勝利訊息從「P1 Wins!」/「P2 Wins!」改為「我贏了！」/「對手贏了」
- Advantage 顯示從「Ad - 40」/「40 - Ad」改為「Ad - 40」/「40 - Ad」（保持不變，因為這是分數顯示而非標籤）

## Capabilities

### New Capabilities

無新增能力。

### Modified Capabilities

無需修改 specs，這是純 UI 文字標籤變更，不影響計分邏輯。

## Impact

- **UI 層**：`WatchApp/TennisScorer Watch App/ContentView.swift` - 按鈕標籤和勝利訊息
- **無 API 變更**：Rust FFI 層保持使用 `PLAYER_1`/`PLAYER_2` 常數
- **無邏輯變更**：計分引擎不受影響
