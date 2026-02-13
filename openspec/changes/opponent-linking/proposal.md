## Why

目前 matches 表沒有對手資訊欄位，無法將比賽關聯到特定對手。使用者無法記錄對手是誰，也無法讓對手認領（claim）比賽記錄。Opponent linking 是 Phase 1 後端演進的關鍵功能，為未來的社交功能（好友對戰記錄、head-to-head 統計）奠定基礎。

## What Changes

- 在 matches 表新增 opponent 相關欄位：`opponent_user_id`（FK→users）、`opponent_email`、`opponent_name`、`opponent_claim_token`（UNIQUE）
- 更新 `CreateMatchRequest` 新增可選欄位：`opponent_email`、`opponent_name`
- 更新 `MatchResponse` 回傳 opponent 資訊
- 新增 `POST /api/matches/claim` 端點，讓對手透過 claim token 認領比賽
- 實作自動配對（auto-match）：當使用者以 opponent_email 相同的 email 註冊時，系統自動連結
- 比賽建立時自動產生 claim_token（UUID）

## Capabilities

### New Capabilities

無新增 capability — 此變更修改現有的 `api-matches` capability。

### Modified Capabilities
- `api-matches`: 新增 opponent 欄位到比賽建立/回傳、新增 claim 端點、自動配對邏輯

## Impact

- **Database**: 新增 migration 為 matches 表加入 4 個 opponent 欄位
- **API Models**: `CreateMatchRequest` 和 `MatchResponse` 新增 opponent 欄位
- **Handlers**: `create_match` 處理 opponent 資訊與 claim token 產生；新增 `claim_match` handler
- **Routes**: `/matches/claim` 新路由
- **Existing queries**: `list_matches`、`get_match` 需更新以回傳 opponent 資訊
- **Core Engine**: 無變更（`tennis-scorer` crate 不受影響）
