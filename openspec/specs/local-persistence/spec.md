## ADDED Requirements

### Requirement: SwiftData models store completed matches
The app SHALL define `MatchRecord` and `MatchEventRecord` SwiftData models to persist completed match data locally on the Watch.

#### Scenario: Match saved on completion
- **WHEN** a match ends (winner is determined)
- **THEN** the app SHALL create a `MatchRecord` with config, final score, timestamps, and all point events, with `isSynced` set to `false`

#### Scenario: Match data includes point events
- **WHEN** a match is saved
- **THEN** the `MatchRecord` SHALL include all `MatchEventRecord` entries with point_number, player, and timestamp from the FFI point events

### Requirement: Each match has a stable local UUID
Each `MatchRecord` SHALL have a UUID generated at creation that serves as the idempotency key (`client_id`) for backend sync.

#### Scenario: UUID persists across sync attempts
- **WHEN** a match is saved locally and later synced
- **THEN** the same UUID SHALL be sent as `client_id` in every upload attempt

### Requirement: ModelContainer configured at app level
The app entry point SHALL configure a SwiftData `ModelContainer` for `MatchRecord` and `MatchEventRecord`.

#### Scenario: App launches with SwiftData
- **WHEN** the app starts
- **THEN** the `ModelContainer` SHALL be available to all views via the environment
