## Why

目前系統只支援一場比賽，比賽結束後按「New Match」會直接重置，無法保留歷史記錄。使用者希望能夠儲存已完成的比賽，並在之後查看歷史對戰記錄。

## What Changes

- 新增 match 歷史儲存機制（使用本地持久化）
- 開始新 match 時，自動儲存當前已完成的 match
- 提供查詢歷史 match 的介面

## Capabilities

### New Capabilities

- `match-persistence`: 持久化儲存已完成的比賽記錄

### Modified Capabilities

無。

## Impact

- **Swift 層**：新增 `MatchHistory` 類別處理持久化
- **Swift 層**：`TennisMatch.swift` - `newMatch()` 需在重置前儲存
- **UI 層**：新增歷史記錄查看功能（基礎版本，可後續擴充）
- **資料格式**：使用 JSON 儲存在 UserDefaults 或 Documents 目錄
