## Why

Apple Watch 螢幕太小，不適合查看詳細統計數據和圖表。需要一個手機 app 來瀏覽比賽紀錄、查看統計分析、動量圖表、以及社交功能。此外，Wear OS 使用者也需要手錶端計分功能。

目前 Watch app 使用 email/password 登入，在小螢幕上輸入不便。根據系統架構設計，全平台改為 Firebase Auth（Google + Apple Sign-In），取代自訂 JWT，統一身份驗證並整合 FCM 推播通知。

## What Changes

### 1. Flutter Phone App（新專案）
- Firebase Auth 登入（Google Sign-In + Apple Sign-In）
- 4 層架構：UI → State Management (Riverpod) → Repository → Network
- 6 個主要畫面：Dashboard、Match Detail、Social、History、Analysis、Settings
- FCM 推播通知整合（好友請求、比賽結果、對手認領）
- 純 API 客戶端，不包含 Rust 引擎，唯讀不做計分

### 2. Flutter Wear OS Watch App（新專案）
- Flutter + dart:ffi（透過 `flutter_rust_bridge` 綁定 Rust 引擎）
- 3 層架構：UI → Service → Engine + Storage
- 離線優先計分，上線後同步到雲端
- 本地持久化：SQLite 或 Hive
- 語音輸入（speech-to-text）
- Firebase Auth 登入

### 3. Watch App 改用 Firebase Auth
- 登入畫面改為 Firebase Auth（支援 Apple Sign-In on watchOS）
- 移除 email/password 登入 UI
- 同一 Firebase UID 在所有平台對應同一使用者

### 4. Flutter CI Pipeline（新增）
- `flutter.yml` workflow：analyze、test、build apk、build ios

## Capabilities

### New Capabilities
- `flutter-phone-dashboard`: Dashboard 畫面，近期比賽、勝率摘要、快速入口
- `flutter-match-list`: 比賽列表（History Screen），分頁瀏覽歷史比賽紀錄
- `flutter-match-detail`: 比賽詳情頁，完整統計分析數據
- `flutter-momentum-chart`: 互動式 Momentum 圖表（Analysis Screen），支援多模式切換
- `flutter-social`: 社交畫面，好友列表、好友請求管理、對戰記錄
- `flutter-settings`: 設定畫面，帳號資訊、通知偏好、登出
- `flutter-fcm`: FCM 推播通知整合
- `wearos-scoring`: Wear OS 計分功能，Rust 引擎 + 離線優先
- `wearos-sync`: Wear OS 雲端同步
- `flutter-ci`: Flutter CI pipeline

### Modified Capabilities
- `watch-auth`: 登入方式從 email/password 改為 Firebase Auth (Apple Sign-In)

## Impact

- **New projects**: `flutter/phone_app/`、`flutter/wearos_app/`
- **Phone app deps**: Flutter SDK, `flutter_riverpod`, `dio`, `firebase_auth`, `firebase_messaging`, `google_sign_in`, `sign_in_with_apple`, `fl_chart`, `go_router`
- **Wear OS deps**: Flutter SDK, `flutter_rust_bridge`, `sqflite`/`hive`, `firebase_auth`, `speech_to_text`
- **Watch app**: `AuthView.swift` 改為 Firebase Auth
- **CI**: 新增 `flutter.yml` GitHub Actions workflow
- **Firebase**: 需建立 Firebase 專案，啟用 Auth + FCM
- **Apple Developer**: App ID 需啟用 Sign in with Apple
- **Google Play**: Wear OS + Phone app 上架
