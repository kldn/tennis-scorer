## MODIFIED Requirements

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
