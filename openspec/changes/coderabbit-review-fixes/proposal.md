## Why

CodeRabbit CLI 對整個專案進行了全面 review，發現了安全漏洞、bug 和程式碼品質問題。這些問題涵蓋 API 安全性（未授權端點、CORS 設定、資料洩漏）、Rust 穩定性（nightly API 使用）、Swift UI bug（Alert 無法關閉）以及多項防禦性程式設計缺失。需要統一修復以提升專案整體品質。

## What Changes

### 安全性修復
- Debug endpoint `/debug/matches` 加上 `#[cfg(debug_assertions)]` 保護，避免 production 暴露未授權端點
- CORS 從 wildcard `Any` 改為環境變數設定的 origin allowlist
- Idempotency check 加上 `AND user_id` 條件，防止跨用戶資料洩漏
- Email 驗證改用 regex 或 validator crate 強化

### Bug 修復
- SwiftUI Alert 的 `.constant()` binding 改為可變的 `@State` binding，使 dismiss 正常運作
- `is_multiple_of(2)` 改為穩定版 `% 2 == 0`（避免需要 nightly Rust）
- 移除 `replay.rs` 中重複的 doc comment
- APIClient retry 的 HTTP status code range 與原始請求保持一致

### 程式碼品質改善
- `epoch_secs_to_system_time` 加入負值防禦處理
- SpeechRecognizer 的 Task 改用 `[weak self]` 避免 retain cycle
- SwiftData save 從 `try?` 改為 do-catch 並記錄錯誤
- 手動 timestamp 轉換改用 chrono 的 `.into()`
- APIClient 加入 request timeout

## Capabilities

### New Capabilities

_無新增 capability_

### Modified Capabilities

- `api-auth`: Email 驗證邏輯強化、debug endpoint 安全性限制
- `api-matches`: Idempotency check 加入 user scope、debug route 條件編譯
- `api-infra`: CORS 設定從 wildcard 改為環境變數控制
- `voice-input`: SpeechRecognizer retain cycle 修復、Alert dismiss 修復
- `match-replay`: 移除重複 doc comment、`is_multiple_of` 改為穩定語法
- `match-sync`: APIClient status code 一致性、加入 request timeout
- `uniffi-bindings`: `epoch_secs_to_system_time` 負值防禦
- `api-stats`: timestamp 轉換改用慣用寫法
- `local-persistence`: SwiftData save 錯誤處理

## Impact

- **Rust crates**: `tennis-scorer`, `tennis-scorer-api`, `tennis-scorer-uniffi`
- **Swift**: `ContentView.swift`, `SpeechRecognizer.swift`, `APIClient.swift`
- **Xcode project**: `project.pbxproj`（library search paths）
- **Dependencies**: 可能新增 `regex` 或 `validator` crate
- **API 行為**: CORS 設定變更需要部署時設定環境變數
