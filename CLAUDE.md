# Tennis Scorer 專案說明

## 專案概述

Tennis Scorer 是一個跨平台的網球計分應用程式，支援 Apple Watch、Wear OS 手錶和手機應用。

**核心理念**：
- **手錶是計分裝置**：內嵌 Rust 引擎，離線優先，比賽後同步到雲端
- **手機是查看裝置**：純 API 客戶端，處理統計、歷史和社交功能
- **後端是資料中心**：儲存、分析計算、社交功能、推播通知

完整架構設計文件：`docs/plans/2026-02-13-system-architecture-design.md`

---

## 技術棧

### 前端
- **Apple Watch**: Swift/SwiftUI + UniFFI (Rust 引擎綁定)
- **Wear OS Watch**: Flutter + dart:ffi (Rust 引擎綁定)
- **Phone App**: Flutter (純 API 客戶端，無 Rust 引擎)

### 後端
- **API Server**: Rust + Axum
- **Database**: PostgreSQL
- **Authentication**: Firebase Auth (Google + Apple Sign-In)
- **Push Notifications**: Firebase Cloud Messaging (FCM)
- **Deployment**: Railway

### 共享核心
- **Scoring Engine**: Rust crate (`tennis-scorer`)，供手錶 app 和後端使用

---

## 專案結構

```
tennis-scorer/
├── crates/
│   ├── tennis-scorer/       # 核心計分引擎 (共享)
│   └── tennis-scorer-api/   # Rust Axum 後端
├── WatchApp/                # Apple Watch app (Swift/SwiftUI)
├── flutter/
│   ├── phone_app/           # 手機應用 (Flutter)
│   └── wearos_app/          # Wear OS 手錶應用 (Flutter)
├── docs/
│   └── plans/               # 架構設計文件
└── .github/workflows/       # CI/CD pipelines
```

---

## 核心設計決策

參考完整決策表：`docs/plans/2026-02-13-system-architecture-design.md` 第 5-16 行

### 重點摘要
1. **資料同步**：手錶直接同步到雲端（離線優先 + 上線時同步）
2. **手機資料來源**：純雲端 API（資料小 ~3KB/match，網路延遲可接受）
3. **身份驗證**：Firebase Auth 取代自訂 JWT，跨平台、與 FCM 整合
4. **社交功能**：好友系統、比賽歷史、對戰記錄
5. **對手連結**：Claim token + 自動郵件配對（支援未註冊對手）

---

## 資料庫架構

### 核心表
- `users`: 使用者資料（`firebase_uid`, `email`, `display_name`, `avatar_url`）
- `matches`: 比賽記錄（包含 `opponent_user_id`, `opponent_email`, `opponent_claim_token`）
- `match_events`: 比賽事件序列

### 社交功能表（新增）
- `friend_requests`: 好友請求（`from_user_id`, `to_user_id`, `status`）
- `friendships`: 好友關係（雙向儲存）

### 推播通知表（新增）
- `device_tokens`: FCM 裝置 token（支援 watch_apple/watch_wearos/phone_android/phone_ios）

參考完整 schema：`docs/plans/2026-02-13-system-architecture-design.md` 第 278-338 行

---

## API 端點

參考完整 API 列表：`docs/plans/2026-02-13-system-architecture-design.md` 第 443-464 行

### 主要端點
- `GET /api/auth/me` - 取得/建立當前使用者
- `POST /api/matches` - 建立比賽（含對手資訊）
- `POST /api/matches/claim` - 認領比賽為對手
- `GET /api/stats/*` - 統計資料（摘要、分析、momentum、pace）
- `POST /api/social/friend-request` - 發送好友請求
- `POST /api/notifications/register` - 註冊裝置 FCM token

---

## 開發工作流程

### CI/CD Pipelines
- `rust.yml`: Rust 程式碼檢查（fmt, clippy, test, build）
- `deploy.yml`: 部署到 Railway（master push 時觸發）
- `watchos.yml`: watchOS app 建置與測試
- `flutter.yml`: Flutter app 建置與測試（待新增）

### 環境變數
```bash
# Railway
DATABASE_URL          # Railway managed PostgreSQL
FIREBASE_PROJECT_ID   # Firebase 專案 ID
HOST=0.0.0.0
PORT=8000

# GitHub Secrets
RAILWAY_TOKEN         # Railway 部署 token
FIREBASE_SERVICE_KEY  # Firebase Admin SDK key
```

---

## 架構原則

1. **離線優先**：手錶 app 可離線計分，上線後同步
2. **關注點分離**：手錶負責計分、手機負責查看與社交、後端負責儲存與分析
3. **跨平台共享**：Rust 引擎在手錶和後端共享，確保計分邏輯一致
4. **安全第一**：Firebase Auth 管理身份驗證，無需自訂 JWT
5. **簡化手機端**：手機不包含 Rust 引擎，保持應用輕量

---

## 遷移路徑

### Phase 1: 後端演進
1. 將 Argon2 + 自訂 JWT 替換為 Firebase Auth token 驗證
2. 在 matches 表新增 opponent 欄位
3. 從 handlers 提取 Service Layer
4. 新增 match claim 端點

### Phase 2: 社交與通知
5. 新增 friend_requests、friendships、device_tokens 表
6. 實作 Social 模組（好友 CRUD、對戰記錄）
7. 整合 FCM 推播通知
8. 實作 Notification 模組

### Phase 3: 跨平台前端
9. 建立 Flutter 手機應用（純 API 客戶端）
10. 建立 Flutter Wear OS 手錶應用（含 Rust 引擎）
11. 新增 flutter.yml CI pipeline
12. 設定應用程式發布（Google Play、App Store）

---

## 相關文件

- 完整架構設計：`docs/plans/2026-02-13-system-architecture-design.md`
- API 文件：（待建立）
- Rust 引擎文件：`crates/tennis-scorer/README.md`（待建立）
