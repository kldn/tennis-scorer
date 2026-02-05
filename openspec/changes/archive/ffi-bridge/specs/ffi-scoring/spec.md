## ADDED Requirements

### Requirement: 計分功能
系統 SHALL 提供 `tennis_match_score_point()` 記錄得分。

#### Scenario: 正常計分
- **WHEN** 呼叫 `tennis_match_score_point(match, PLAYER_1)`
- **THEN** Player 1 得一分，match 狀態更新，回傳 true

#### Scenario: Match 已結束時計分
- **WHEN** match 已有 winner，呼叫 `tennis_match_score_point()`
- **THEN** 狀態不變，回傳 true（操作成功但無效果）

#### Scenario: Null match 計分
- **WHEN** 呼叫 `tennis_match_score_point(NULL, PLAYER_1)`
- **THEN** 回傳 false

#### Scenario: 無效 player 值
- **WHEN** 呼叫 `tennis_match_score_point(match, 0)` 或 player > 2
- **THEN** 回傳 false，狀態不變

### Requirement: Undo 功能
系統 SHALL 提供 `tennis_match_undo()` 回退上一個得分。

#### Scenario: 正常 undo
- **WHEN** history 不為空，呼叫 `tennis_match_undo(match)`
- **THEN** 狀態回退到上一個得分前，回傳 true

#### Scenario: 無法 undo（history 為空）
- **WHEN** history 為空，呼叫 `tennis_match_undo(match)`
- **THEN** 狀態不變，回傳 true（操作成功但無效果）

#### Scenario: Null match undo
- **WHEN** 呼叫 `tennis_match_undo(NULL)`
- **THEN** 回傳 false
