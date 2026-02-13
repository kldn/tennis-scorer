## 1. Database Migration

- [ ] 1.1 建立 migration：ALTER TABLE users ADD COLUMN firebase_uid TEXT UNIQUE
- [ ] 1.2 建立 migration：ALTER TABLE users ADD COLUMN display_name TEXT, ADD COLUMN avatar_url TEXT
- [ ] 1.3 建立 migration：ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL
- [ ] 1.4 驗證 migration 可正確執行（cargo test 觸發 sqlx::migrate!）

## 2. Firebase Token 驗證模組

- [ ] 2.1 新增 `src/auth/firebase.rs`：Google JWKS 公鑰獲取與快取（TTL 1 小時）
- [ ] 2.2 實作 `verify_firebase_token(token: &str) -> Result<FirebaseClaims>`：RS256 JWT 驗證
- [ ] 2.3 FirebaseClaims struct：sub (firebase_uid), email, name, picture
- [ ] 2.4 錯誤處理：過期 token、無效簽名、錯誤 issuer/audience

## 3. Auth Handler 重寫

- [ ] 3.1 移除 `handlers.rs` 中的 register、login、refresh handlers
- [ ] 3.2 新增 `get_or_create_me` handler：驗證 Firebase token → 以 firebase_uid 查找/建立使用者
- [ ] 3.3 回傳使用者資料（id, email, display_name, avatar_url）
- [ ] 3.4 首次建立時觸發 opponent auto-match（如 opponent-linking 已完成）

## 4. Middleware 重寫

- [ ] 4.1 修改 `middleware.rs`：AuthUser extractor 改為呼叫 verify_firebase_token
- [ ] 4.2 從 token claims 中提取 firebase_uid → 查詢 users 表取得 user_id
- [ ] 4.3 移除 token_type 檢查（Firebase token 不分 access/refresh）

## 5. Config 與 AppState 更新

- [ ] 5.1 修改 `config.rs`：移除 jwt_secret，新增 firebase_project_id
- [ ] 5.2 修改 `lib.rs`：AppState 移除 jwt_secret，新增 firebase_project_id
- [ ] 5.3 修改 `main.rs`：環境變數改為 FIREBASE_PROJECT_ID

## 6. Route 更新

- [ ] 6.1 修改 `auth/mod.rs`：routes 改為 GET /auth/me
- [ ] 6.2 移除 register、login、refresh route 定義

## 7. 依賴清理

- [ ] 7.1 從 Cargo.toml 移除 `argon2` dependency
- [ ] 7.2 確認 `jsonwebtoken` 是否保留（Firebase JWKS 驗證可能仍需）或替換為 `jwt-simple`
- [ ] 7.3 新增 `reqwest` dependency（用於獲取 Google JWKS）
- [ ] 7.4 刪除 `src/auth/jwt.rs` 檔案

## 8. 測試更新

- [ ] 8.1 移除現有 auth 相關整合測試（register/login/refresh）
- [ ] 8.2 新增 Firebase token 驗證 unit tests（mock JWKS）
- [ ] 8.3 新增 GET /api/auth/me 整合測試
- [ ] 8.4 更新其他整合測試中的 auth 流程（改用 Firebase token）
