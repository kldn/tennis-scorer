## 1. Flutter Phone App — 專案初始化

- [ ] 1.1 建立 Flutter 專案 `flutter/phone_app/`（Android + iOS）
- [ ] 1.2 加入依賴：`flutter_riverpod`, `dio`, `firebase_auth`, `firebase_messaging`, `google_sign_in`, `sign_in_with_apple`, `fl_chart`, `go_router`, `flutter_secure_storage`
- [ ] 1.3 建立目錄結構（models, repositories, providers, screens, widgets, services）
- [ ] 1.4 設定 Firebase 專案（google-services.json, GoogleService-Info.plist）

## 2. Phone App — 認證

- [ ] 2.1 實作 `AuthService`：Firebase Auth 初始化、Google Sign-In、Apple Sign-In
- [ ] 2.2 實作 `ApiClient`（Dio）：Firebase token interceptor 自動附加、error handling
- [ ] 2.3 實作 `AuthProvider`（Riverpod）：登入狀態、自動登入、token 刷新
- [ ] 2.4 建立 `LoginScreen`：Google + Apple Sign-In 按鈕、loading 狀態、error 提示

## 3. Phone App — Dashboard

- [ ] 3.1 實作 `StatsRepository`：呼叫 `GET /api/stats/summary`
- [ ] 3.2 實作 `DashboardProvider`：載入勝率摘要、近期比賽
- [ ] 3.3 建立 `DashboardScreen`：勝率、連勝、近期比賽快速入口

## 4. Phone App — 比賽列表 (History)

- [ ] 4.1 定義 `Match` model
- [ ] 4.2 實作 `MatchRepository`：呼叫 `GET /api/matches` 含分頁
- [ ] 4.3 實作 `MatchListProvider`：分頁載入、pull-to-refresh
- [ ] 4.4 建立 `HistoryScreen`：日期、比數、勝負、分頁、空狀態

## 5. Phone App — 比賽詳情與分析

- [ ] 5.1 定義 `MatchAnalysis`, `MomentumData`, `PaceData` models
- [ ] 5.2 實作 `StatsRepository`：analysis、momentum、pace endpoints
- [ ] 5.3 實作 `MatchDetailProvider` + `AnalysisProvider`
- [ ] 5.4 建立 `MatchDetailScreen`：header + 統計區塊
- [ ] 5.5 建立 `AnalysisScreen`：fl_chart 折線圖、模式切換、pace 資訊

## 6. Phone App — 社交

- [ ] 6.1 實作 `SocialRepository`：好友請求、好友列表、head-to-head endpoints
- [ ] 6.2 實作 `SocialProvider`：好友請求管理、好友列表
- [ ] 6.3 建立 `SocialScreen`：好友列表、待處理請求、搜尋好友
- [ ] 6.4 建立 `HeadToHeadView`：與好友的對戰記錄統計

## 7. Phone App — 設定

- [ ] 7.1 建立 `SettingsScreen`：帳號資訊、通知偏好、登出
- [ ] 7.2 實作通知偏好 API 呼叫（`PUT /api/notifications/settings`）

## 8. Phone App — FCM 推播

- [ ] 8.1 初始化 `firebase_messaging`
- [ ] 8.2 取得 FCM token 並呼叫 `POST /api/notifications/register`
- [ ] 8.3 處理前景/背景通知顯示
- [ ] 8.4 實作通知點擊導航（好友請求 → 社交頁、比賽結果 → 比賽詳情）

## 9. Phone App — 路由與整合

- [ ] 9.1 設定 GoRouter：auth guard、畫面導航
- [ ] 9.2 整合測試：完整流程驗證

## 10. Flutter Wear OS Watch App — 專案初始化

- [ ] 10.1 建立 Flutter 專案 `flutter/wearos_app/`（Android only, Wear OS target）
- [ ] 10.2 加入依賴：`flutter_rust_bridge`, `hive`, `firebase_auth`, `wear`
- [ ] 10.3 設定 flutter_rust_bridge + cargo-ndk 交叉編譯

## 11. Wear OS — Rust 引擎整合

- [ ] 11.1 設定 Rust → Dart FFI bindings（flutter_rust_bridge codegen）
- [ ] 11.2 實作 `TennisMatch` wrapper（dart:ffi → Rust MatchWithHistory）
- [ ] 11.3 驗證計分邏輯正確性

## 12. Wear OS — 計分 UI

- [ ] 12.1 建立 `ScoreScreen`：圓形螢幕適配、得分按鈕、比分顯示
- [ ] 12.2 建立 `HistoryScreen`：本地比賽記錄
- [ ] 12.3 建立 `AuthScreen`：Firebase Auth 登入

## 13. Wear OS — 離線同步

- [ ] 13.1 實作 Hive 本地持久化（MatchRecord, EventRecord）
- [ ] 13.2 實作 `SyncService`：離線比賽上傳、client_id 冪等、失敗重試
- [ ] 13.3 同步觸發：比賽完成時 + app 啟動時

## 14. Watch App — Firebase Auth 遷移

- [ ] 14.1 改寫 `AuthView.swift`：移除 email/password，改為 Firebase Auth (Apple Sign-In)
- [ ] 14.2 修改 `APIClient.swift`：傳送 Firebase ID Token 而非 JWT

## 15. CI — Flutter Pipeline

- [ ] 15.1 建立 `.github/workflows/flutter.yml`
- [ ] 15.2 Phone App：flutter analyze + test + build apk + build ios
- [ ] 15.3 Wear OS App：flutter analyze + test + build apk
- [ ] 15.4 設定觸發條件（flutter/ 目錄變更時執行）
