## 1. 專案設定

- [x] 1.1 在 Cargo.toml 加入 `libc` 依賴
- [x] 1.2 建立 `src/ffi.rs` module
- [x] 1.3 在 `lib.rs` 加入 `mod ffi;` 並 pub export

## 2. C-Compatible 型別定義（ffi.rs）

- [x] 2.1 定義 `PLAYER_1` 和 `PLAYER_2` 常數
- [x] 2.2 定義 `GameStateCode` 常數（PLAYING=0, DEUCE=1, ADVANTAGE_P1=2, ADVANTAGE_P2=3, COMPLETED=4）
- [x] 2.3 定義 `#[repr(C)] MatchScore` struct
- [x] 2.4 定義 opaque `TennisMatch` type（包裝 `MatchWithHistory`）

## 3. Lifecycle 函數

- [x] 3.1 實作 `tennis_match_new_default()` → 建立預設 Best-of-3 match
- [x] 3.2 實作 `tennis_match_new_best_of_5()` → 建立 Best-of-5 match
- [x] 3.3 實作 `tennis_match_new_no_ad()` → 建立 No-Ad scoring match
- [x] 3.4 實作 `tennis_match_new_custom()` → 建立自訂設定 match
- [x] 3.5 實作 `tennis_match_free()` → 釋放 match 記憶體（處理 null 安全）

## 4. 計分函數

- [x] 4.1 實作 `tennis_match_score_point()` → 計分，回傳 bool
- [x] 4.2 實作 `tennis_match_undo()` → 回退，回傳 bool
- [x] 4.3 處理 null pointer 和無效 player 值的邊界情況

## 5. 查詢函數

- [x] 5.1 實作 `tennis_match_get_score()` → 回傳 MatchScore struct
- [x] 5.2 實作 helper 函數將內部 state 轉換為 MatchScore
- [x] 5.3 實作 `tennis_match_can_undo()` → 回傳 bool
- [x] 5.4 實作 `tennis_match_is_complete()` → 回傳 bool
- [x] 5.5 實作 `tennis_match_get_winner()` → 回傳 uint8_t (0/1/2)

## 6. Header 產生

- [x] 6.1 建立 `cbindgen.toml` 設定檔
- [x] 6.2 手動或透過 build.rs 產生 `include/tennis_scorer.h`
- [x] 6.3 驗證 header 語法正確（可被 clang 解析）

## 7. 測試

- [x] 7.1 撰寫 FFI 函數的 Rust 單元測試
- [x] 7.2 確保 `cargo build --release` 產生正確的 static library
- [x] 7.3 驗證 library 包含所有 exported symbols（`nm` 檢查）
