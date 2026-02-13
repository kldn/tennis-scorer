## Context

tennis-scorer 由 Rust core library、Axum REST API（Railway 部署）、watchOS app 組成。根據架構設計文件，需建立 Flutter Phone App（純 API 客戶端）和 Flutter Wear OS Watch App（含 Rust 引擎）。全平台改用 Firebase Auth（Google + Apple Sign-In），取代自訂 JWT。

現有 users 表結構：
```sql
users (id UUID PK, email TEXT UNIQUE NOT NULL, password_hash TEXT NOT NULL, created_at TIMESTAMPTZ)
```

## Goals / Non-Goals

**Goals:**
- Flutter Phone App：Firebase Auth 登入、6 個主要畫面、FCM 推播、4 層架構
- Flutter Wear OS Watch App：Rust 引擎 + 離線計分 + 雲端同步
- Watch App 改用 Firebase Auth（Apple Sign-In）
- flutter.yml CI pipeline
- 同一 Firebase UID 在所有平台對應同一使用者

**Non-Goals:**
- Phone App 不做計分功能（唯讀）
- 不處理應用商店上架流程（Phase 3）
- 不移除既有 email/password API endpoints（由 firebase-auth-migration change 處理）
- Wear OS app 第一版不含語音輸入（後續加入）

## Decisions

### D1: Phone App 架構 — 4 層

```
UI (Widgets) → State Management (Riverpod) → Repository → Network (Dio)
```

**理由**: 職責分離清晰，Repository 隔離 API 細節，方便加入本地快取。

### D2: 狀態管理 — Riverpod

**替代方案**:
- Provider → 維護停滯，官方推薦遷移到 Riverpod
- Bloc → boilerplate 多
- GetX → 社群爭議大

**理由**: 型別安全、支援 async providers、不依賴 BuildContext，Flutter 社群推薦。

### D3: HTTP 客戶端 — Dio

**替代方案**:
- http → 功能較基本，需手動處理 interceptor
- Chopper → 需 code generation

**理由**: 支援 interceptor（Firebase token 自動附加）、retry、timeout，API 成熟。

### D4: 路由 — GoRouter

**理由**: Flutter 官方推薦的聲明式路由，支援 deep link、auth redirect guard。

### D5: 圖表 — fl_chart

**理由**: Flutter 最成熟的圖表庫，支援折線圖，自訂性高。

### D6: Wear OS Rust 綁定 — flutter_rust_bridge

**理由**: 自動產生 Dart bindings，支援 async、struct mapping。與 Apple Watch UniFFI 對稱。

### D7: Wear OS 本地持久化 — Hive

**替代方案**:
- sqflite → 較重
- shared_preferences → 不適合結構化資料

**理由**: 輕量、無 native dependency、適合手錶簡單資料模型。

### D8: Phone App 專案結構

```
flutter/phone_app/
├── lib/
│   ├── models/         # 資料模型
│   ├── repositories/   # API 封裝
│   ├── providers/      # Riverpod providers
│   ├── screens/        # 頁面（dashboard, history, detail, analysis, social, settings）
│   ├── widgets/        # 可重用元件
│   ├── services/       # Auth, API client
│   └── main.dart
├── android/
├── ios/
└── pubspec.yaml
```

### D9: Wear OS App 專案結構

```
flutter/wearos_app/
├── lib/
│   ├── models/
│   ├── services/       # TennisMatch (dart:ffi), SyncService, SpeechService
│   ├── screens/        # ScoreScreen, HistoryScreen, AuthScreen
│   └── main.dart
├── android/
├── rust/               # Rust engine source + flutter_rust_bridge config
└── pubspec.yaml
```

## Risks / Trade-offs

**Flutter Wear OS 支援** → Flutter 官方對 Wear OS 支援有限，需使用 `wear` package 處理圓形螢幕。

**Rust cross-compile** → 需 compile for `aarch64-linux-android`，使用 cargo-ndk。

**Firebase Auth on watchOS** → watchOS Firebase SDK 支援有限，可能需直接用 REST API。

**FCM on Wear OS** → Wear OS 支援 FCM，但需考慮低功耗模式下的通知傳遞。
