## 1. API 安全性修復

- [ ] 1.1 在 `crates/tennis-scorer-api/src/matches/handlers.rs` 的 `create_match_debug` 函數加上 `#[cfg(debug_assertions)]`
- [ ] 1.2 在 `crates/tennis-scorer-api/src/matches/mod.rs` 將 debug route 註冊包在 `#[cfg(debug_assertions)]` 條件編譯中
- [ ] 1.3 在 `AppConfig` 新增 `allowed_origins` 欄位，從環境變數 `ALLOWED_ORIGINS` 讀取（逗號分隔）
- [ ] 1.4 修改 `crates/tennis-scorer-api/src/lib.rs` 的 CorsLayer 建構，根據 `allowed_origins` 設定 origin allowlist，未設定時 fallback 到 Any
- [ ] 1.5 在 `crates/tennis-scorer-api/src/matches/handlers.rs` 的 idempotency check SQL 加上 `AND user_id = $2` 並 bind user_id
- [ ] 1.6 在 `crates/tennis-scorer-api/src/auth/handlers.rs` 將 email 驗證改為 regex `^[^\s@]+@[^\s@]+\.[^\s@]+$`，加入 `regex` crate 依賴

## 2. Rust Bug 修復

- [ ] 2.1 在 `crates/tennis-scorer/src/analysis/replay.rs` 將 `is_multiple_of(2)` 改為 `% 2 == 0`（兩處）
- [ ] 2.2 在 `crates/tennis-scorer/src/analysis/replay.rs` 移除第 501 行附近重複的 doc comment

## 3. Rust 程式碼品質改善

- [ ] 3.1 在 `crates/tennis-scorer-uniffi/src/lib.rs` 的 `epoch_secs_to_system_time` 加入負值和 NaN 防禦處理
- [ ] 3.2 在 `crates/tennis-scorer-api/src/stats/handlers.rs` 將手動 timestamp 轉換改為 `let system_time: SystemTime = ts.into()`

## 4. Swift Bug 修復

- [ ] 4.1 在 `ContentView.swift` 將 Alert 的 `.constant(speechRecognizer.permissionDenied)` 改為 `@State` binding + `onChange` 驅動
- [ ] 4.2 在 `ContentView.swift` 將 `try? modelContext.save()` 改為 do-catch 並 log 錯誤

## 5. Swift 程式碼品質改善

- [ ] 5.1 在 `SpeechRecognizer.swift` 的 `scheduleReturnToIdle` 改用 stored `idleTask` property + `[weak self]` capture，並在 `stopListening()` 中 cancel
- [ ] 5.2 在 `APIClient.swift` 統一 retry 的 HTTP status code range 與原始請求一致（200...201 + 204）
- [ ] 5.3 在 `APIClient.swift` 加入 `request.timeoutInterval = 30`
