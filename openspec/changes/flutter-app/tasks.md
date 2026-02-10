## 1. 後端 — Apple 認證

- [ ] 1.1 新增 DB migration：`apple_user_id TEXT UNIQUE` 欄位，`email` 和 `password_hash` 改為 nullable
- [ ] 1.2 新增 Apple JWKS 驗證模組（fetch Apple 公鑰、驗證 identity token signature/issuer/audience、快取機制）
- [ ] 1.3 新增 `POST /api/auth/apple` handler（驗證 token → 查找或建立使用者 → 回傳 JWT access + refresh token）
- [ ] 1.4 註冊新 route 到 Axum router
- [ ] 1.5 寫 API 整合測試（新使用者登入、重複登入、無效 token）

## 2. Watch App — Sign in with Apple

- [ ] 2.1 改寫 `AuthView.swift`：移除 email/password 表單，改為 Sign in with Apple button（`ASAuthorizationAppleIDButton`）
- [ ] 2.2 實作 `ASAuthorizationController` delegate 處理 Apple 認證回調
- [ ] 2.3 修改 `APIClient` 呼叫 `/api/auth/apple`（傳送 identity token，接收 JWT）
- [ ] 2.4 在 Xcode project 啟用 Sign in with Apple capability

## 3. Flutter — 專案初始化

- [ ] 3.1 建立 Flutter 專案（`flutter/` 目錄，iOS only）
- [ ] 3.2 加入依賴：`flutter_riverpod`、`fl_chart`、`sign_in_with_apple`、`flutter_secure_storage`、`http`、`go_router`
- [ ] 3.3 建立專案目錄結構（models、services、providers、screens、widgets）
- [ ] 3.4 設定 iOS Info.plist（Sign in with Apple capability）

## 4. Flutter — 認證

- [ ] 4.1 實作 `AuthService`：Sign in with Apple flow → 取得 identity token → 呼叫 `/api/auth/apple` → 儲存 JWT 到 secure storage
- [ ] 4.2 實作 `ApiClient`：附加 Bearer token、401 時自動 refresh、base URL 設定
- [ ] 4.3 實作 `AuthProvider`（Riverpod）：管理登入狀態、token 持久化、自動登入
- [ ] 4.4 建立 `LoginScreen`：Sign in with Apple 按鈕、錯誤提示

## 5. Flutter — 比賽列表

- [ ] 5.1 定義 `Match` model（對應 API response）
- [ ] 5.2 實作 `MatchRepository`：呼叫 `GET /api/matches` 含分頁參數
- [ ] 5.3 實作 `MatchListProvider`：分頁載入、pull-to-refresh
- [ ] 5.4 建立 `MatchListScreen`：顯示日期、比數、勝負指示、下拉刷新、滑到底載入更多、空狀態

## 6. Flutter — 比賽詳情與統計

- [ ] 6.1 定義 `MatchAnalysis` model（對應 `/stats/match/:id/analysis` response）
- [ ] 6.2 實作 `StatsRepository`：呼叫 analysis、momentum endpoints
- [ ] 6.3 實作 `MatchDetailProvider`：載入比賽資料 + 統計數據
- [ ] 6.4 建立 `MatchDetailScreen`：比賽 header（日期、比數、時長）+ 各項統計區塊（break point、發球、deuce、連續得分、關鍵分、tiebreak）

## 7. Flutter — Momentum 圖表

- [ ] 7.1 定義 `MomentumData` model（對應 `/stats/match/:id/momentum` response）
- [ ] 7.2 實作 `MomentumProvider`：載入動量數據
- [ ] 7.3 建立 `MomentumChartScreen`：fl_chart 折線圖、零線指示、模式切換（basic / weighted / per-set basic / per-set weighted）

## 8. Flutter — 路由與整合

- [ ] 8.1 設定 `GoRouter`：登入 → 比賽列表 → 比賽詳情 → Momentum 圖表
- [ ] 8.2 根據登入狀態自動導向（有 token → 列表，無 token → 登入）
- [ ] 8.3 整合測試：完整流程（登入 → 列表 → 詳情 → 圖表）
