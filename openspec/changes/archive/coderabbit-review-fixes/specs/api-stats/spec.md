## MODIFIED Requirements

### Requirement: Match analysis endpoint
The system SHALL provide GET /api/stats/match/:id/analysis with full MatchAnalysis.
Timestamp conversion from chrono DateTime to SystemTime SHALL use the idiomatic `.into()` conversion instead of manual arithmetic with `timestamp()` and `timestamp_subsec_nanos()`.

#### Scenario: Timestamp conversion uses idiomatic chrono API
- **WHEN** the stats handler converts a chrono DateTime<Utc> to SystemTime
- **THEN** it uses `ts.into()` (the From implementation) instead of manual UNIX_EPOCH arithmetic
