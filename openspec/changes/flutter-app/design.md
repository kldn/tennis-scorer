## Context

tennis-scorer 由 Rust core library、Axum REST API（Railway 部署）、watchOS app 組成。Watch app 目前用 email/password 登入，在小螢幕上輸入不便。API 已提供 match CRUD、統計分析、momentum、pace endpoints。

需要新增 Flutter iPhone app 作為唯讀比賽分析工具，同時將認證統一改為 Sign in with Apple。

現有 users 表結構：
```sql
users (id UUID PK, email TEXT UNIQUE NOT NULL, password_hash TEXT NOT NULL, created_at TIMESTAMPTZ)
```

## Goals / Non-Goals

**Goals:**
- 後端新增 Sign in with Apple 認證 endpoint
- Watch app 改用 Sign in with Apple 登入
- Flutter iPhone app：Sign in with Apple 登入、比賽列表、完整統計詳情、momentum 圖表
- 同一 Apple ID 在兩端對應同一使用者

**Non-Goals:**
- Flutter app 不做計分功能（唯讀）
- 不支援 Android / Web（第一版僅 iPhone）
- 不移除既有 email/password API endpoints（保留向後相容）
- 不做 head-to-head 比較或趨勢分析（後續版本）

## Decisions

### D1: Apple identity token 後端驗證

**選擇**: 用 Apple JWKS 公鑰驗證 identity token

**替代方案**:
- Apple REST API `POST /auth/token` 驗證 authorization code → 多一次網路請求、需管理 client secret
- 信任 client 傳來的 user info → 不安全

**理由**: JWKS 驗證是無狀態的，公鑰可快取。用 `jsonwebtoken` crate 搭配 JWK set 解碼，與現有 JWT 基礎設施一致。

### D2: Users 表 schema 變更

**選擇**: 新增 `apple_user_id TEXT UNIQUE` 欄位，`email` 和 `password_hash` 改為 nullable

```sql
ALTER TABLE users
  ADD COLUMN apple_user_id TEXT UNIQUE,
  ALTER COLUMN email DROP NOT NULL,
  ALTER COLUMN password_hash DROP NOT NULL;
```

**替代方案**:
- 建獨立的 `apple_users` 表 → 查詢分散，token 發放邏輯要分兩路
- 用 email 關聯既有帳號 → Apple 使用者可能隱藏 email，不可靠

**理由**: 單表最簡潔。Apple 使用者只需 `apple_user_id`，既有 email 使用者不受影響。

### D3: Flutter 狀態管理 — Riverpod

**替代方案**:
- Provider → 維護停滯，官方推薦遷移到 Riverpod
- Bloc → boilerplate 多，對這個簡單 app 過度

**理由**: 型別安全、支援 async providers、不依賴 BuildContext，適合以 API 資料展示為主的 app。

### D4: 圖表套件 — fl_chart

**理由**: Flutter 最成熟的圖表庫，支援折線圖，自訂性高，活躍維護。

### D5: Token 儲存 — flutter_secure_storage

**理由**: 底層是 iOS Keychain，與 Watch app 做法一致，安全性符合 JWT 儲存需求。

### D6: Watch app 認證遷移

**選擇**: AuthView 改為 Sign in with Apple，用 `AuthenticationServices` framework

**理由**: watchOS 6.2+ 原生支援，使用者連按兩下側邊按鈕確認即可，體驗遠優於打字。登入後拿到 identity token，打相同的 `/api/auth/apple` endpoint。

### D7: Flutter 專案結構

```
flutter/
├── lib/
│   ├── models/         # 資料模型
│   ├── services/       # API client, auth service
│   ├── providers/      # Riverpod providers
│   ├── screens/        # 頁面（login, match_list, match_detail, momentum）
│   ├── widgets/        # 可重用元件
│   └── main.dart
├── ios/
└── pubspec.yaml
```

**理由**: 分層清晰，適合這個規模的 app。

## Risks / Trade-offs

**[Apple 隱藏 email]** → 用 `apple_user_id` 識別使用者，不依賴 email。email 欄位 nullable。

**[既有 email 使用者資料]** → 改用 Apple 登入後，舊帳號資料不會自動遷移。目前 Watch 使用量小，可接受。未來可加帳號綁定。

**[Apple JWKS 快取]** → Apple 公鑰偶爾輪替。實作快取 + 失效時重新拉取機制。

**[Flutter iPhone only]** → 第一版只測 iPhone，日後擴展到其他平台成本低。
