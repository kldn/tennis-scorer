## ADDED Requirements

### Requirement: 主畫面顯示比分
系統 SHALL 在主畫面清楚顯示當前比分。

#### Scenario: 顯示 Sets 比分
- **WHEN** 進入主畫面
- **THEN** 顯示雙方 sets 數（如 "Sets: 1 - 0"）

#### Scenario: 顯示 Games 比分
- **WHEN** 進入主畫面
- **THEN** 顯示當前 set 的 games 數（如 "Games: 5 - 4"）

#### Scenario: 顯示當前 Game 分數
- **WHEN** 進入主畫面
- **THEN** 以大字顯示當前 game 分數（如 "40 - 30"）

#### Scenario: Tiebreak 顯示
- **WHEN** 處於 tiebreak 狀態
- **THEN** 顯示數字分數（如 "6 - 5"）並標示 "Tiebreak"

#### Scenario: Deuce 顯示
- **WHEN** 處於 Deuce 狀態
- **THEN** 顯示 "Deuce"

#### Scenario: Advantage 顯示
- **WHEN** 某方有 Advantage
- **THEN** 顯示 "Ad - 40" 或 "40 - Ad"

### Requirement: 計分按鈕
系統 SHALL 提供兩個計分按鈕供觸控操作。

#### Scenario: Player 1 按鈕
- **WHEN** 點擊 Player 1 按鈕
- **THEN** Player 1 得一分，畫面更新

#### Scenario: Player 2 按鈕
- **WHEN** 點擊 Player 2 按鈕
- **THEN** Player 2 得一分，畫面更新

#### Scenario: 按鈕視覺回饋
- **WHEN** 點擊按鈕
- **THEN** 提供 haptic feedback 和視覺回饋

### Requirement: Undo 按鈕
系統 SHALL 提供 Undo 按鈕回退得分。

#### Scenario: Undo 可用
- **WHEN** 有歷史記錄
- **THEN** Undo 按鈕可點擊

#### Scenario: Undo 不可用
- **WHEN** 無歷史記錄（match 剛開始）
- **THEN** Undo 按鈕 disabled 或隱藏

#### Scenario: 防止誤觸
- **WHEN** 點擊 Undo 按鈕
- **THEN** 顯示確認提示或要求長按確認

### Requirement: Match 結束畫面
系統 SHALL 在 match 結束時顯示結果。

#### Scenario: 顯示 Winner
- **WHEN** match 結束
- **THEN** 顯示 "Player X Wins!" 和最終比分

#### Scenario: 新 Match 按鈕
- **WHEN** match 結束
- **THEN** 提供 "New Match" 按鈕開始新比賽

### Requirement: 設定畫面
系統 SHALL 提供設定畫面配置 match 格式。

#### Scenario: 選擇賽制
- **WHEN** 進入設定
- **THEN** 可選擇 Best-of-3 或 Best-of-5

#### Scenario: 選擇 Scoring 模式
- **WHEN** 進入設定
- **THEN** 可選擇 Ad scoring 或 No-Ad scoring
