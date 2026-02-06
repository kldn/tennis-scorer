## ADDED Requirements

### Requirement: Core engine records timestamp on each scored point

The scoring engine SHALL record the current system time (`std::time::SystemTime`) whenever a point is scored via `score_point()`. The timestamp SHALL be stored alongside the corresponding history entry.

#### Scenario: Timestamp recorded on point scored
- **WHEN** `score_point()` is called with a valid player
- **THEN** the new history entry SHALL include a timestamp of the current system time
- **AND** the timestamp SHALL have at least millisecond precision

#### Scenario: No timestamp on failed score attempt
- **WHEN** `score_point()` is called on a completed match
- **THEN** no new history entry SHALL be created
- **AND** no timestamp SHALL be recorded

### Requirement: Undo preserves timestamp consistency

When a point is undone, the corresponding timestamp SHALL be removed from history. Re-scoring after undo SHALL record a new, current timestamp.

#### Scenario: Undo removes timestamp
- **WHEN** a point is scored at time T1
- **AND** the point is undone
- **THEN** the history entry with timestamp T1 SHALL be removed

#### Scenario: Re-score after undo gets new timestamp
- **WHEN** a point is scored at time T1
- **AND** the point is undone
- **AND** a new point is scored at time T2 (where T2 > T1)
- **THEN** the new history entry SHALL have timestamp T2, not T1

### Requirement: Point event list retrievable via FFI

The C FFI layer SHALL expose a function to retrieve the complete list of scored points with their timestamps and player information.

#### Scenario: Retrieve point events from FFI
- **WHEN** a match has N points scored
- **AND** the caller requests the point event list via FFI
- **THEN** the system SHALL return an array of N point events
- **AND** each event SHALL contain the player identifier (1 or 2) and a timestamp (as Unix epoch seconds with millisecond precision)

#### Scenario: Empty match returns empty point list
- **WHEN** no points have been scored
- **AND** the caller requests the point event list via FFI
- **THEN** the system SHALL return an empty list (count = 0)

#### Scenario: Point list reflects undo operations
- **WHEN** 3 points are scored and then 1 is undone
- **AND** the caller requests the point event list via FFI
- **THEN** the system SHALL return 2 point events

### Requirement: Timestamps do not affect scoring logic

Timestamps SHALL be purely observational metadata. They SHALL NOT influence match scoring rules, state transitions, or any game/set/match outcome.

#### Scenario: Scoring outcomes unchanged
- **WHEN** two matches are played with identical point sequences but different timestamps
- **THEN** both matches SHALL produce identical final scores and state transitions
