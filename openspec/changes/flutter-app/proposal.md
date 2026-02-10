## Why

Apple Watch 螢幕太小，不適合查看詳細統計數據和圖表。需要一個 iPhone app 來瀏覽比賽紀錄、查看統計分析、和動量圖表。同時，目前 Watch app 使用 email/password 登入，在小螢幕上輸入很不方便，藉此機會一併改為 Sign in with Apple。

## What Changes

### 1. Flutter iPhone App（新專案）
- Sign in with Apple 登入
- 比賽列表（分頁瀏覽，顯示日期、比數、勝負）
- 比賽詳情 + 完整統計數據（break point、發球、deuce、連續得分、關鍵分、tiebreak）
- Momentum 圖表（支援 basic/weighted/per-set 切換）
- 唯讀工具，不做計分功能

### 2. 後端新增 Apple 認證（API 改動）
- 新增 `POST /api/auth/apple` endpoint
- 接收 Apple identity token，用 Apple 公鑰驗證
- 以 Apple User ID 查找或建立使用者
- 回傳標準 JWT（access + refresh token）

### 3. Watch App 改用 Sign in with Apple
- 登入畫面改為 Sign in with Apple（連按兩下側邊按鈕確認）
- 移除 email/password 登入 UI
- 同一個 Apple ID 在 Watch 和 iPhone app 對應同一個使用者

## Capabilities

### New Capabilities
- `apple-auth`: Sign in with Apple 認證流程（後端驗證 Apple identity token、建立/查找使用者）
- `flutter-match-list`: 比賽列表，分頁瀏覽歷史比賽紀錄
- `flutter-match-detail`: 比賽詳情頁，顯示完整統計分析數據
- `flutter-momentum-chart`: 互動式 Momentum 圖表，支援多種模式切換

### Modified Capabilities
- `watch-auth`: Watch app 登入方式從 email/password 改為 Sign in with Apple

## Impact

- **New project**: `flutter/` directory at repository root
- **Dependencies**: Flutter SDK, fl_chart, sign_in_with_apple, flutter_secure_storage, http
- **API changes**: 新增 `/api/auth/apple` endpoint，需加入 Apple JWT 驗證邏輯
- **Watch app changes**: `AuthView.swift` 改為 Sign in with Apple UI
- **Apple Developer**: App ID 需啟用 Sign in with Apple capability
