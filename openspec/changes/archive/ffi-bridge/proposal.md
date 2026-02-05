## Why

core-scoring library 已完成，但目前只能在 Rust 內使用。要讓 watchOS app（Swift）能呼叫這個計分引擎，需要透過 C FFI 橋接。Rust 可以編譯為 static library，再透過 C-compatible 的函數介面讓 Swift 呼叫。

## What Changes

- 新增 `ffi.rs` module，定義 C-compatible 的 API
- 定義 opaque pointer types 處理 Rust 的複雜型別（MatchWithHistory）
- 提供 create/destroy 函數處理記憶體管理
- 提供計分、undo、查詢狀態等核心操作的 C 函數
- 產生 C header file 供 Swift 使用

## Capabilities

### New Capabilities

- `ffi-types`: C-compatible 的型別定義（opaque pointers、enums、structs）
- `ffi-lifecycle`: 建立與釋放 match 的函數（create/destroy pattern）
- `ffi-scoring`: 計分與 undo 的 C 函數
- `ffi-query`: 查詢當前狀態的 C 函數（分數、winner、是否可 undo）
- `ffi-header`: 產生 cbindgen header file

### Modified Capabilities

- `lib.rs`: 新增 ffi module 的 public export

## Impact

- **Code**: 新增 `src/ffi.rs`，約 200-300 行
- **Cargo.toml**: 加入 `cbindgen` dev-dependency（用於產生 header）
- **Build**: 新增 `cbindgen.toml` 設定、`build.rs` 或手動產生 header
- **Output**: 產生 `include/tennis_scorer.h` 供 Swift/Xcode 使用
