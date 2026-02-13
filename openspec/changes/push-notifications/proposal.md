## Why

Tennis Scorer 目前沒有推播通知功能。當使用者收到好友請求、好友完成比賽、或對手認領比賽時，只有主動開啟 app 才會知道。FCM 推播通知可以讓使用者即時收到重要事件提醒，提升社交互動的即時性。

## What Changes

- 建立 `device_tokens` 資料庫表，儲存 FCM device token 和裝置類型
- 新增 `POST /api/notifications/register`：註冊裝置 FCM token
- 新增 `PUT /api/notifications/settings`：通知偏好設定
- 建立 `NotificationService`，封裝 FCM HTTP v1 API 呼叫
- 實作三種通知觸發：好友請求、好友比賽結果、比賽認領
- 新增 `reqwest` 依賴用於 FCM HTTP 呼叫

## Capabilities

### New Capabilities
- `notification-token-registration`: FCM token 註冊與管理，支援多裝置類型
- `notification-delivery`: 透過 FCM HTTP v1 API 發送推播通知
- `notification-triggers`: 事件驅動通知觸發（好友請求、比賽結果、比賽認領）

### Modified Capabilities
- `api-infra`: 新增 `device_tokens` migration、notification routes

## Impact

- **Code**: 新增 `src/notifications/` 模組
- **Dependencies**: 新增 `reqwest`（FCM HTTP v1 API）
- **Database**: 新增 `device_tokens` 表
- **Infrastructure**: 需要 `FIREBASE_PROJECT_ID` 和 `FIREBASE_SERVICE_KEY` 環境變數
- **Integration**: Social/Match 模組需呼叫 NotificationService
