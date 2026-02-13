## 1. Database Migration

- [ ] 1.1 建立 migration：CREATE TABLE device_tokens（id, user_id, fcm_token, device_type, created_at, updated_at, UNIQUE(user_id, device_type)）
- [ ] 1.2 device_type CHECK constraint：'watch_apple', 'watch_wearos', 'phone_android', 'phone_ios'

## 2. Models

- [ ] 2.1 建立 `src/notifications/models.rs`：RegisterTokenRequest（fcm_token, device_type）
- [ ] 2.2 NotificationSettings（friend_requests: bool, match_results: bool, match_claims: bool）
- [ ] 2.3 內部 FCM message structs（FCMMessage, FCMNotification, FCMData）

## 3. NotificationService

- [ ] 3.1 建立 `src/notifications/service.rs`：NotificationService struct（持有 PgPool + reqwest::Client）
- [ ] 3.2 `register_token(user_id, fcm_token, device_type)`：UPSERT device_tokens
- [ ] 3.3 `get_settings(user_id)` / `update_settings(user_id, settings)`
- [ ] 3.4 `send_push(user_id, title, body, data)`：查詢使用者所有 device tokens → 呼叫 FCM API
- [ ] 3.5 FCM HTTP v1 API 呼叫（OAuth 2.0 access token 管理 + 快取）
- [ ] 3.6 `notify_friend_request(to_user_id, from_user_name)`
- [ ] 3.7 `notify_match_result(friend_user_ids, match_summary)`
- [ ] 3.8 `notify_match_claim(user_id, opponent_name)`
- [ ] 3.9 處理 FCM 404 回應：刪除無效 token

## 4. Handlers

- [ ] 4.1 建立 `src/notifications/handlers.rs`
- [ ] 4.2 `register_device`：POST /api/notifications/register
- [ ] 4.3 `update_settings`：PUT /api/notifications/settings

## 5. Route 註冊

- [ ] 5.1 建立 `src/notifications/mod.rs`：定義 routes()
- [ ] 5.2 在 `lib.rs` 新增 `pub mod notifications;` 和 `.merge(notifications::routes())`
- [ ] 5.3 在 AppState 新增 NotificationService

## 6. 環境設定

- [ ] 6.1 新增 FIREBASE_PROJECT_ID 和 FIREBASE_SERVICE_KEY 環境變數讀取
- [ ] 6.2 在 Railway 設定對應 secrets
- [ ] 6.3 更新 deploy.yml 如有需要

## 7. 測試

- [ ] 7.1 測試 device token 註冊（新增 + 更新）
- [ ] 7.2 測試 settings 更新與讀取
- [ ] 7.3 Mock FCM API 測試推播發送
- [ ] 7.4 測試無效 token 清理
