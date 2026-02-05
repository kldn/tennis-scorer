## ADDED Requirements

### Requirement: 標準 Game 計分（Ad Scoring）
系統 SHALL 依照網球標準規則追蹤單個 game 內的得分：Love (0) → 15 → 30 → 40 → Game。當雙方都到達 40 時進入 Deuce 狀態。

#### Scenario: 一般得分推進
- **WHEN** 一方球員得分且分數未到達 40-40
- **THEN** 該球員分數按 Love → 15 → 30 → 40 推進

#### Scenario: 直接拿下 Game（非 Deuce）
- **WHEN** 一方球員分數為 40 且對手分數低於 40，該球員再得一分
- **THEN** 該球員贏得此 game

#### Scenario: 進入 Deuce
- **WHEN** 雙方球員分數皆為 40
- **THEN** game 狀態變為 Deuce

#### Scenario: 取得 Advantage
- **WHEN** game 處於 Deuce 狀態，一方球員得分
- **THEN** 該球員取得 Advantage

#### Scenario: 從 Advantage 贏得 Game
- **WHEN** 擁有 Advantage 的球員再次得分
- **THEN** 該球員贏得此 game

#### Scenario: 從 Advantage 回到 Deuce
- **WHEN** 未擁有 Advantage 的球員得分
- **THEN** game 狀態回到 Deuce

### Requirement: No-Ad Game 計分
系統 SHALL 支援 No-Ad scoring 模式，Deuce 時一球定勝負，不需 Advantage。

#### Scenario: No-Ad 模式下的 Deuce 得分
- **WHEN** No-Ad 模式啟用，雙方分數皆為 40（Deuce），一方球員得分
- **THEN** 該球員直接贏得此 game，不進入 Advantage 狀態

### Requirement: Game 完成後不可再計分
系統 SHALL 拒絕對已完成的 game 繼續計分。

#### Scenario: 已完成的 game 嘗試計分
- **WHEN** game 已有 winner，嘗試再次呼叫 score_point
- **THEN** 系統回傳原有狀態不做任何變更
