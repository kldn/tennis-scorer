## MODIFIED Requirements

### Requirement: TennisMatch object provides match lifecycle
The epoch_secs_to_system_time helper function SHALL handle negative and non-finite input values defensively.
For negative values, the function SHALL subtract the absolute duration from UNIX_EPOCH.
For NaN or infinite values, the function SHALL return UNIX_EPOCH as a safe default.

#### Scenario: Negative timestamp handling
- **WHEN** epoch_secs_to_system_time receives a negative value (e.g., -100.0)
- **THEN** it returns a SystemTime before UNIX_EPOCH without panicking

#### Scenario: NaN timestamp handling
- **WHEN** epoch_secs_to_system_time receives NaN
- **THEN** it returns UNIX_EPOCH as a safe default without panicking

#### Scenario: Positive timestamp handling
- **WHEN** epoch_secs_to_system_time receives a positive value (e.g., 1700000000.0)
- **THEN** it returns the correct SystemTime corresponding to that epoch
