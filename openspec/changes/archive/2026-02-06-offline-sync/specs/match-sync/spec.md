## ADDED Requirements

### Requirement: Sync service uploads unsynced matches
The app SHALL provide a `SyncService` that uploads matches where `isSynced == false` to `POST /api/matches`.

#### Scenario: Successful upload
- **WHEN** a match is uploaded and the backend returns 201 or 200
- **THEN** the app SHALL set `isSynced = true` on the `MatchRecord`

#### Scenario: Upload failure due to network
- **WHEN** the upload fails due to network error
- **THEN** the `MatchRecord` SHALL remain with `isSynced = false` for retry later

#### Scenario: Upload failure due to expired token
- **WHEN** the backend returns 401
- **THEN** the sync service SHALL attempt a token refresh; if refresh fails, stop syncing and leave matches unsynced

### Requirement: Sync triggers on match completion and app launch
The sync service SHALL attempt to upload unsynced matches at two trigger points.

#### Scenario: Sync after match ends
- **WHEN** a match is completed and saved to SwiftData
- **THEN** the sync service SHALL immediately attempt to upload it

#### Scenario: Sync on app launch
- **WHEN** the app launches and the user is authenticated
- **THEN** the sync service SHALL query all unsynced matches and attempt to upload each one

### Requirement: Idempotent uploads via client_id
The sync service SHALL include the local UUID as `client_id` in the upload payload.

#### Scenario: Duplicate upload returns existing match
- **WHEN** the same match is uploaded twice (same `client_id`)
- **THEN** the backend SHALL return 200 with the existing match ID, and the app SHALL mark it as synced
