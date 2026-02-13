## Context

Matches 表目前僅記錄建立者（user_id），無對手資訊。架構設計要求支援 opponent linking：在建立比賽時記錄對手資訊，並提供 claim 機制讓對手認領比賽。

## Goals / Non-Goals

**Goals:**
- 在 matches 表儲存對手資訊（email、名稱、user_id）
- 提供 claim token 機制讓對手認領比賽
- 實作 email 自動配對（opponent_email 與註冊 email 匹配時自動連結）

**Non-Goals:**
- 不實作 claim URL 分享 UI（屬前端範疇）
- 不實作 email 通知對手（屬 push-notifications change 範疇）
- 不實作 opponent 搜尋功能

## Decisions

### D1: Claim Token 格式

**選擇**: UUID v4（透過 `Uuid::new_v4()` 產生）

**替代方案**:
- 短碼（6 位英數字）→ 碰撞風險，需額外檢查
- JWT → 過度設計，claim token 不需攜帶資料

**理由**: UUID 碰撞率極低，與現有 ID 格式一致，無需額外檢查邏輯。

### D2: 自動配對觸發時機

**選擇**: 在 `GET /api/auth/me`（使用者首次登入/建立帳號）時執行自動配對

**替代方案**:
- 背景排程 → 增加架構複雜度
- 手動觸發 → 使用者體驗差

**理由**: 使用者註冊時自動執行，即時且零延遲。查詢 `WHERE opponent_email = :email AND opponent_user_id IS NULL` 效能可接受。

### D3: Opponent 欄位設計

**選擇**: 全部 nullable 欄位加在 matches 表上

```
opponent_user_id  UUID REFERENCES users(id)  -- 已認領時填入
opponent_email    TEXT                         -- 建立時填入
opponent_name     TEXT                         -- 建立時填入
opponent_claim_token UUID UNIQUE              -- 認領前存在，認領後清除
```

**理由**: 簡單直接，避免額外的 junction table。Nullable 設計適合漸進式填入（先有 email/name，認領後填 user_id）。

## Risks / Trade-offs

**Claim token 洩漏** → 知道 token 的人都能認領。可接受風險，因 token 為 UUID 不可猜測。

**同一比賽重複認領** → claim handler 需檢查 opponent_user_id 是否已填入，已認領則拒絕。

**Opponent email 變更** → 自動配對基於 email，若使用者更換 email 則無法配對。可透過 claim token 手動認領。
