## ADDED Requirements

### Requirement: 記錄得分歷史
系統 SHALL 在每次 score_point 時自動將前一個 state 保存到 history stack。

#### Scenario: 得分後保存歷史
- **WHEN** 呼叫 score_point 且 match 尚未結束
- **THEN** 前一個 MatchState 被 push 到 history stack，current state 更新為新狀態

#### Scenario: Match 已結束時不記錄
- **WHEN** match 已有 winner，嘗試 score_point
- **THEN** history stack 不變，state 不變

### Requirement: Undo 上一個得分
系統 SHALL 支援 undo 操作，回退到上一個 state。

#### Scenario: 成功 undo
- **WHEN** history stack 不為空，呼叫 undo
- **THEN** current state 回復為 history stack 最頂端的 state，該 state 從 stack 中移除

#### Scenario: 無法 undo（history 為空）
- **WHEN** history stack 為空（例如 match 剛開始），呼叫 undo
- **THEN** state 不變，操作無效果

### Requirement: 連續 Undo
系統 SHALL 支援連續多次 undo，可回退到 match 起始狀態。

#### Scenario: 連續 undo 多個得分
- **WHEN** history stack 有 N 個 state，連續呼叫 N 次 undo
- **THEN** state 回到 match 的初始狀態，history stack 為空

### Requirement: Undo 後重新計分
系統 SHALL 允許在 undo 之後繼續計分，新的得分會覆蓋原有的後續歷史。

#### Scenario: Undo 後得分
- **WHEN** 執行 undo 回退到某個 state，然後呼叫 score_point
- **THEN** 新的 state 基於 undo 後的 state 計算，之前被 undo 的後續歷史不會恢復
