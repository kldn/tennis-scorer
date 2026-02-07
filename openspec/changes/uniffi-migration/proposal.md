## Why

現有 C FFI 層 (`ffi.rs`, 734 行) 全手動管理記憶體、型別轉換和 null 檢查，維護成本高且 header 已經與實作不同步。UniFFI spike (`spike/uniffi` branch) 已驗證 proc-macro 方案可在 watchOS 上 build 並產生 idiomatic Swift bindings，是時候正式遷移。

## What Changes

- **新增** `crates/tennis-scorer-uniffi/` crate，用 UniFFI proc-macro 包裝完整 scoring API
- **BREAKING** 移除 `crates/tennis-scorer/src/ffi.rs` 手寫 C FFI 層
- **BREAKING** 移除 cbindgen 產生的 C header，改用 UniFFI 產生的 Swift bindings
- **改寫** watchOS app 的 `MatchManager.swift`，從 `UnsafeMutablePointer` 操作切換為 UniFFI 生成的 `TennisMatch` class
- **移除** `Bridging-Header.h`，改用 UniFFI modulemap

## Capabilities

### New Capabilities
- `uniffi-bindings`: UniFFI proc-macro crate 提供完整 scoring API 的 Swift bindings（TennisMatch class, Player/GameScore enums, MatchScore record）

### Modified Capabilities
- `match-history`: Swift 端呼叫方式從 C FFI pointer 改為 UniFFI object，行為不變但 API surface 完全不同
- `ui-labels`: MatchManager 內部實作改變（不再使用 unsafe pointer），但 UI 對外介面不變

## Impact

- **Rust crates**: 新增 `tennis-scorer-uniffi`，移除 `tennis-scorer` 的 `ffi` module、`libc` 依賴、`staticlib` crate-type
- **Swift/Xcode**: 移除 Bridging Header，加入 UniFFI 生成的 `.swift` + `.modulemap`，改寫 `MatchManager`
- **Build**: Xcode build phase 需連結 `libtennis_scorer_uniffi.a` 而非 `libtennis_scorer.a`
- **Binary size**: 靜態庫從 2.8MB 增至 ~19MB（linker strip 後實際 app 差異待測）
- **依賴**: 新增 `uniffi` crate (0.29)
