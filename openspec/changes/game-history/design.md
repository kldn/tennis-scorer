## Context

目前 `TennisMatch` 類別在呼叫 `newMatch()` 時直接釋放舊的 FFI handle 並建立新的。已完成的比賽資訊會完全丟失。watchOS 有 UserDefaults 和 FileManager 可用於持久化。

## Goals / Non-Goals

**Goals:**
- 自動儲存已完成的比賽
- 儲存基本比賽資訊（最終比分、時間戳記、贏家）
- 提供讀取歷史記錄的 API

**Non-Goals:**
- 不儲存每一分的完整歷史（太複雜）
- 不同步到 iCloud（未來功能）
- 不提供編輯或刪除歷史的功能（v1 簡化）
- 不儲存進行中的比賽（只儲存完成的）

## Decisions

### D1: 儲存位置

**選擇**：使用 UserDefaults 儲存 JSON 編碼的歷史記錄

**理由**：
- watchOS 上 UserDefaults 簡單可靠
- 資料量不大（每場比賽約 100 bytes）
- 不需要 CloudKit 設定

**替代方案**：
- FileManager Documents 目錄 → 較複雜
- Core Data → 過度設計
- SwiftData → 需要 watchOS 10+

### D2: 資料結構

**選擇**：
```swift
struct CompletedMatch: Codable {
    let id: UUID
    let date: Date
    let winner: Int  // 1 or 2
    let player1Sets: Int
    let player2Sets: Int
}
```

**理由**：
- 只儲存最終結果，簡單明瞭
- Codable 方便 JSON 序列化
- UUID 方便未來擴充（刪除、同步）

### D3: 儲存時機

**選擇**：在 `newMatch()` 被呼叫時，如果當前比賽已完成，則自動儲存

**理由**：
- 不需要額外的「儲存」按鈕
- 只儲存完成的比賽，避免儲存進行中的狀態

### D4: 歷史限制

**選擇**：最多保留 100 場歷史

**理由**：
- 防止 UserDefaults 無限增長
- 對一般使用者足夠
- 先進先出（刪除最舊的）

## Risks / Trade-offs

- **[風險] UserDefaults 有大小限制** → 限制 100 場 + 每場約 100 bytes = 約 10KB，遠低於限制
- **[風險] 資料可能在 app 重裝後丟失** → UserDefaults 預設會隨 app 保留，可接受
