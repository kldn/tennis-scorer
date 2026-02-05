## ADDED Requirements

### Requirement: Match Set 追蹤
系統 SHALL 追蹤每位球員贏得的 set 數量。

#### Scenario: 球員贏得一個 set
- **WHEN** 一個 set 結束，某球員為 winner
- **THEN** 該球員的 match set count 加 1，並開始新的 set

### Requirement: Best-of-3 Match 勝利條件
系統 SHALL 在 Best-of-3 賽制下，當一方球員先贏得 2 個 sets 時判定為 match winner。

#### Scenario: 2-0 贏得 match
- **WHEN** 一方球員贏得第 2 個 set，對手僅有 0 個 set
- **THEN** 該球員贏得 match

#### Scenario: 2-1 贏得 match
- **WHEN** 一方球員贏得第 2 個 set，對手有 1 個 set
- **THEN** 該球員贏得 match

#### Scenario: 1-1 繼續比賽
- **WHEN** 雙方各贏得 1 個 set
- **THEN** match 繼續進入決勝盤

### Requirement: Best-of-5 Match 勝利條件
系統 SHALL 在 Best-of-5 賽制下，當一方球員先贏得 3 個 sets 時判定為 match winner。

#### Scenario: 3-0 贏得 match
- **WHEN** 一方球員贏得第 3 個 set，對手有 0 個 sets
- **THEN** 該球員贏得 match

#### Scenario: 3-2 贏得 match
- **WHEN** 一方球員贏得第 3 個 set，對手有 2 個 sets
- **THEN** 該球員贏得 match

### Requirement: 決勝盤 Tiebreak 設定
系統 SHALL 根據 MatchConfig 的 `final_set_tiebreak` 設定，決定決勝盤是否使用 tiebreak。

#### Scenario: 決勝盤啟用 tiebreak
- **WHEN** `final_set_tiebreak` 為 true，決勝盤比分達到 6-6
- **THEN** 進入 tiebreak（使用設定的目標分數）

#### Scenario: 決勝盤不啟用 tiebreak
- **WHEN** `final_set_tiebreak` 為 false，決勝盤比分達到 6-6
- **THEN** 繼續比賽直到一方領先 2 games（如 7-5、8-6 等）

### Requirement: Match 完成後不可再計分
系統 SHALL 拒絕對已完成的 match 繼續計分。

#### Scenario: 已完成的 match 嘗試計分
- **WHEN** match 已有 winner，嘗試繼續計分
- **THEN** 系統回傳原有狀態不做任何變更
