## ADDED Requirements

### Requirement: 建立預設 Match
系統 SHALL 提供 `tennis_match_new_default()` 建立 Best-of-3、Ad scoring 的 match。

#### Scenario: 建立預設 match
- **WHEN** 呼叫 `tennis_match_new_default()`
- **THEN** 回傳有效的 `TennisMatch*`，設定為 Best-of-3、7 分 tiebreak、Ad scoring

### Requirement: 建立 Best-of-5 Match
系統 SHALL 提供 `tennis_match_new_best_of_5()` 建立 Best-of-5 match。

#### Scenario: 建立 Best-of-5 match
- **WHEN** 呼叫 `tennis_match_new_best_of_5()`
- **THEN** 回傳 Best-of-5 設定的 match

### Requirement: 建立 No-Ad Match
系統 SHALL 提供 `tennis_match_new_no_ad()` 建立 No-Ad scoring match。

#### Scenario: 建立 No-Ad match
- **WHEN** 呼叫 `tennis_match_new_no_ad()`
- **THEN** 回傳 No-Ad scoring 的 Best-of-3 match

### Requirement: 建立自訂 Match
系統 SHALL 提供 `tennis_match_new_custom()` 允許完整自訂設定。

#### Scenario: 建立自訂 match
- **WHEN** 呼叫 `tennis_match_new_custom(3, 10, false, true)`
- **THEN** 回傳 Best-of-5、super tiebreak 10 分、決勝盤無 tiebreak、No-Ad 的 match

### Requirement: 釋放 Match
系統 SHALL 提供 `tennis_match_free()` 釋放 match 佔用的記憶體。

#### Scenario: 釋放 match
- **WHEN** 呼叫 `tennis_match_free(match)`
- **THEN** match 的記憶體被釋放，指標不再有效

#### Scenario: 釋放 null 是安全的
- **WHEN** 呼叫 `tennis_match_free(NULL)`
- **THEN** 不發生錯誤（no-op）
