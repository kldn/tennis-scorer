## Why

目前後端使用 Argon2 密碼雜湊 + 自訂 JWT（access/refresh token），但架構設計已決定全平台改用 Firebase Auth（Google + Apple Sign-In）。自訂 JWT 需要自行管理 token 過期、刷新、密碼儲存等安全問題；Firebase Auth 提供跨平台統一認證、與 FCM 整合、且無需自管密碼。此變更是 Phase 1 後端演進的最優先項目，所有其他模組（Social、Notification、Flutter App）都依賴此認證基礎。

## What Changes

- **BREAKING**: 移除 `POST /api/auth/register`、`/api/auth/login`、`/api/auth/refresh` 端點
- **BREAKING**: 移除 Argon2 密碼雜湊、jsonwebtoken JWT 產生/驗證
- 新增 `GET /api/auth/me`：以 Firebase ID Token 取得或建立使用者
- 新增 Firebase Token Verify middleware，取代自訂 JWT middleware
- Users 表：新增 `firebase_uid`（UNIQUE）、`display_name`、`avatar_url`；移除 `password_hash`
- AppState：移除 `jwt_secret`，新增 `firebase_project_id`
- AppConfig：移除 `jwt_secret`，新增 `firebase_project_id`
- 環境變數：移除 `JWT_SECRET`，新增 `FIREBASE_PROJECT_ID`

## Capabilities

### New Capabilities
- `firebase-auth`: Firebase Auth token 驗證與使用者管理（驗證 Firebase ID token、以 firebase_uid 查找/建立使用者、回傳使用者資料）

### Modified Capabilities
- `api-auth`: 認證端點從 register/login/refresh 改為 GET /api/auth/me；middleware 從自訂 JWT 改為 Firebase Token Verify

## Impact

- **Code**: 重寫 `src/auth/` 模組（handlers.rs, middleware.rs）；移除 `jwt.rs`；修改 `config.rs`、`lib.rs`、`main.rs`
- **Database**: 新增 migration 修改 users 表（加欄位、移除 password_hash）
- **Dependencies**: 移除 `argon2`、`jsonwebtoken`；新增 `reqwest`（Firebase token 驗證用）或 `jsonwebtoken` + `reqwest`（JWKS 驗證）
- **Environment**: 移除 `JWT_SECRET`，新增 `FIREBASE_PROJECT_ID`
- **Existing tests**: 整合測試需全面改寫（認證流程完全不同）
- **Watch App**: `APIClient.swift` 需改為傳送 Firebase ID Token
- **Breaking**: 所有現有使用者帳號將無法使用（需要重新以 Firebase 登入）
