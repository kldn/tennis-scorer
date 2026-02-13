## ADDED Requirements

### Requirement: Wear OS match scoring with Rust engine
The Wear OS app SHALL use the shared Rust scoring engine via flutter_rust_bridge for tennis match scoring.

#### Scenario: Start new match
- **WHEN** the user starts a new match with selected configuration (sets, games, tiebreak rules)
- **THEN** the app SHALL initialize the Rust scoring engine with the match config and display the initial score (0-0)

#### Scenario: Record a point
- **WHEN** the user taps "Player 1" or "Player 2" point button
- **THEN** the app SHALL call the Rust engine to record the point and update the displayed score

#### Scenario: Undo last point
- **WHEN** the user taps the undo button
- **THEN** the app SHALL call the Rust engine to undo the last point and revert the displayed score

#### Scenario: Match completion
- **WHEN** the Rust engine determines the match is complete
- **THEN** the app SHALL display the final score and offer options to save or discard

### Requirement: Offline-first scoring
The Wear OS app SHALL function fully offline for match scoring.

#### Scenario: No network connection
- **WHEN** the watch has no network connection during a match
- **THEN** the app SHALL continue scoring normally using the local Rust engine

#### Scenario: Local persistence
- **WHEN** a match is completed or in progress
- **THEN** the app SHALL persist match data (config, events, score) locally using Hive

### Requirement: Rust engine integration via FFI
The Wear OS app SHALL integrate the tennis-scorer Rust crate via dart:ffi using flutter_rust_bridge.

#### Scenario: Engine initialization
- **WHEN** the app starts
- **THEN** the app SHALL load the compiled Rust shared library (.so) and initialize the flutter_rust_bridge bindings

#### Scenario: Cross-compilation
- **WHEN** the app is built
- **THEN** the build system SHALL use cargo-ndk to cross-compile the Rust crate for Android ARM architectures (arm64-v8a, armeabi-v7a)

### Requirement: Voice input for scoring
The Wear OS app SHALL support voice input for recording points.

#### Scenario: Voice command recognized
- **WHEN** the user activates voice input and says a recognized command (e.g., "ace", "fault", "point player one")
- **THEN** the app SHALL record the corresponding point via the Rust engine

#### Scenario: Unrecognized voice input
- **WHEN** the user's voice input is not recognized
- **THEN** the app SHALL display an error message and not record any point
