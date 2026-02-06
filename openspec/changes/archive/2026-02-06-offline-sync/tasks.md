## 1. SwiftData Models

- [x] 1.1 Create `Models/MatchRecord.swift` — @Model with id (UUID), matchType, config (Data), winner, sets, timestamps, isSynced, events relationship
- [x] 1.2 Create `Models/MatchEventRecord.swift` — @Model with pointNumber, player, timestamp, inverse relationship to MatchRecord
- [x] 1.3 Update `TennisScorerApp.swift` — add modelContainer for MatchRecord and MatchEventRecord

## 2. Match Saving

- [x] 2.1 Add `saveMatch()` method to TennisMatch or ContentView that extracts FFI data (score, config, point events) into a MatchRecord
- [x] 2.2 Wire match completion in ContentView to call saveMatch() when winner is determined
- [x] 2.3 Track match start time (set when match begins, used for startedAt)

## 3. Keychain & Auth

- [x] 3.1 Create `Services/KeychainHelper.swift` — save/read/delete tokens from Keychain
- [x] 3.2 Create `Services/APIClient.swift` — URLSession wrapper with base URL, auth headers, token refresh on 401
- [x] 3.3 Create `Views/AuthView.swift` — login/register form with email and password fields, error display
- [x] 3.4 Add auth state management — ObservableObject tracking isLoggedIn, wire into app navigation

## 4. Sync Service

- [x] 4.1 Create `Services/SyncService.swift` — query unsynced matches, upload via APIClient, mark synced on success
- [x] 4.2 Wire sync on match completion — call SyncService after saving to SwiftData
- [x] 4.3 Wire sync on app launch — trigger SyncService.syncAll() when app becomes active and user is authenticated

## 5. Match History View

- [x] 5.1 Create `Views/MatchHistoryView.swift` — @Query for MatchRecord sorted by date, show result/score/sync status
- [x] 5.2 Add navigation to MatchHistoryView from main ContentView (e.g., toolbar button or list item)

## 6. Integration

- [x] 6.1 Add API base URL to configuration (hardcoded for now, or from environment)
- [x] 6.2 Test full flow: complete match → save → sync → view in history
