## 1. Service Layer 基礎

- [ ] 1.1 在 Cargo.toml 新增 `tower` dependency（rate limiting 用）
- [ ] 1.2 修改 AppState：新增 AuthService、MatchService、StatsService 欄位

## 2. AuthService 提取

- [ ] 2.1 建立 `src/auth/service.rs`：定義 AuthService struct（持有 PgPool）
- [ ] 2.2 提取 register 邏輯至 `AuthService::register()`
- [ ] 2.3 提取 login 邏輯至 `AuthService::login()`
- [ ] 2.4 提取 refresh 邏輯至 `AuthService::refresh()`
- [ ] 2.5 修改 auth handlers 為薄層：解析 request → call service → return response
- [ ] 2.6 在 auth/mod.rs 新增 `pub mod service;`

## 3. MatchService 提取

- [ ] 3.1 建立 `src/matches/service.rs`：定義 MatchService struct
- [ ] 3.2 提取 create_match 邏輯（含交易）至 `MatchService::create()`
- [ ] 3.3 提取 list_matches 邏輯至 `MatchService::list()`
- [ ] 3.4 提取 get_match 邏輯至 `MatchService::get()`
- [ ] 3.5 提取 delete_match 邏輯至 `MatchService::delete()`
- [ ] 3.6 修改 match handlers 為薄層
- [ ] 3.7 在 matches/mod.rs 新增 `pub mod service;`

## 4. StatsService 提取

- [ ] 4.1 建立 `src/stats/service.rs`：定義 StatsService struct
- [ ] 4.2 提取 summary 邏輯至 `StatsService::summary()`
- [ ] 4.3 提取 match_analysis 邏輯至 `StatsService::match_analysis()`
- [ ] 4.4 提取 match_momentum 邏輯至 `StatsService::match_momentum()`
- [ ] 4.5 提取 match_pace 邏輯至 `StatsService::match_pace()`
- [ ] 4.6 修改 stats handlers 為薄層
- [ ] 4.7 在 stats/mod.rs 新增 `pub mod service;`

## 5. Rate Limiting

- [ ] 5.1 在 `lib.rs` 的 create_router 中新增 `tower::limit::RateLimitLayer`
- [ ] 5.2 設定全局速率限制（100 requests/second）
- [ ] 5.3 驗證超限時回傳 429 Too Many Requests

## 6. 整合與驗證

- [ ] 6.1 更新 lib.rs：在 AppState 建立並注入所有 services
- [ ] 6.2 執行現有整合測試，確認全部通過（外部行為不變）
- [ ] 6.3 驗證 cargo clippy 和 cargo fmt 通過
