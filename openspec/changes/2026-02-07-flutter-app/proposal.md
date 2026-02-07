## Why

The Apple Watch screen is too small for detailed statistics, charts, and match history browsing. A cross-platform Flutter app provides a rich UI for viewing match data, momentum charts, and performance trends on iOS, Android, and Web.

## What Changes

- Create a new Flutter project for iOS/Android/Web
- Implement user authentication (login/register) against the Rust API
- Match history list with search and filtering
- Match detail view with full point-by-point replay
- Momentum chart visualization (using fl_chart)
- Statistics dashboard: win rate, break point stats, trends
- Head-to-head comparison view

## Capabilities

### New Capabilities
- `flutter-auth`: Authentication flow (login/register) using JWT from Rust API
- `flutter-match-history`: Paginated match list with filtering
- `flutter-match-detail`: Point-by-point match detail with contextual stats
- `flutter-momentum-chart`: Interactive momentum chart (fl_chart)
- `flutter-stats-dashboard`: Statistics overview and trends

### Modified Capabilities
- None (pure client, consumes existing API endpoints)

## Impact

- **New project**: `flutter/` directory at repository root
- **Dependencies**: Flutter SDK, fl_chart, http, flutter_secure_storage
- **API**: Consumes existing endpoints (no backend changes needed)
- **Depends on**: `api-backend` (#5, completed)
