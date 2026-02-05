## ADDED Requirements

### Requirement: 查詢完整分數
系統 SHALL 提供 `tennis_match_get_score()` 取得完整比賽狀態。

#### Scenario: 取得分數
- **WHEN** 呼叫 `tennis_match_get_score(match)`
- **THEN** 回傳填充完整的 `MatchScore` struct

#### Scenario: Null match 查詢
- **WHEN** 呼叫 `tennis_match_get_score(NULL)`
- **THEN** 回傳全零的 `MatchScore`

### Requirement: 查詢是否可 Undo
系統 SHALL 提供 `tennis_match_can_undo()` 檢查是否可回退。

#### Scenario: 有 history 時
- **WHEN** history 不為空
- **THEN** `tennis_match_can_undo(match)` 回傳 true

#### Scenario: 無 history 時
- **WHEN** history 為空（match 剛開始）
- **THEN** `tennis_match_can_undo(match)` 回傳 false

#### Scenario: Null match
- **WHEN** 呼叫 `tennis_match_can_undo(NULL)`
- **THEN** 回傳 false

### Requirement: 查詢 Match 是否結束
系統 SHALL 提供 `tennis_match_is_complete()` 檢查 match 是否已結束。

#### Scenario: Match 進行中
- **WHEN** match 尚未有 winner
- **THEN** `tennis_match_is_complete(match)` 回傳 false

#### Scenario: Match 已結束
- **WHEN** match 已有 winner
- **THEN** `tennis_match_is_complete(match)` 回傳 true

### Requirement: 查詢 Winner
系統 SHALL 提供 `tennis_match_get_winner()` 取得 winner。

#### Scenario: 有 winner
- **WHEN** match 已結束
- **THEN** `tennis_match_get_winner(match)` 回傳 1 (P1) 或 2 (P2)

#### Scenario: 無 winner
- **WHEN** match 進行中
- **THEN** `tennis_match_get_winner(match)` 回傳 0
