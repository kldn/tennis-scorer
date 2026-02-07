## Context

目前 `GameState` enum 有 `Deuce` 變體，但不追蹤進入 deuce 的次數。每當從 40-40 或 Advantage 回到 Deuce 時，都應該增加計數。

現有架構：
- `GameState::Deuce` - 無狀態的 deuce 表示
- `GameState::Advantage(Player)` - 優勢狀態
- 當 Advantage 被破後回到 Deuce

## Goals / Non-Goals

**Goals:**
- 追蹤單個 game 中進入 deuce 的總次數
- 透過 FFI 暴露給 Swift 層
- 在 UI 顯示 deuce 次數

**Non-Goals:**
- 不追蹤整場比賽的 deuce 總次數（可以在 Swift 層自行累加）
- 不儲存歷史 game 的 deuce 次數
- 不修改 no-ad scoring 邏輯（no-ad 模式下不會有 deuce）

## Decisions

### D1: 將 deuce_count 加入 GameState

**選擇**：修改 `GameState::Deuce` 從 unit variant 變為 `Deuce { count: u8 }`

**理由**：
- 保持 deuce 狀態的語義完整
- count 只在 Deuce 和 Advantage 狀態間流轉時有意義
- 使用 u8 足夠（實際上很少超過 10 次 deuce）

**替代方案**：
- 在 GameState 外部追蹤 → 會破壞 immutable 設計
- 新增獨立的 GameStats 結構 → 過度設計

### D2: 計數時機

**選擇**：
1. 第一次進入 deuce（40-40）時，count = 1
2. 從 Advantage 回到 deuce 時，count += 1

**理由**：
- 符合直覺：「這個 game 進入了 N 次 deuce」

### D3: FFI 暴露方式

**選擇**：在 `MatchScore` 結構新增 `deuce_count: u8` 欄位

**理由**：
- 與其他分數資訊一起返回
- 簡單直接

## Risks / Trade-offs

- **[風險] 破壞 API 相容性** → `MatchScore` 結構變更，但這是 pre-1.0，可接受
- **[風險] GameState enum 變更** → 需要更新所有 pattern matching，編譯器會檢查
