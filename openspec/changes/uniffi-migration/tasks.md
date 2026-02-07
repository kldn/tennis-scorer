## 1. 擴充 UniFFI crate 到完整 API

- [x] 1.1 加入 `MatchConfig` UniFFI Record（sets_to_win, tiebreak_points, final_set_tiebreak, no_ad_scoring, match_type, serve_order）
- [x] 1.2 加入 `TennisMatch` 自訂 constructor：`new_with_config(config: MatchConfig)`
- [x] 1.3 加入 `undo()` 方法，回傳 `MatchScore`
- [x] 1.4 加入 `can_undo()` 方法，回傳 `bool`
- [x] 1.5 加入 `MatchScore` 的 `deuce_count` 和 `current_server` 欄位
- [x] 1.6 加入 `PointEvent` Record 和 `get_point_events()` 方法
- [x] 1.7 加入 `new_match()` 方法重置比賽
- [x] 1.8 為所有新增功能撰寫 Rust 單元測試

## 2. 產生並驗證 Swift bindings

- [x] 2.1 重新產生 Swift bindings（`uniffi-bindgen generate`）
- [x] 2.2 確認生成的 Swift API 包含所有 constructors、methods、enums、records
- [x] 2.3 驗證 watchOS sim + device cross-compile 通過

## 3. 改寫 Swift 端 TennisMatch.swift

- [x] 3.1 移除 `import TennisScorerFFI` 和 C FFI 常數引用
- [x] 3.2 改用 UniFFI 生成的 `TennisMatch` class（移除 `OpaquePointer` / `handle`）
- [x] 3.3 改寫 `Score` struct 從 UniFFI `MatchScore` / `GameScore` 轉換
- [x] 3.4 改寫 `getPointEvents()` 使用 UniFFI `PointEvent` record
- [x] 3.5 移除 `deinit` 中的手動 `tennis_match_free` 呼叫
- [x] 3.6 確認 ContentView / MatchHistoryView 不需要改動（只依賴 `Score` 和 `TennisMatch` API）

## 4. 清理舊 C FFI

- [x] 4.1 從 `crates/tennis-scorer/src/lib.rs` 移除 `pub mod ffi`
- [x] 4.2 刪除 `crates/tennis-scorer/src/ffi.rs`
- [x] 4.3 從 `crates/tennis-scorer/Cargo.toml` 移除 `libc` 依賴和 `staticlib` crate-type
- [x] 4.4 移除 `cbindgen.toml` 和 `build.rs` 中 cbindgen 邏輯
- [x] 4.5 更新 Xcode project 連結 `libtennis_scorer_uniffi.a` 取代 `libtennis_scorer.a`
- [x] 4.6 移除 Bridging Header 相關設定
- [x] 4.7 執行 `cargo test` 確認所有 workspace 測試通過
