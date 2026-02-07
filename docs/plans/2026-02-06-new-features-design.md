# New Features Design

## Overview

Additional features and technical enhancements discussed for tennis-scorer, building on top of the backend & CI/CD plan.

## Updated Architecture

```
┌─────────────────┐     ┌──────────────────────┐     ┌────────────┐
│  Apple Watch     │────▶│  Rust API (Axum)      │────▶│ PostgreSQL │
│  (Swift + UniFFI)│ HTTP│                      │     │            │
│  + SwiftData     │     │  tennis-scorer crate  │     └────────────┘
│  + Widget        │     │  (shared types)       │
│  + Voice input   │     │  + 統計分析           │
└─────────────────┘     └──────────────────────┘
                                  ▲
┌─────────────────┐               │
│  Flutter App     │──────────────┘
│  (iOS/Android/   │ HTTP
│   Web)           │
│  + Momentum Chart│        Deployed on Shuttle.rs
│  + Match History │
│  + Statistics    │
└─────────────────┘
```

**Frontend strategy:**
- **Watch App (Swift)** — scoring tool (primary input device)
- **Flutter App (iOS/Android/Web)** — match history, statistics, momentum charts
- **Rust API** — 統計分析 endpoints，時間序列計算，momentum 數據

---

## Feature 1: Event Timestamps

### Decision
Timestamps built into Rust core engine (not frontend layer).

### Rationale
- History/undo already tracks state changes; timestamp is a natural addition
- All frontends (Watch, Flutter, future) automatically inherit timestamps
- No format inconsistency between frontends

### Implementation

```rust
// In history.rs — extend HistoryEntry
pub struct HistoryEntry {
    pub player: Player,
    pub timestamp: std::time::SystemTime,  // NEW
    pub state_before: MatchState,
}
```

Update `score_point()` to record `SystemTime::now()` with each point.

### Updated points format (for API/DB)

```json
[
  {"player": 1, "timestamp": "2026-02-06T14:30:15.123Z"},
  {"player": 2, "timestamp": "2026-02-06T14:32:41.456Z"}
]
```

### Enabled analytics
- Per-point interval (pace of play)
- Per-game / per-set duration
- Time-based performance analysis (first 20 min vs last 20 min)
- Scoring streak speed

---

## Feature 2: Match Replay & Analysis

### Data Model — PointContext

Replay the point sequence through the scoring engine, annotating each point with context:

```rust
pub struct PointContext {
    pub point_number: u32,
    pub player: Player,
    pub timestamp: SystemTime,

    // Scoring context (computed from replay)
    pub score_before: MatchScore,
    pub is_break_point: bool,
    pub is_game_point: bool,
    pub is_set_point: bool,
    pub is_match_point: bool,
    pub serving_player: Player,
    pub game_number_in_set: u32,
    pub set_number: u32,
}

// New API in core engine
pub fn replay_with_context(
    config: &MatchConfig,
    points: &[(Player, SystemTime)]
) -> Vec<PointContext>
```

### Key Statistics

**Break Points:**
- `break_points_created` — break points I created on opponent's serve
- `break_points_converted` — how many I won
- `break_points_faced` — break points opponent created on my serve
- `break_points_saved` — how many I saved

**Game/Set/Match Points:**
- Conversion rate at game point, set point, match point

**Deuce Analysis:**
- Average deuces per deuce game
- Win rate in deuce games vs non-deuce games
- (Leverages existing deuce counter feature)

### Momentum Chart

**Basic formula:**
```
momentum[i] = momentum[i-1] + (player == Player1 ? +1 : -1)
```

**Weighted formula (advanced):**
```
weight = match context {
    break_point converted => 3.0,
    set_point won => 5.0,
    deuce game => 1.5,
    _ => 1.0,
};
momentum[i] = momentum[i-1] + weight * direction;
```

**Visualization:**
```
+10 |        /\
    |       /  \        /\
  0 |------/----\------/--\-------
    |              \  /    \
-10 |               \/      \
    └──────────────────────────▶
     Set 1      Set 2      Set 3
```

- X-axis: point number or timestamp
- Y-axis: momentum value
- Set boundaries marked with vertical dashed lines
- Key points (break, set point) highlighted

**Display by platform:**

| Platform | Content |
|----------|---------|
| Watch App | Simple numeric stats after match (win %, break point rate) |
| Flutter App | Full momentum chart (fl_chart), detailed stats tables |
| API endpoints | replay_with_context, 時間統計, momentum JSON |

---

## Feature 3: Haptic Rhythm Training

**Priority: Low**

Metronome-style vibration on Apple Watch for serve practice rhythm.

### Concept
- User selects "Practice Mode"
- Sets interval (e.g., every 8 seconds)
- Watch vibrates at set intervals
- Advanced: multi-step rhythm (short buzz = ready, long buzz = toss, double buzz = hit)

### Technical approach
```swift
Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { _ in
    WKInterfaceDevice.current().play(.notification)
}
```

Different haptic types (`.click`, `.directionUp`, `.success`) for rhythm stages.

### Notes
- Low practical value in actual matches
- Implement only if time permits after core features

---

## Feature 4: Voice Scoring

### Decision
Button-triggered speech recognition (not continuous listening).

