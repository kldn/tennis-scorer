## ADDED Requirements

### Requirement: Cloud sync for completed matches
The Wear OS app SHALL sync completed matches to the cloud when network is available.

#### Scenario: Online after match completion
- **WHEN** a match is completed and the watch has network connectivity
- **THEN** the app SHALL call `POST /api/matches` with the match data (config, score, events, opponent info) and mark the local record as synced

#### Scenario: Offline match completion
- **WHEN** a match is completed but the watch has no network connectivity
- **THEN** the app SHALL store the match locally and queue it for sync when connectivity is restored

#### Scenario: Sync on connectivity restore
- **WHEN** network connectivity is restored and there are unsynced matches in the local queue
- **THEN** the app SHALL automatically sync all pending matches to the cloud in chronological order

#### Scenario: Idempotent sync
- **WHEN** a match sync is retried (e.g., after a network failure mid-sync)
- **THEN** the app SHALL use the client_id field to ensure the match is not duplicated on the server

### Requirement: Sync status indicator
The Wear OS app SHALL show sync status to the user.

#### Scenario: All synced
- **WHEN** all local matches have been synced to the cloud
- **THEN** the app SHALL display a "synced" indicator

#### Scenario: Pending sync
- **WHEN** there are unsynced matches in the local queue
- **THEN** the app SHALL display the number of matches pending sync

#### Scenario: Sync in progress
- **WHEN** a sync operation is actively running
- **THEN** the app SHALL display a syncing indicator

### Requirement: Wear OS Firebase Auth
The Wear OS app SHALL authenticate users via Firebase Auth.

#### Scenario: Sign in on Wear OS
- **WHEN** the user opens the Wear OS app for the first time
- **THEN** the app SHALL present a Firebase Auth sign-in flow (Google Sign-In or phone-based auth)

#### Scenario: Auth token in API calls
- **WHEN** the app makes API calls for sync
- **THEN** the app SHALL include the Firebase ID Token in the Authorization header
