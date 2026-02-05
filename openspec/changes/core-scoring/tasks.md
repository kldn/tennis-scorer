## 1. 專案結構設置

- [x] 1.1 將 Cargo.toml 改為 library crate，設定 `crate-type = ["staticlib", "rlib"]`，加入 `serde` 與 `serde_derive` 依賴
- [x] 1.2 建立 module 結構：`lib.rs`、`types.rs`、`config.rs`、`game.rs`、`tiebreak.rs`、`set.rs`、`match_state.rs`、`history.rs`

## 2. 基礎型別（types.rs）

- [x] 2.1 定義 `Player` enum（Player1, Player2）
- [x] 2.2 定義 `Point` enum（Love, Fifteen, Thirty, Forty）及其遞增方法

## 3. 設定系統（config.rs）

- [x] 3.1 實作 `MatchConfig` struct，包含 `sets_to_win`、`tiebreak_points`、`final_set_tiebreak`、`no_ad_scoring` 欄位
- [x] 3.2 實作 `MatchConfig::default()`，預設為 Best-of-3、tiebreak 7 分、決勝盤啟用 tiebreak、Ad scoring
- [x] 3.3 為 `MatchConfig` 加上 `Serialize` / `Deserialize` derive

## 4. Game 計分邏輯（game.rs）

- [x] 4.1 定義 `GameState` enum（Points、Deuce、Advantage、Completed）
- [x] 4.2 實作 Ad scoring 的 `score_point`：Love→15→30→40→Game、Deuce↔Advantage 流程
- [x] 4.3 實作 No-Ad scoring：Deuce 時一球定勝負
- [x] 4.4 處理已完成 game 的計分拒絕（回傳原 state）
- [x] 4.5 撰寫 game scoring 單元測試，涵蓋所有 spec scenarios

## 5. Tiebreak 計分邏輯（tiebreak.rs）

- [x] 5.1 定義 `TiebreakState` struct（player1_points、player2_points、target_points）
- [x] 5.2 實作 tiebreak `score_point`：數字計分、達到目標分數且領先 2 分判勝
- [x] 5.3 處理 tiebreak 延長情境（雙方都接近目標分數時繼續到領先 2 分）
- [x] 5.4 撰寫 tiebreak 單元測試，涵蓋標準 7 分、super 10 分、延長情境

## 6. Set 計分邏輯（set.rs）

- [x] 6.1 定義 `SetState` enum（Playing、Completed），Playing 包含 games 記錄與可選的 tiebreak
- [x] 6.2 實作 set `score_point`：將得分委派給當前 game 或 tiebreak
- [x] 6.3 實作 set 勝利判定：6 games 且領先 2 games
- [x] 6.4 實作 6-6 時自動觸發 tiebreak
- [x] 6.5 處理已完成 set 的計分拒絕
- [x] 6.6 撰寫 set scoring 單元測試，涵蓋 6-4、7-5、tiebreak 7-6 等情境

## 7. Match 計分邏輯（match_state.rs）

- [x] 7.1 定義 `MatchState` enum（Playing、Completed），Playing 包含 sets 記錄與 config
- [x] 7.2 實作 match `score_point`：將得分委派給當前 set
- [x] 7.3 實作 Best-of-3 / Best-of-5 勝利判定
- [x] 7.4 實作決勝盤 tiebreak 開關邏輯（`final_set_tiebreak` 設定）
- [x] 7.5 處理已完成 match 的計分拒絕
- [x] 7.6 撰寫 match scoring 單元測試，涵蓋 2-0、2-1、3-0、3-2 等情境

## 8. 歷史與 Undo（history.rs）

- [x] 8.1 定義 `MatchWithHistory` struct，包含 current state 與 history stack
- [x] 8.2 實作 `score_point` 方法：計分前將 current state push 到 history
- [x] 8.3 實作 `undo` 方法：從 history stack pop 並恢復 state
- [x] 8.4 處理邊界情況：match 已結束時不記錄、history 為空時 undo 無效
- [x] 8.5 撰寫 history/undo 單元測試，涵蓋連續 undo、undo 後重新計分等情境

## 9. Public API 匯出（lib.rs）

- [x] 9.1 在 `lib.rs` 中 re-export 所有 public types 與函數
- [x] 9.2 撰寫整合測試：模擬一場完整的 match 從頭到尾（含 tiebreak 與 undo）
