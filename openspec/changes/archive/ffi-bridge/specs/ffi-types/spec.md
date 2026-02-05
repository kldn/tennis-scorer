## ADDED Requirements

### Requirement: Opaque Match Handle
系統 SHALL 提供 opaque pointer type `TennisMatch` 作為 match 的 handle。

#### Scenario: Handle 不透露內部結構
- **WHEN** Swift 端取得 `TennisMatch*`
- **THEN** 無法直接存取內部欄位，只能透過 API 函數操作

### Requirement: Player 常數定義
系統 SHALL 定義 `PLAYER_1` (1) 和 `PLAYER_2` (2) 常數。

#### Scenario: 使用 Player 常數
- **WHEN** 呼叫 `tennis_match_score_point(match, PLAYER_1)`
- **THEN** Player 1 得分

### Requirement: MatchScore 結構
系統 SHALL 提供 `MatchScore` C struct 包含完整的比賽狀態。

#### Scenario: MatchScore 欄位
- **WHEN** 呼叫 `tennis_match_get_score(match)`
- **THEN** 回傳包含以下欄位的 struct:
  - `player1_sets`, `player2_sets`: 雙方贏得的 set 數
  - `player1_games`, `player2_games`: 當前 set 的 game 數
  - `player1_points`, `player2_points`: 當前 game 的分數
  - `is_tiebreak`: 是否在 tiebreak 中
  - `game_state`: 當前 game 狀態 (playing/deuce/advantage/completed)
  - `winner`: match winner (0=無, 1=P1, 2=P2)

### Requirement: GameState 編碼
系統 SHALL 將 GameState 編碼為 uint8_t。

#### Scenario: GameState 數值定義
- `0` = Playing (一般得分中)
- `1` = Deuce
- `2` = Advantage Player 1
- `3` = Advantage Player 2
- `4` = Game Completed
