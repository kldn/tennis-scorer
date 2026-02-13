## Context

架構設計要求整合 FCM 推播通知，在社交事件（好友請求、比賽結果、對手認領）發生時即時通知使用者。需支援多裝置類型（Apple Watch、Wear OS、Android Phone、iOS Phone）。

## Goals / Non-Goals

**Goals:**
- 實作 device token 註冊與管理
- 整合 FCM HTTP v1 API 發送推播
- 實作三種通知觸發情境
- 支援多裝置同時收到通知

**Non-Goals:**
- 不實作通知排程（僅即時推送）
- 不實作豐富通知（圖片、動作按鈕等）
- 不實作通知歷史記錄

## Decisions

### D1: FCM API 版本

**選擇**: FCM HTTP v1 API（`https://fcm.googleapis.com/v1/projects/{project}/messages:send`）

**替代方案**:
- Legacy HTTP API → Google 已標記為 deprecated
- FCM Admin SDK → Rust 無官方 SDK

**理由**: v1 API 是 Google 推薦版本，支援 OAuth 2.0 認證，文件完整。

### D2: Firebase 認證方式

**選擇**: Service Account JSON key + OAuth 2.0 access token

```
FIREBASE_SERVICE_KEY (JSON string) → parse → sign JWT → exchange for access token
Access token cache with auto-refresh (expires in 1 hour)
```

**理由**: 標準 Google Cloud 認證方式，安全且可靠。

### D3: Device Token 去重

**選擇**: UPSERT on (user_id, device_type)，同一使用者同一裝置類型只保留最新 token

```sql
CREATE TABLE device_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    fcm_token TEXT NOT NULL,
    device_type TEXT NOT NULL CHECK (device_type IN ('watch_apple', 'watch_wearos', 'phone_android', 'phone_ios')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(user_id, device_type)
);
```

**替代方案**:
- 允許多 token per device type → 可能累積過時 token
- 以 fcm_token 為 unique key → 同裝置重裝後 token 變更會累積

**理由**: 一個裝置類型一個有效 token，簡單且防止 stale token 累積。

### D4: 通知失敗處理

**選擇**: Fire-and-forget + log error。FCM 回傳 404（token 無效）時刪除該 token。

**理由**: 通知不是關鍵路徑，失敗不應影響主要操作（如接受好友請求）。Log 用於監控。

## Risks / Trade-offs

**FCM quota** → Free tier 足夠（每天可發大量通知），暫無風險。

**Service key 安全** → 透過環境變數注入，不存入 code。Railway secrets 管理。

**通知延遲** → 同步呼叫 FCM API 會增加 response time。可考慮用 `tokio::spawn` 非同步發送。
