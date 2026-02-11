## MODIFIED Requirements

### Requirement: SwiftData models store completed matches
The system SHALL persist completed matches using SwiftData.
When saving to modelContext, the system SHALL use do-catch error handling instead of `try?`.
Save failures SHALL be logged for debugging purposes.

#### Scenario: Save failure is logged
- **WHEN** modelContext.save() fails after inserting a match record
- **THEN** the error is caught and logged (not silently discarded)
- **AND** the app does not crash

#### Scenario: Successful save
- **WHEN** modelContext.save() succeeds
- **THEN** the match record is persisted normally
