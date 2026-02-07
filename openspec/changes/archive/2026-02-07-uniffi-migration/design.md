## Context

現有 C FFI 層 (`ffi.rs`, 734 行) 用手動 `Box::into_raw` / `Box::from_raw` 管理記憶體，透過 `#[repr(C)]` struct 和 `extern "C"` 函式暴露 API。Swift 端 (`TennisMatch.swift`, 190 行) 使用 `OpaquePointer` + `import TennisScorerFFI` 呼叫這些 C 函式。

spike/uniffi branch 已驗證 UniFFI proc-macro 可在 watchOS (sim + device) 上 build，並產生 idiomatic Swift bindings。

## Goals / Non-Goals

**Goals:**
- 用 UniFFI proc-macro 取代手寫 C FFI，覆蓋完整 API（含 undo、doubles、point events）
- Swift 端改用 UniFFI 生成的 class，消除所有 unsafe pointer 操作
- 保持 core crate (`tennis-scorer`) 完全不變
- 維持現有 UI 行為不變

**Non-Goals:**
- 不改 scoring engine 邏輯
- 不整合 UniFFI bindings 到 Xcode project（Xcode 整合另案處理）
- 不處理 binary size 最佳化（後續可用 LTO / strip 處理）
- 不移除現有 C FFI（先並存，確認 UniFFI 版本完整後再移除）

## Decisions

### D1: UniFFI proc-macro 而非 UDL
- proc-macro 不需要額外 `.udl` 檔案和 `build.rs` boilerplate
- 直接在 Rust 程式碼加 attribute，維護單一 source of truth
- UniFFI 官方推薦新專案用 proc-macro

### D2: 獨立 crate 而非修改 core
- `crates/tennis-scorer-uniffi/` 只依賴 core crate 的 public API
- core crate 完全不受影響，C FFI 可先並存
- 回退成本為零（刪目錄即可）

### D3: 內部用 RwLock 包裝 MatchWithHistory
- UniFFI Object 需要 `Sync + Send`，而 `MatchWithHistory` 的 immutable pattern (clone-based) 不直接滿足
- 用 `RwLock` 包裝，write lock 只在 `score_point` / `undo` 時取得
- 不影響效能（watch app 單線程 UI）

### D4: MatchScore 用 Vec<u8> 記錄每盤比分
- 取代 C FFI 只回傳當前盤比分的限制
- Swift 端可看到完整多盤歷史（如 6-4, 3-6, 7-5）
- `Vec<u8>` 在 Swift 映射為 `Data`，可用 extension 轉換

### D5: GameScore enum 取代 numeric game_state
- C FFI 用 `u8` constants (`GAME_STATE_DEUCE = 1` 等)
- UniFFI 直接用 enum with associated values，Swift 端可 pattern match
- 更 type-safe，不需要手動 switch 數字

### D6: 先並存再移除 C FFI
- 先完成 UniFFI crate 全部功能
- 驗證 Swift 端可正常呼叫
- 最後才移除 `ffi.rs` 和相關 cbindgen 配置

## Risks / Trade-offs

- **Binary size 增大** (2.8MB → 19MB staticlib) → 實際 app 差異待測，後續可用 LTO + strip 最佳化
- **UniFFI 版本鎖定** → 0.29 是穩定版，升級路徑清晰
- **`Vec<u8>` → `Data` mapping** → 不影響功能，但 Swift 端需 `[UInt8](data)` 轉換；可改用 `Vec<u16>` 或加 helper
- **並存期間兩套 FFI** → 短期維護成本，但降低遷移風險
