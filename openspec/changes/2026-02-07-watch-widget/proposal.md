## Why

Users want to glance at their tennis stats (win rate, recent results, current streak) directly from the Watch face without opening the app.

## What Changes

- Create a WidgetKit extension for the Apple Watch
- Implement `TimelineProvider` that reads from SwiftData local data
- Display content options: recent 5 match results (W/L), overall win rate, current streak
- Refresh timeline after each match sync
- Support multiple complication families (circular, rectangular, inline)

## Capabilities

### New Capabilities
- `watch-widget`: WidgetKit complication showing tennis statistics on the Watch face

### Modified Capabilities
- None (reads from existing SwiftData models)

## Impact

- **Watch App**: New Widget Extension target in Xcode project
- **Dependencies**: WidgetKit framework, App Group for data sharing
- **Data source**: SwiftData local data (no network needed)
- **Depends on**: `offline-sync` (#6, completed)
