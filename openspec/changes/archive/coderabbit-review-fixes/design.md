## Context

CodeRabbit CLI review 發現了 14 項問題，橫跨 Rust API（安全性）、Swift watchOS app（UI bug、記憶體管理）和 build configuration。這些是獨立的修復，不涉及新功能或架構變更，每項修復範圍小且風險低。

## Goals / Non-Goals

**Goals:**
- 修復所有 CodeRabbit 發現的安全漏洞（debug endpoint、CORS、idempotency、email validation）
- 修復所有 bug（Alert dismiss、nightly API、重複 comment、status code 不一致）
- 改善防禦性程式設計（負值處理、retain cycle、錯誤處理、timeout）

**Non-Goals:**
- 不重構現有架構
- 不新增功能
- 不修改 watchOS library path（Xcode project 的 build configuration 變更風險較高，且目前只需要 simulator build for development，留待後續處理）
- 不處理 spec 與實作的語言不一致（voice command spec vs implementation 語言差異，這是 spec 文件問題而非 code 問題）

## Decisions

### D1: Debug endpoint 保護方式 — `#[cfg(debug_assertions)]`
使用 `#[cfg(debug_assertions)]` 而非 feature flag，因為：
- 不需要額外 Cargo.toml 設定
- release build 自動排除，零風險
- 同時保護 handler 和 route 註冊

### D2: CORS 設定 — 環境變數 `ALLOWED_ORIGINS`
在 `AppConfig` 加入 `allowed_origins: Vec<String>`，從環境變數 `ALLOWED_ORIGINS`（逗號分隔）讀取。未設定時 development 環境 fallback 到 `Any`。

### D3: Email 驗證 — 使用 regex
使用簡單 regex `^[^\s@]+@[^\s@]+\.[^\s@]+$` 而非引入 `validator` crate，避免新增依賴。

### D4: Alert dismiss — `@State` + `onChange`
新增 `@State private var showPermissionAlert = false`，用 `onChange(of: speechRecognizer.permissionDenied)` 驅動，取代 `.constant()` binding。

### D5: SpeechRecognizer Task — stored property + weak self
新增 `private var idleTask: Task<Void, Never>?`，在 `scheduleReturnToIdle()` 中 cancel 舊 task 再建立新的，用 `[weak self]` capture。在 `stopListening()` 中也 cancel。

## Risks / Trade-offs

- [CORS 變更需要部署設定] → 文件說明需設定 `ALLOWED_ORIGINS` 環境變數，未設定時 fallback 到 permissive mode
- [Debug endpoint 移除可能影響開發流程] → `#[cfg(debug_assertions)]` 確保 `cargo run` 仍可使用，只有 release build 排除
- [Regex email validation 不完整] → 對於此應用場景已足夠，未來可升級到 validator crate
