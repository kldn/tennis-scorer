## Context

目前專案是一個空的 Rust binary crate（只有 `Hello, world!`）。需要將它改造為 library crate，實作完整的網球計分引擎。此引擎未來會編譯為 static library，透過 C FFI 供 watchOS Swift app 呼叫，因此 API 設計必須簡潔且 FFI-friendly。

網球計分有三個層級結構：Match → Set → Game。每個層級有獨立的計分規則，且上層依賴下層的結果。

## Goals / Non-Goals

**Goals:**

- 以 Rust library crate 實作正確的網球計分 state machine
- 支援 Ad / No-Ad 兩種 game scoring 模式
- 支援可設定 tiebreak 目標分數（7 分、10 分、或自訂）
- 支援 Best-of-3 與 Best-of-5 賽制
- 提供 undo 功能（基於 point history stack）
- API 設計為 FFI-friendly，所有 public types 可透過 C 表達
- 高 test coverage，確保計分規則正確性

**Non-Goals:**

- C FFI bridge 的實作（屬於後續 `ffi-bridge` change）
- watchOS UI 或語音辨識（屬於 `watchos-app` change）
- 網路對戰、多人同步、雲端儲存
- 球員資料、統計分析、歷史戰績
- 計時功能（shot clock、match duration）

## Decisions

### 1. State machine 架構：巢狀 enum + struct

使用 Rust enum 表達每個層級的狀態，struct 持有狀態資料。

```
MatchState
├── Playing { sets: Vec<SetState>, config }
└── Completed { winner, final_score }

SetState
├── Playing { games: Vec<GameState>, tiebreak: Option<TiebreakState> }
└── Completed { winner, game_score }

GameState
├── Points(ServerPoints, ReceiverPoints)  // Love, Fifteen, Thirty, Forty
├── Deuce
├── Advantage(Player)
└── Completed(Player)
```

**為什麼不用 trait object / dynamic dispatch**：計分邏輯是固定的有限狀態，enum 比 trait object 更適合，且對 FFI 更友善（不涉及 vtable）。

### 2. 計分 API 風格：immutable state + 回傳新 state

```rust
pub fn score_point(state: &MatchState, scorer: Player) -> MatchState
```

每次得分回傳一個新的 `MatchState`，不修改原狀態。

**為什麼不用 mutable method（`&mut self`）**：
- Immutable 設計天然支援 undo（保留舊 state 即可）
- 更容易測試（每個 state transition 都是純函數）
- 對 FFI 更安全（不需處理 mutable borrow 的生命週期）

### 3. Undo 機制：history stack 存放完整 state snapshot

```rust
pub struct MatchWithHistory {
    current: MatchState,
    history: Vec<MatchState>,
}
```

每次 `score_point` 後，把前一個 state push 到 history。Undo 就是 pop。

**為什麼不用 command pattern（存操作記錄再反向推導）**：
- 網球計分 state 很小（幾百 bytes），snapshot 成本低
- 反向推導複雜且容易出錯（例如從 Deuce 回退到 40-40 需要知道之前是哪種狀態）
- Snapshot 方式正確性有保證

### 4. 設定系統：MatchConfig struct

```rust
pub struct MatchConfig {
    pub sets_to_win: u8,           // 2（Best-of-3）或 3（Best-of-5）
    pub tiebreak_points: u8,       // 7（標準）或 10（super）或自訂
    pub final_set_tiebreak: bool,  // 決勝盤是否用 tiebreak
    pub no_ad_scoring: bool,       // No-Ad mode 開關
}
```

提供 `MatchConfig::default()` 回傳最常見設定（Best-of-3、7 分 tiebreak、Ad scoring）。

### 5. Module 結構

```
src/
├── lib.rs              // public API re-exports
├── types.rs            // Player, Point, Score 等基礎型別
├── config.rs           // MatchConfig
├── game.rs             // Game-level state machine
├── tiebreak.rs         // Tiebreak-level state machine
├── set.rs              // Set-level state machine
├── match_state.rs      // Match-level state machine
└── history.rs          // MatchWithHistory, undo/redo
```

每個 module 對應一個計分層級，職責清楚分離。`tiebreak.rs` 獨立出來是因為 tiebreak 的計分邏輯（數字分數、serve 輪換）與一般 game 不同。

### 6. Serde 的使用範圍

只對 `MatchConfig` 加上 `Serialize/Deserialize`，用於從設定檔或前端傳入。State types 暫時不需要序列化（FFI 時會另外定義 C-compatible 的表示）。

## Risks / Trade-offs

**State snapshot 記憶體使用** → 一場 match 最多約 200-300 個 points，每個 snapshot 幾百 bytes，總共不到 100KB。在 Apple Watch 上完全可接受。

**Immutable state 的效能** → 每次 score_point 會 clone 整個 MatchState（包含 Vec）。Clone 成本極低（小型 Vec），且 Apple Watch 上計分頻率不高（每秒最多 1 次），不會是效能瓶頸。

**Tiebreak 規則變體** → 目前只支援「可設定目標分數」的 tiebreak。如果未來需要更多變體（例如某些聯賽的特殊規則），可能需要擴展 `MatchConfig`。目前先不過度設計。

**Enum exhaustiveness** → 使用 Rust enum 的好處是 compiler 強制 match exhaustive，新增狀態時會在所有未處理的地方報錯，降低漏掉邊界情況的風險。
