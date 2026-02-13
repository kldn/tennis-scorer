## Context

現有 handler 直接包含 SQL 查詢和業務邏輯。例如 `create_match` handler 執行交易、插入 match/events；`summary` handler 執行統計 SQL。架構設計要求提取 Service Layer，使 handler 僅處理 HTTP 關注點。

## Goals / Non-Goals

**Goals:**
- 提取 AuthService、MatchService、StatsService
- Handler 變為薄層（parse request → call service → return response）
- 新增速率限制 middleware
- 建立 Service trait 模式供後續模組遵循

**Non-Goals:**
- 不改變任何 API 外部行為
- 不引入 DI 框架（直接用 PgPool 注入即可）
- 不建立 Repository 層（Service 直接用 sqlx，避免過度抽象）

## Decisions

### D1: Service 注入方式

**選擇**: Service struct 持有 `PgPool`，在 AppState 中建立並共享

```rust
pub struct MatchService { pool: PgPool }

pub struct AppState {
    pub pool: PgPool,
    pub match_service: MatchService,
    pub auth_service: AuthService,
    pub stats_service: StatsService,
}
```

**替代方案**:
- Trait object (`dyn Service`) → 增加複雜度，目前不需 mock
- 每次 request 建立 service → 不必要的開銷

**理由**: 簡單直接，Clone 成本低（PgPool 內部是 Arc），與 Axum 的 State 機制完美契合。

### D2: 速率限制策略

**選擇**: Tower `RateLimitLayer`，全局 100 requests/second

**替代方案**:
- Per-IP（需自訂 middleware 提取 IP）→ Phase 1 先用全局，Phase 2 加 per-IP
- Per-user（需在 auth 後限制）→ 增加複雜度

**理由**: 全局速率限制最簡單，tower 原生支援。後續可按需求改為 per-IP。

### D3: 提取策略

**選擇**: 逐模組提取（auth → matches → stats），每個模組獨立 PR 可驗證

**理由**: 減少單次變更範圍，每步驟都可獨立測試驗證。

## Risks / Trade-offs

**過度抽象** → 保持 Service 直接用 sqlx，不再加 Repository 層。

**AppState 膨脹** → 可接受，目前只有 3 個 service。未來 SocialService、NotificationService 加入後若過大，可考慮 nested state。
