## Context

The Watch app currently manages match state entirely in-memory via FFI to the Rust scoring engine. When the app exits, all data is lost. The backend API (`POST /api/matches`) is ready to receive match data. We need a bridge: save locally, then sync to the server.

The Watch is the single source of truth — sync is one-way (write-only to backend). No two-way sync or conflict resolution needed.

## Goals / Non-Goals

**Goals:**
- Persist completed matches locally using SwiftData
- Upload matches to the backend API with automatic retry
- Show match history on the Watch
- Authenticate with the backend (login/register)
- Work fully offline — sync when connectivity is available

**Non-Goals:**
- Two-way sync (backend never modifies match data)
- Syncing in-progress matches (only completed matches)
- Push notifications from backend
- Background app refresh / URLSession background tasks (watchOS limitations)
- Match editing after completion

## Decisions

### 1. Storage: SwiftData
**Choice**: SwiftData with `@Model` classes
**Why**: Native watchOS support (watchOS 10+), lightweight, integrates with SwiftUI via `@Query`. Preferred over Core Data (older API, more boilerplate) or UserDefaults (not suitable for structured data).

### 2. Sync trigger: On match completion + app launch
**Choice**: Attempt sync immediately when match ends; retry all unsynced on app launch
**Why**: watchOS has limited background execution. Foreground sync on natural trigger points is reliable. No need for `BGTaskScheduler` which is unreliable on watchOS.

### 3. Auth token storage: Keychain
**Choice**: Store JWT tokens in Keychain via Security framework
**Why**: Persists across app launches, encrypted at rest. UserDefaults is not secure for tokens.

### 4. Idempotency: client_id = local UUID
**Choice**: Each `MatchRecord` gets a UUID at creation, sent as `client_id` to the API
**Why**: Backend returns 200 for duplicates — safe to retry without creating duplicate records.

### 5. Match data capture: Extract from FFI at completion
**Choice**: When match ends, read all data from FFI (score, config, point events) and store in SwiftData
**Why**: The FFI handle is the source of truth during the match. At completion, we snapshot everything into SwiftData before the handle is freed.

## Data Model

```swift
@Model
class MatchRecord {
    var id: UUID                    // local ID, sent as client_id
    var matchType: String           // "singles" or "doubles"
    var config: Data                // JSON-encoded MatchConfig
    var winner: Int                 // 1 or 2
    var player1Sets: Int
    var player2Sets: Int
    var startedAt: Date
    var endedAt: Date
    var isSynced: Bool              // false until backend confirms
    var createdAt: Date
    var events: [MatchEventRecord]  // point-by-point data
}

@Model
class MatchEventRecord {
    var pointNumber: Int
    var player: Int                 // 1 or 2
    var timestamp: Date
    var matchRecord: MatchRecord?   // inverse relationship
}
```

## Sync Flow

```
Match ends
  → Extract data from FFI (score, events, config)
  → Save MatchRecord to SwiftData (isSynced = false)
  → Attempt HTTP POST /api/matches
      ├─ 201/200 → Mark isSynced = true
      └─ Failure → Leave isSynced = false

App launches
  → Query MatchRecord WHERE isSynced = false
  → For each, attempt upload
  → Mark synced on success
```

## Auth Flow

```
Settings screen
  → User enters email/password
  → POST /api/auth/register or /api/auth/login
  → Store access_token + refresh_token in Keychain
  → Sync service uses stored token for uploads

Token refresh
  → On 401 response, attempt POST /api/auth/refresh
  → If refresh fails, clear tokens and prompt re-login
```

## Project Structure (new files)

```
WatchApp/TennisScorer Watch App/
├── Models/
│   ├── MatchRecord.swift          — SwiftData model
│   └── MatchEventRecord.swift     — SwiftData model
├── Services/
│   ├── SyncService.swift          — HTTP upload + retry logic
│   ├── APIClient.swift            — URLSession wrapper, auth headers
│   └── KeychainHelper.swift       — Token storage
├── Views/
│   ├── MatchHistoryView.swift     — List of past matches
│   └── AuthView.swift             — Login/register form
└── TennisScorerApp.swift          — Add modelContainer
```

## Risks / Trade-offs

- **[Risk] watchOS memory constraints** → SwiftData is lightweight; only store completed matches (not in-progress state)
- **[Risk] No background sync** → Acceptable; sync on match completion and app launch covers the main use cases
- **[Risk] Token expiry while offline** → Refresh token has 30d expiry; if expired, user re-authenticates on next launch
- **[Risk] Large match history** → Pagination in history view; SwiftData handles storage efficiently
