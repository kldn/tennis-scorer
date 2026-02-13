## Context

架構設計定義了好友系統，包含好友請求、好友關係、對戰記錄。此模組是 Phase 2 的核心功能，需在 service-layer-extraction 和 opponent-linking 完成後實作。

## Goals / Non-Goals

**Goals:**
- 實作好友請求流程（發送、接受、拒絕）
- 雙向儲存好友關係
- 查詢好友清單、好友比賽記錄、head-to-head 統計
- 遵循 Service Layer 模式

**Non-Goals:**
- 不實作封鎖功能（Phase 3 考慮）
- 不實作好友推薦
- 不實作即時聊天

## Decisions

### D1: 好友關係儲存方式

**選擇**: 雙向儲存（A→B 和 B→A 各一筆記錄），UNIQUE(user_id, friend_id)

```sql
CREATE TABLE friendships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    friend_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(user_id, friend_id)
);
```

**替代方案**:
- 單向儲存 + 查詢時 OR → 查詢複雜度高，index 不好用
- 中間表含 status → 與 friend_requests 重複

**理由**: 雙向儲存查詢最簡單（`WHERE user_id = :me`），index 效率高。Accept 時同時插入兩筆。

### D2: 好友請求欄位

**選擇**: `status` enum（pending/accepted/rejected）

```sql
CREATE TABLE friend_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_user_id UUID NOT NULL REFERENCES users(id),
    to_user_id UUID NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(from_user_id, to_user_id)
);
```

**理由**: 簡單清晰，UNIQUE constraint 防止重複請求。

### D3: Head-to-head 計算

**選擇**: 即時查詢（基於 matches 表的 user_id 和 opponent_user_id）

**替代方案**:
- 預計算快取 → 增加複雜度，資料量小不需要

**理由**: 查詢 `WHERE (user_id = :a AND opponent_user_id = :b) OR (user_id = :b AND opponent_user_id = :a)` 效能足夠。

## Risks / Trade-offs

**好友數量無上限** → 可接受，暫不限制。未來可加 limit。

**Rejected 後重新發送** → 允許。刪除舊的 rejected 記錄，重新建立 pending 請求。
