## Why

Completed matches on the Apple Watch are lost when the app exits — there is no persistence layer. To enable match history, statistics, and future cross-device access, we need to save match results locally and upload them to the backend API.

## What Changes

- Add SwiftData models to persist completed matches on the Watch
- Save match data (config, final score, point events) to SwiftData when a match ends
- Add an HTTP sync service that uploads unsynced matches to `POST /api/matches`
- Retry failed uploads on next app launch or network availability
- Add a match history list view showing past matches from SwiftData
- Add user authentication flow (login/register) in the Watch app settings
- Store auth tokens securely in Keychain

## Capabilities

### New Capabilities
- `local-persistence`: SwiftData models and storage for completed matches on watchOS
- `match-sync`: HTTP upload service with retry logic for syncing matches to the backend API
- `watch-auth`: User authentication UI and token management on the Watch app
- `match-history`: Match history list view displaying past results from local storage

### Modified Capabilities

## Impact

- **Watch App**: New SwiftData models, sync service, auth flow, history view
- **Dependencies**: SwiftData framework, URLSession, Security framework (Keychain)
- **Backend**: No changes — uses existing `POST /api/matches` and auth endpoints
- **Info.plist**: No new permissions needed (network access is default on watchOS)
