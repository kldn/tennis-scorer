## Why

Currently, unsynced matches only retry upload on app launch (via `SyncService.syncAll()`). Users may want to manually trigger a retry — especially if they see the ⚠ indicator next to an unsynced match and want immediate feedback.

## What Changes

- Add a "Sync All" button at the top of MatchHistoryView (visible only when unsynced matches exist)
- Add a per-match retry button next to unsynced matches (⚠ indicator becomes tappable)
- Show sync progress indicator while syncing
- Haptic feedback on sync success/failure
- Update `isSynced` status in real-time after successful sync

## Capabilities

### New Capabilities
- `manual-sync`: Manual sync trigger UI in MatchHistoryView (single match + batch)

### Modified Capabilities
- `MatchHistoryView`: Add sync buttons and progress indicators
- `SyncService`: May need a `syncMatch(_:)` public API (already exists)

## Impact

- **Watch App**: UI changes in MatchHistoryView only
- **Dependencies**: None new — uses existing SyncService and APIClient
- **Depends on**: `offline-sync` (#6, completed)
