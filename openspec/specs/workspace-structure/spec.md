## ADDED Requirements

### Requirement: Project uses Cargo workspace with multiple crates

The project root SHALL be a Cargo workspace containing at least two member crates: `tennis-scorer` (core library) and `tennis-scorer-api` (backend API server).

#### Scenario: Workspace builds successfully
- **WHEN** `cargo build` is run from the workspace root
- **THEN** all workspace members SHALL compile without errors

#### Scenario: Core crate tests pass
- **WHEN** `cargo test -p tennis-scorer` is run
- **THEN** all existing tests SHALL pass with no modifications to test logic

#### Scenario: API crate builds as binary
- **WHEN** `cargo build -p tennis-scorer-api` is run
- **THEN** the output SHALL include an executable binary (not just a library)

### Requirement: Core crate is reusable as a dependency

The `tennis-scorer` crate SHALL be usable as a path dependency by sibling crates in the workspace.

#### Scenario: API crate depends on core crate
- **WHEN** `tennis-scorer-api` declares `tennis-scorer = { path = "../tennis-scorer" }` in its dependencies
- **THEN** it SHALL be able to import and use types from `tennis-scorer` (e.g., `MatchConfig`, `Player`, `MatchWithHistory`)

### Requirement: watchOS build continues to work

The `build-watchos.sh` script SHALL produce static libraries for watchOS targets after the workspace conversion.

#### Scenario: watchOS simulator build
- **WHEN** `build-watchos.sh` is executed
- **THEN** `target/aarch64-apple-watchos-sim/release/libtennis_scorer.a` SHALL be produced

#### Scenario: watchOS device build
- **WHEN** `build-watchos.sh` is executed
- **THEN** `target/aarch64-apple-watchos/release/libtennis_scorer.a` SHALL be produced

### Requirement: C FFI header generation works

`cbindgen` SHALL successfully generate the C header from the workspace structure.

#### Scenario: Header regeneration
- **WHEN** cbindgen is run with the updated configuration
- **THEN** `include/tennis_scorer.h` SHALL be generated with the same content as before the workspace conversion
