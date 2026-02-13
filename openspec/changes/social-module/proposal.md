## Why

目前 Tennis Scorer 後端僅支援個人比賽記錄，沒有任何社交功能。使用者無法新增好友、查看好友的比賽記錄，或查看與特定對手的對戰統計。社交功能是手機端應用的核心價值之一，讓使用者能追蹤朋友的比賽、比較對戰記錄，提升應用的黏著度。

## What Changes

- 新增 `friend_requests` 和 `friendships` 資料庫表（含 migration）
- 建立 `social` 模組（handlers、models、service）
- 實作好友請求發送、接受、拒絕流程
- 實作好友清單、好友比賽記錄、對戰統計（head-to-head）API
- 在 `lib.rs` 中註冊 social 路由到 `/api/social/*`

## Capabilities

### New Capabilities
- `social-friend-requests`: 好友請求系統——發送、接受、拒絕，含重複/自我請求防止
- `social-friendships`: 好友關係管理——雙向儲存、好友清單查詢、解除好友
- `social-head-to-head`: 對戰記錄——根據 opponent_user_id 計算兩人勝負統計

### Modified Capabilities
- `api-infra`: `lib.rs` 新增 `.merge(social::routes())` 路由註冊

## Impact

- **Code**: 新增 `crates/tennis-scorer-api/src/social/` 模組
- **Database**: 新增 migration 建立 friend_requests 和 friendships 表
- **Dependencies**: 無新外部依賴
- **API**: 新增 5 個端點於 `/api/social/*`
- **Prerequisites**: 依賴 `service-layer-extraction`（Service Layer 模式）；依賴 `opponent-linking`（head-to-head 需 opponent_user_id）
