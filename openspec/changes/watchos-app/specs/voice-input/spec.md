## ADDED Requirements

### Requirement: 語音辨識啟動
系統 SHALL 提供方式啟動語音辨識。

#### Scenario: 按鈕啟動
- **WHEN** 點擊麥克風按鈕
- **THEN** 開始語音辨識，顯示聆聽狀態

#### Scenario: 釋放停止
- **WHEN** 放開麥克風按鈕
- **THEN** 停止辨識，處理辨識結果

### Requirement: Player 1 語音指令
系統 SHALL 辨識表示 Player 1 得分的語音指令。

#### Scenario: 標準指令
- **WHEN** 辨識到 "Player one"
- **THEN** Player 1 得一分

#### Scenario: 簡短指令
- **WHEN** 辨識到 "One" 或 "First"
- **THEN** Player 1 得一分

### Requirement: Player 2 語音指令
系統 SHALL 辨識表示 Player 2 得分的語音指令。

#### Scenario: 標準指令
- **WHEN** 辨識到 "Player two"
- **THEN** Player 2 得一分

#### Scenario: 簡短指令
- **WHEN** 辨識到 "Two" 或 "Second"
- **THEN** Player 2 得一分

### Requirement: Undo 語音指令
系統 SHALL 辨識 Undo 語音指令。

#### Scenario: Undo 指令
- **WHEN** 辨識到 "Undo" 或 "Cancel"
- **THEN** 回退上一個得分

### Requirement: 辨識回饋
系統 SHALL 提供語音辨識的回饋。

#### Scenario: 成功辨識
- **WHEN** 成功辨識指令
- **THEN** 提供 haptic feedback 和視覺確認

#### Scenario: 辨識失敗
- **WHEN** 無法辨識指令
- **THEN** 顯示錯誤提示（如 "Didn't catch that"）

### Requirement: 權限處理
系統 SHALL 正確處理語音辨識權限。

#### Scenario: 首次使用
- **WHEN** 首次啟動語音辨識
- **THEN** 請求麥克風和語音辨識權限

#### Scenario: 權限被拒
- **WHEN** 使用者拒絕權限
- **THEN** 隱藏語音功能，僅顯示觸控介面
