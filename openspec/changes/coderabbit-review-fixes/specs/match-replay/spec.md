## MODIFIED Requirements

### Requirement: Serving player detection for singles
The system SHALL alternate serving player each game.
The system SHALL use `% 2 == 0` (stable Rust) instead of `is_multiple_of(2)` (nightly-only) for determining serving player based on game count.

#### Scenario: Serving player uses stable Rust API
- **WHEN** the replay engine calculates the serving player
- **THEN** it uses modulo operator (`% 2 == 0`) which compiles on stable Rust
- **AND** does not require nightly features

### Requirement: PointContext struct captures full scoring context
The system SHALL define PointContext with point_number, scorer, timestamp, serving_player, score_before, and point type flags.
Documentation comments SHALL not be duplicated.

#### Scenario: No duplicate doc comments
- **WHEN** viewing the source code of replay.rs
- **THEN** each function has exactly one doc comment block with no duplicate lines
