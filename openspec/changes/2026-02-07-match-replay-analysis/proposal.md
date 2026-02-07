## Why

After a match, players want to understand their performance patterns — break point conversion, momentum shifts, scoring pace. The raw point-by-point data is already stored (via event-timestamps), but there's no way to compute contextual statistics from it.

## What Changes

- Implement `replay_with_context()` in the core `tennis-scorer` crate that replays a point sequence through the scoring engine, annotating each point with context (break point, game point, set point, match point, serving player)
- Define `PointContext` struct with full scoring context per point
- Compute key statistics: break points created/converted/faced/saved, deuce analysis, game/set/match point conversion rates
- Implement momentum calculation (basic + weighted formula)
- Add API endpoints in `tennis-scorer-api`:
  - `GET /api/stats/match/:id/analysis` — break point, deuce, conversion stats
  - `GET /api/stats/match/:id/momentum` — momentum chart data (JSON array)
  - `GET /api/stats/match/:id/pace` — per-point interval, per-game/set duration

## Capabilities

### New Capabilities
- `match-replay`: Core engine replay function that annotates points with scoring context
- `match-statistics`: Statistical calculations (break points, deuce, conversion rates)
- `momentum-chart`: Momentum value computation with basic and weighted formulas
- `stats-api`: API endpoints serving analysis, momentum, and pace data

### Modified Capabilities
- `tennis-scorer` crate: New public API `replay_with_context()`

## Impact

- **Core crate**: New `PointContext` struct, `replay_with_context()` function, statistics module
- **API crate**: 3 new endpoints under `/api/stats/match/:id/`
- **Dependencies**: None new — uses existing types from core crate
- **Depends on**: `event-timestamps` (#1, completed)
