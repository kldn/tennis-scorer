## Why

After a match, players want to understand their performance patterns — break point conversion, momentum shifts, scoring pace. The raw point-by-point data is already stored (via event-timestamps), but there's no way to compute contextual statistics from it.

## What Changes

- Implement `replay_with_context()` in the core `tennis-scorer` crate that replays a point sequence through the scoring engine, annotating each point with context (break point, game point, set point, match point, serving player)
- Define `PointContext` struct with full scoring context per point
- Compute comprehensive statistics from `Vec<PointContext>` (details below)
- Implement momentum calculation (basic + weighted formula)
- Add API endpoints in `tennis-scorer-api`
- Expose core statistics via UniFFI for watchOS app

## Data Constraints

All statistics are computed from **point-winner + timestamp** data only (no serve speed, shot type, or tracking hardware). This covers ATP/WTA "standard box score" stats minus serve/return split (which requires 1st/2nd serve tagging).

Future extension: `PointContext` will include `point_end_type: Option<PointEndType>` (reserved, not implemented) to support Ace/DF/Winner/UE tagging when UI is ready. Use `#[serde(default)]` for backward compatibility.

## Statistics Catalog

### Break Points
- Break points created / converted (per player)
- Break points faced / saved (per player)
- Break point conversion rate

### Service & Return
- Service games held / lost (hold %)
- Return games won / lost (break %)
- Dominance Ratio: return points won % / serve points lost %

### Deuce Analysis
- Average deuce count per deuce game
- Deuce game win rate (per player)

### Conversion Rates
- Game point conversion rate
- Set point conversion rate
- Match point conversion rate

### Momentum
- Basic: `momentum[i] = momentum[i-1] + (P1 scored ? +1 : -1)`
- Weighted: break point conversion ×3, set point ×5, deuce game win ×1.5
- Per-set momentum curves (independent per set)

### Streaks
- Longest consecutive points won
- Longest consecutive points lost
- Longest consecutive service games held
- Max games won in a row

### Clutch Performance
- Win rate on break points vs normal points
- Win rate on set points vs normal points
- Win rate on match points vs normal points
- Overall "clutch score": weighted aggregate of critical-point performance

### Pace & Timing
- Average interval between points
- Per-game duration
- Per-set duration
- Total match duration

### Tiebreak Performance
- Tiebreaks won / played
- Average point margin in tiebreaks

### Total Points
- Points won per player (count + percentage)

## API Endpoints

| Endpoint | Response |
|----------|----------|
| `GET /api/stats/match/:id/analysis` | Break points, deuce, conversion rates, clutch, dominance, streaks, tiebreak, total points |
| `GET /api/stats/match/:id/momentum` | Momentum array (basic + weighted) + per-set curves |
| `GET /api/stats/match/:id/pace` | Per-point intervals, per-game/set durations |

## Capabilities

### New Capabilities
- `match-replay`: Core engine replay function that annotates points with scoring context (`PointContext`)
- `match-statistics`: Statistical calculations (break points, deuce, conversion, clutch, dominance, streaks, tiebreak)
- `momentum-chart`: Momentum value computation with basic and weighted formulas, per-set curves
- `stats-api`: API endpoints serving analysis, momentum, and pace data
- `stats-uniffi`: UniFFI bindings exposing core statistics to watchOS app

### Modified Capabilities
- `tennis-scorer` crate: New public API `replay_with_context()`, statistics module

## Impact

- **Core crate**: New `PointContext` struct, `replay_with_context()` function, statistics module, UniFFI exports
- **API crate**: 3 new endpoints under `/api/stats/match/:id/`
- **watchOS app**: Match summary stats view (post-match)
- **Dependencies**: None new — uses existing types from core crate
- **Depends on**: `event-timestamps` (#1, completed), `uniffi-migration` (#6, completed)
