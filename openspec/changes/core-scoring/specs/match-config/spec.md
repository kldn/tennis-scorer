## ADDED Requirements

### Requirement: Match 賽制設定
系統 SHALL 提供 MatchConfig 型別，允許設定 match 的賽制參數。

#### Scenario: 建立 Best-of-3 設定
- **WHEN** 使用者建立 MatchConfig 並設定 sets_to_win 為 2
- **THEN** match 以 Best-of-3（三盤兩勝）進行

#### Scenario: 建立 Best-of-5 設定
- **WHEN** 使用者建立 MatchConfig 並設定 sets_to_win 為 3
- **THEN** match 以 Best-of-5（五盤三勝）進行

### Requirement: Tiebreak 目標分數設定
系統 SHALL 允許設定 tiebreak 的目標分數，預設為 7 分。

#### Scenario: 標準 7 分 tiebreak
- **WHEN** tiebreak_points 設定為 7
- **THEN** tiebreak 以先到 7 分（需領先 2 分）進行

#### Scenario: Super tiebreak 10 分
- **WHEN** tiebreak_points 設定為 10
- **THEN** tiebreak 以先到 10 分（需領先 2 分）進行

#### Scenario: 自訂目標分數
- **WHEN** tiebreak_points 設定為任意正整數
- **THEN** tiebreak 以先到該分數（需領先 2 分）進行

### Requirement: 決勝盤 Tiebreak 開關
系統 SHALL 允許設定決勝盤是否使用 tiebreak，預設為啟用。

#### Scenario: 啟用決勝盤 tiebreak
- **WHEN** final_set_tiebreak 設定為 true
- **THEN** 決勝盤在 6-6 時進入 tiebreak

#### Scenario: 停用決勝盤 tiebreak
- **WHEN** final_set_tiebreak 設定為 false
- **THEN** 決勝盤不使用 tiebreak，必須領先 2 games 才能贏得 set

### Requirement: Scoring Mode 設定
系統 SHALL 允許在 Ad scoring 與 No-Ad scoring 之間切換，預設為 Ad scoring。

#### Scenario: Ad scoring 模式
- **WHEN** no_ad_scoring 設定為 false
- **THEN** game 使用標準 Deuce / Advantage 規則

#### Scenario: No-Ad scoring 模式
- **WHEN** no_ad_scoring 設定為 true
- **THEN** game 在 Deuce 時一球定勝負

### Requirement: 預設設定
系統 SHALL 提供合理的預設設定值。

#### Scenario: 使用預設值
- **WHEN** 使用者建立 MatchConfig 未指定任何參數
- **THEN** 預設為 Best-of-3（sets_to_win = 2）、tiebreak 7 分、決勝盤啟用 tiebreak、Ad scoring
