## Context

match-stats 依賴於 game-history 功能。一旦有了 `MatchHistoryStore` 和 `CompletedMatch` 記錄，就可以計算統計數據。

使用者場景：
- Player 1 = 我（使用者自己）
- Player 2 = 對手
- 所以「我贏」= winner == 1，「對手贏」= winner == 2

## Goals / Non-Goals

**Goals:**
- 顯示「我」的總勝場數
- 顯示「我」的總敗場數
- 顯示勝率百分比
- 在 UI 上有一個地方可以看到統計

**Non-Goals:**
- 不按對手分類統計（目前沒有對手識別）
- 不顯示 set 統計或 game 統計
- 不顯示時間趨勢圖表

## Decisions

### D1: 統計計算位置

**選擇**：在 `MatchHistoryStore` 新增計算方法

**理由**：
- 統計基於歷史資料，放在同一處合理
- 避免在 UI 層做複雜計算

### D2: 統計結構

**選擇**：
```swift
struct MatchStats {
    let totalMatches: Int
    let wins: Int      // Player 1 (我) wins
    let losses: Int    // Player 2 (對手) wins
    var winRate: Double {
        totalMatches > 0 ? Double(wins) / Double(totalMatches) : 0
    }
}
```

### D3: UI 顯示位置

**選擇**：在比賽完成畫面下方顯示簡易統計

**理由**：
- 不干擾主要的記分介面
- 比賽結束是查看統計的自然時機
- 保持 watchOS 介面簡潔

### D4: 顯示格式

**選擇**：
```
戰績: 5勝 3敗 (62%)
```

**理由**：
- 簡潔明瞭
- 中文介面一致性

## Risks / Trade-offs

- **[風險] 依賴 game-history** → 需確保 game-history 先實作
- **[風險] 統計計算效能** → 歷史最多 100 場，即使每次計算也很快
