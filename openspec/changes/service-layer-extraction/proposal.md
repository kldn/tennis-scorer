## Why

目前後端 API 的 handler 函數直接包含資料庫查詢和業務邏輯（Argon2 雜湊、SQL 交易、統計計算等），形成 Router → Handlers → DB 的兩層架構。這造成：

1. **職責不分離**：handler 同時處理 HTTP 關注點與業務邏輯
2. **無法複用**：業務邏輯綁死在 handler 中，未來 Social 和 Notification 模組無法呼叫已有邏輯
3. **測試困難**：測試業務邏輯必須經由 HTTP 層
4. **缺乏速率限制**：API 沒有任何速率限制保護

架構設計文件定義目標為四層架構：Middleware → Router → Service → Data Access。

## What Changes

- 從現有 handler 中提取業務邏輯至獨立 Service Layer：`AuthService`、`MatchService`、`StatsService`
- Handler 變為薄層：解析請求參數 → 呼叫 service → 回傳 HTTP response
- 新增 tower-based 速率限制 middleware
- 為未來 SocialService、NotificationService 建立可遵循的模式

## Capabilities

### New Capabilities
- `service-layer`: Service trait 定義與實作，封裝業務邏輯與資料存取
- `rate-limiting`: Tower middleware 實現的 API 速率限制

### Modified Capabilities
- `api-auth`: handler 內部重構為呼叫 AuthService（外部行為不變）
- `api-matches`: handler 內部重構為呼叫 MatchService（外部行為不變）
- `api-stats`: handler 內部重構為呼叫 StatsService（外部行為不變）

## Impact

- **Code**: 新增 `auth/service.rs`、`matches/service.rs`、`stats/service.rs`；修改所有 handler 檔案；修改 `lib.rs`
- **Dependencies**: 新增 `tower` crate（rate limiting）
- **External behavior**: 所有 API 端點行為不變（純內部重構），唯一可觀察的變化是速率限制（429 Too Many Requests）
- **Existing tests**: 現有整合測試應全部通過
