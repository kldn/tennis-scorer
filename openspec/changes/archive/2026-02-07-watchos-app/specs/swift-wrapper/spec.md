## ADDED Requirements

### Requirement: TennisMatch Swift Class
系統 SHALL 提供 `TennisMatch` class 封裝 Rust FFI 呼叫。

#### Scenario: 建立預設 match
- **WHEN** 呼叫 `TennisMatch()` 無參數
- **THEN** 建立 Best-of-3、Ad scoring、7 分 tiebreak 的 match

#### Scenario: 建立自訂 match
- **WHEN** 呼叫 `TennisMatch(config:)` 傳入自訂設定
- **THEN** 建立對應設定的 match

#### Scenario: 物件釋放時自動清理
- **WHEN** `TennisMatch` instance 被釋放（deinit）
- **THEN** 自動呼叫 `tennis_match_free()` 釋放 Rust 記憶體

### Requirement: Player Enum
系統 SHALL 提供 Swift-native `Player` enum。

#### Scenario: Player 定義
- **GIVEN** `Player` enum
- **THEN** 包含 `.player1` 和 `.player2` 兩個 case

### Requirement: 計分功能
系統 SHALL 透過 `scorePoint(player:)` 方法記錄得分。

#### Scenario: 計分後狀態更新
- **WHEN** 呼叫 `match.scorePoint(player: .player1)`
- **THEN** FFI 被呼叫，`score` property 更新，UI 自動刷新

### Requirement: Undo 功能
系統 SHALL 透過 `undo()` 方法回退上一個得分。

#### Scenario: Undo 成功
- **WHEN** 有歷史記錄，呼叫 `match.undo()`
- **THEN** 狀態回退，`score` property 更新

#### Scenario: 檢查是否可 Undo
- **WHEN** 存取 `match.canUndo`
- **THEN** 回傳 Bool 表示是否可回退

### Requirement: Score Property
系統 SHALL 透過 `score` property 提供當前比分。

#### Scenario: Score 包含完整資訊
- **GIVEN** `match.score`
- **THEN** 包含 `player1Sets`、`player2Sets`、`player1Games`、`player2Games`、`player1Points`、`player2Points`、`isTiebreak`、`isDeuce`、`hasAdvantage`、`winner`

### Requirement: ObservableObject 支援
系統 SHALL 讓 `TennisMatch` 實作 `ObservableObject`。

#### Scenario: SwiftUI 自動更新
- **WHEN** `score` 改變
- **THEN** 使用 `@Published` 或類似機制通知 SwiftUI 更新
