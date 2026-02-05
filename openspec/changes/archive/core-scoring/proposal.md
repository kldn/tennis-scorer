## Why

本專案需要一個可靠的、跨平台的網球計分引擎，以 Rust library 實作。核心邏輯必須與平台無關，以便編譯為 static library 後透過 C FFI 供 Swift（watchOS）、Flutter 或其他前端呼叫。在建構任何 UI 之前，先把計分規則做對並充分測試是最重要的第一步。

## What Changes

- 新增完整的網球 match state machine，涵蓋 Game、Set、Match 三個層級
- 支援可設定的賽制：Best-of-3 與 Best-of-5 sets
- 支援可設定目標分數的 tiebreak（標準 7 分、super tiebreak 10 分、或自訂分數）
- 同時支援傳統 Ad（Advantage）scoring 與 No-Ad scoring（Deuce 時一球定勝負）
- 提供 undo 功能，可回退上一個得分
- 提供乾淨的 public API，為未來 C FFI bridging 做準備

## Capabilities

### New Capabilities

- `game-scoring`: 單個 game 內的 point-level 計分，處理 Love/15/30/40、Deuce、Advantage、以及 No-Ad sudden death
- `set-scoring`: 單個 set 內的 game-level 追蹤，包含 tiebreak 觸發條件與可設定的 tiebreak 目標分數
- `match-scoring`: match 層級的 set-level 追蹤，支援 Best-of-3 與 Best-of-5 賽制，判定 match winner
- `match-config`: 賽制設定的型別定義，包含 match format、tiebreak 規則、scoring mode（Ad vs No-Ad）
- `score-history`: 得分歷史堆疊，支援 undo/redo 計分操作

### Modified Capabilities

（無 — 這是全新專案，沒有既有的 specs）

## Impact

- **Code**: 在 `src/` 下新增 Rust library crate，每個 capability 對應一個 module
- **Cargo.toml**: 新增 `[lib]` section 並設定 `crate-type = ["staticlib", "rlib"]`；加入 `serde` 用於 config 序列化
- **Dependencies**: `serde` + `serde_derive` 用於設定；預期不需其他外部依賴
- **Future**: 此 library 將成為 C FFI bridge 與 watchOS app 整合的基礎