### Rationale
- Continuous listening drains Watch battery quickly
- Button trigger avoids false positives from court noise
- Simple UX: tap mic button, say one word, done

### Technical approach
- Use `Speech` framework with on-device recognition (watchOS 10+)
- No network required

### Voice commands
```
"我" / "my"       → score_point(Player1)
"對手" / "opponent" → score_point(Player2)
"取消" / "undo"     → undo()
```

### UX flow
1. User taps microphone button on Watch
2. Watch listens for ~2 seconds
3. Recognizes keyword, scores point
4. Haptic confirmation

---

## Feature 5: Offline Sync (SwiftData + HTTP)

### Decision
SwiftData for local storage + background HTTP upload to Rust API.

### Rationale
- CloudKit would add unnecessary middle layer since we have our own backend
- SwiftData is lightweight and sufficient as local cache
- Background upload handles intermittent connectivity

### Sync flow
```
Match ends → Save to SwiftData (local)
          → Attempt HTTP upload to Rust API
              ├─ Success → Mark as synced
              └─ Failure → Retry on next network availability
```

### Sync strategy decision
- **Watch 直接上傳 API**（方案 A） — 不透過 iPhone companion app 中繼
- watchOS 的 URLSession 在 iPhone 在身邊時自動透過 iPhone 網路出去，效能等同
- 不需要 WatchConnectivity 或額外的 iPhone companion app
- 離線優先：先存 SwiftData，再嘗試上傳

### Manual retry
- MatchHistoryView 加入手動「重新同步」按鈕
- 未同步比賽旁顯示 ⚠ 圖示 + 重試按鈕
- 列表頂部加「全部同步」按鈕（當有未同步項目時）

### Conflict handling
- Watch is the single source of truth for scoring (write-only to backend)
- No two-way sync needed — backend never modifies match data
- Simple idempotency: match has a local UUID, backend rejects duplicates

---

## Feature 6: Watch Widget (Complications)

### Content options
- Recent 5 match results (W L W W L)
- Overall win rate (73%, 22W/8L)
- Current streak (3 wins)

### Technical approach
- WidgetKit with `TimelineProvider`
- Data source: SwiftData local data (no network needed)
- Timeline refresh after each match sync

```swift
struct TennisWidgetProvider: TimelineProvider {
    func getTimeline(in context: Context, completion: @escaping (Timeline<Entry>) -> Void) {
        let recentMatches = fetchRecentMatches() // from SwiftData
        let winRate = calculateWinRate(recentMatches)
        // ...
    }
}
```

### Note on Flutter
iOS widgets must be written in Swift (not Flutter). A small Widget Extension sharing data via App Group with the main Flutter app.

---

## OpenSpec Changes Breakdown

Each change is an independent OpenSpec change that can be worked on through the `/opsx:` workflow.

| # | Change Name | Description | Depends On | Priority |
|---|-------------|-------------|------------|----------|
| 1 | `event-timestamps` | Add timestamps to HistoryEntry in Rust core engine | — | High |
| 2 | `match-replay-analysis` | PointContext replay API + statistics (break points, momentum) | #1 | High |
| 3 | `doubles-support` | Doubles rules: serve rotation, 4-player tracking | — | High |
| 4 | `cargo-workspace` | Restructure project into Cargo workspace | — | High |
| 5 | `api-backend` | Rust Axum API + PostgreSQL (sqlx) + JWT auth | #4 | High |
| 6 | `offline-sync` | SwiftData local storage + background HTTP upload | #5 | Medium |
| 7 | `voice-scoring` | Button-triggered Speech framework on watchOS | — | Medium |
| 8 | `watch-widget` | WidgetKit complication (win rate, recent results) | #6 | Medium |
| 9 | `flutter-app` | Flutter cross-platform app (charts, history, stats) | #5 | Medium |
| 10 | `cicd-pipelines` | GitHub Actions for Rust CI, watchOS build, API deploy | #4 | Medium |
| 11 | `haptic-training` | Apple Watch haptic rhythm for serve practice | — | Low |
| 12 | `manual-sync-button` | MatchHistoryView 手動重新同步按鈕（單筆 + 全部） | #6 | Medium |

### Dependency Graph

```
event-timestamps ──────▶ match-replay-analysis

cargo-workspace ──┬────▶ api-backend ──┬────▶ offline-sync ──┬──▶ watch-widget
                  │                    └────▶ flutter-app   └──▶ manual-sync-button
                  └────▶ cicd-pipelines

doubles-support          (independent)
voice-scoring            (independent)
haptic-training          (independent, low priority)
```

### Recommended Build Order

**Wave 1 — No dependencies (can be parallel):**
- `event-timestamps`
- `cargo-workspace`
- `doubles-support`

**Wave 2 — Depends on Wave 1:**
- `match-replay-analysis` (after event-timestamps)
- `api-backend` (after cargo-workspace)
- `cicd-pipelines` (after cargo-workspace)

**Wave 3 — Depends on API backend:**
- `offline-sync`
- `flutter-app`
- `voice-scoring` (independent, can start anytime)

**Wave 4 — Final layer:**
- `watch-widget` (after offline-sync)
- `haptic-training`
