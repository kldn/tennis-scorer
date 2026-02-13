## Context

Tennis Scorer 後端目前使用自訂認證系統：Argon2 密碼雜湊 + jsonwebtoken JWT（access/refresh token）。架構設計文件決定遷移至 Firebase Auth，以統一跨平台認證（Apple Watch、Wear OS、Phone App）並與 FCM 推播通知整合。

現有認證相關程式碼：
- `src/auth/handlers.rs`：register、login、refresh handlers
- `src/auth/jwt.rs`：JWT 產生與驗證
- `src/auth/middleware.rs`：AuthUser extractor（驗證 JWT）
- `src/config.rs`：AppConfig 含 jwt_secret
- `src/lib.rs`：AppState 含 jwt_secret
- `migrations/001_create_users.sql`：users 表（email, password_hash）

## Goals / Non-Goals

**Goals:**
- 以 Firebase Auth token 驗證取代自訂 JWT
- 單一 `GET /api/auth/me` 端點取代 register/login/refresh
- Users 表新增 firebase_uid、display_name、avatar_url
- AuthUser middleware 改為驗證 Firebase ID token

**Non-Goals:**
- 不實作使用者資料遷移（現有帳號需重新登入）
- 不實作 Firebase Admin SDK（使用 HTTP API 驗證即可）
- 不處理前端 Firebase SDK 整合（屬 flutter-app change 範疇）

## Decisions

### D1: Firebase token 驗證方式

**選擇**: 使用 Google 公開的 JWKS endpoint 驗證 Firebase ID Token（RS256 JWT）

**替代方案**:
- Firebase Admin SDK（Rust 無官方 SDK，需用第三方或自行封裝）→ 維護成本高
- 每次 call Firebase REST API 驗證 → 增加延遲、有 rate limit 風險

**理由**: Firebase ID Token 是標準 RS256 JWT，可直接用 `jsonwebtoken` crate + Google JWKS 公鑰驗證。無需額外 HTTP 呼叫（快取 JWKS 即可），且無第三方 SDK 依賴。

### D2: Users 表遷移策略

**選擇**: 新增 migration 加入 firebase_uid/display_name/avatar_url 欄位，移除 password_hash

**替代方案**:
- 建立全新 users 表 → 需處理 matches 等 FK 遷移，風險高
- 保留 password_hash 允許雙模式 → 增加複雜度，無長期價值

**理由**: ALTER TABLE 最簡單，現有 FK 關係不受影響。password_hash 設為 nullable 後可移除。

### D3: AuthUser extractor 設計

**選擇**: 維持現有 `FromRequestParts<AppState>` trait 實作，內部改為驗證 Firebase token

**理由**: 保持 handler 簽名不變（`AuthUser` 作為 extractor），最小化對現有程式碼的影響。

## Risks / Trade-offs

**現有使用者帳號失效** → 可接受，目前為開發階段，無正式用戶。遷移後需以 Firebase 重新登入。

**JWKS 快取失效** → Google JWKS 有 `max-age` header，實作快取刷新機制（每小時刷新一次）。

**Firebase 服務中斷** → 低風險，Firebase Auth SLA 99.95%。本地 JWKS 快取可在短暫中斷時持續驗證。
