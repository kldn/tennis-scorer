## ADDED Requirements

### Requirement: Set 內 Game 計分追蹤
系統 SHALL 追蹤一個 set 內每位球員贏得的 game 數。

#### Scenario: 球員贏得一個 game
- **WHEN** 一個 game 結束，某球員為 winner
- **THEN** 該球員的 set game count 加 1，並開始新的 game

### Requirement: 標準 Set 勝利條件
系統 SHALL 在一方球員拿到 6 games 且領先對手至少 2 games 時判定該球員贏得此 set。

#### Scenario: 6-4 贏得 set
- **WHEN** 一方球員拿到第 6 個 game，對手僅有 4 個或更少的 games
- **THEN** 該球員贏得此 set

#### Scenario: 6-5 時繼續比賽
- **WHEN** 比分為 6-5
- **THEN** set 繼續進行（不觸發 tiebreak），落後方若追平則進入 6-6

#### Scenario: 7-5 贏得 set
- **WHEN** 比分從 6-5 變為 7-5
- **THEN** 領先方贏得此 set

### Requirement: Tiebreak 觸發
系統 SHALL 在 set 比分達到 6-6 時自動觸發 tiebreak。

#### Scenario: 進入 tiebreak
- **WHEN** set 比分達到 6-6
- **THEN** 下一個 game 以 tiebreak 規則進行

### Requirement: Tiebreak 計分
系統 SHALL 使用數字計分進行 tiebreak，目標分數由 MatchConfig 設定。球員 MUST 領先至少 2 分才能贏得 tiebreak。

#### Scenario: 標準 tiebreak（目標 7 分）達到目標分數
- **WHEN** tiebreak 目標為 7 分，一方球員拿到 7 分且對手不超過 5 分
- **THEN** 該球員贏得 tiebreak 並贏得此 set（比分 7-6）

#### Scenario: Tiebreak 延長（需領先 2 分）
- **WHEN** tiebreak 比分為 6-6（以目標 7 分為例）
- **THEN** 繼續比賽直到一方領先 2 分

#### Scenario: Super tiebreak（目標 10 分）
- **WHEN** tiebreak 目標為 10 分，一方球員拿到 10 分且領先至少 2 分
- **THEN** 該球員贏得 tiebreak 並贏得此 set

### Requirement: Set 完成後不可再計分
系統 SHALL 拒絕對已完成的 set 繼續計分。

#### Scenario: 已完成的 set 嘗試計分
- **WHEN** set 已有 winner，嘗試繼續計分
- **THEN** 系統回傳原有狀態不做任何變更
