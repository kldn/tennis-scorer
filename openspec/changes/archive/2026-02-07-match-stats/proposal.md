## Why

使用者想要看到自己的對戰成績統計，包括總共贏了幾場、輸了幾場。這可以幫助追蹤進步情況和與特定對手的歷史戰績。

## What Changes

- 基於 game-history 功能，計算並顯示統計數據
- 顯示總勝場數和總敗場數
- 顯示勝率

## Capabilities

### New Capabilities

- `match-statistics`: 計算和顯示比賽統計數據

### Modified Capabilities

無。

## Impact

- **Swift 層**：新增 `MatchStats` 計算邏輯
- **UI 層**：新增統計顯示區域
- **依賴**：需要 game-history 功能先完成
