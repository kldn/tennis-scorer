## ADDED Requirements

### Requirement: Rust CI workflow triggers

The Rust CI workflow SHALL run on every push and every pull request to any branch.

#### Scenario: Push triggers CI
- **WHEN** a developer pushes commits to any branch
- **THEN** the Rust CI workflow SHALL be triggered

#### Scenario: Pull request triggers CI
- **WHEN** a pull request is opened, synchronized, or reopened
- **THEN** the Rust CI workflow SHALL be triggered

### Requirement: Rust CI workflow steps

The Rust CI workflow SHALL execute formatting, linting, testing, and release build steps in sequence.

#### Scenario: Formatting check
- **WHEN** the Rust CI workflow runs
- **THEN** it SHALL execute `cargo fmt --check` across the entire workspace
- **AND** the workflow SHALL fail if any file is not properly formatted

#### Scenario: Linting check
- **WHEN** the Rust CI workflow runs
- **THEN** it SHALL execute `cargo clippy -- -D warnings` across the entire workspace
- **AND** the workflow SHALL fail if any Clippy warning is present

#### Scenario: Test execution
- **WHEN** the Rust CI workflow runs
- **THEN** it SHALL execute `cargo test` across the entire workspace
- **AND** the workflow SHALL fail if any test fails

#### Scenario: Release build
- **WHEN** the Rust CI workflow runs
- **THEN** it SHALL execute `cargo build --release` across the entire workspace
- **AND** the workflow SHALL fail if the release build fails

### Requirement: watchOS build verification triggers

The watchOS build workflow SHALL run on version tags and manual dispatch.

#### Scenario: Tag triggers watchOS build
- **WHEN** a tag matching the pattern `v*` is pushed
- **THEN** the watchOS build workflow SHALL be triggered

#### Scenario: Manual trigger for watchOS build
- **WHEN** a developer manually triggers the workflow via `workflow_dispatch`
- **THEN** the watchOS build workflow SHALL be triggered

### Requirement: watchOS build verification steps

The watchOS build workflow SHALL cross-compile the Rust library for watchOS targets using the nightly toolchain.

#### Scenario: watchOS cross-compilation
- **WHEN** the watchOS build workflow runs
- **THEN** it SHALL execute `build-watchos.sh`
- **AND** the workflow MUST use `cargo +nightly` with the `rust-src` component installed
- **AND** the workflow MUST run on a macOS runner

#### Scenario: Static library output verification
- **WHEN** `build-watchos.sh` completes successfully
- **THEN** the files `target/aarch64-apple-watchos-sim/release/libtennis_scorer.a` and `target/aarch64-apple-watchos/release/libtennis_scorer.a` SHALL exist

### Requirement: FFI header freshness check

The watchOS build workflow SHALL verify that the committed FFI header matches what cbindgen would generate.

#### Scenario: FFI header is up-to-date
- **WHEN** the watchOS build workflow runs cbindgen to regenerate the header
- **AND** the generated header matches the committed header
- **THEN** the FFI header check SHALL pass

#### Scenario: FFI header is stale
- **WHEN** the watchOS build workflow runs cbindgen to regenerate the header
- **AND** the generated header differs from the committed header
- **THEN** the FFI header check SHALL fail
- **AND** the workflow SHALL output the diff showing what changed

### Requirement: API deploy workflow triggers

The API deploy workflow SHALL run on pushes to the main branch only.

#### Scenario: Push to main triggers deploy
- **WHEN** a developer pushes commits to the `main` branch
- **THEN** the API deploy workflow SHALL be triggered

#### Scenario: Push to non-main branch does not trigger deploy
- **WHEN** a developer pushes commits to a branch other than `main`
- **THEN** the API deploy workflow SHALL NOT be triggered

### Requirement: API deploy workflow steps

The API deploy workflow SHALL run tests and build a Docker image as a placeholder for future deployment.

#### Scenario: API test execution
- **WHEN** the API deploy workflow runs
- **THEN** it SHALL execute `cargo test -p tennis-scorer-api`
- **AND** the workflow SHALL fail if any test fails

#### Scenario: Docker image build
- **WHEN** the API deploy workflow runs and tests pass
- **THEN** it SHALL build a Docker image for the API crate
- **AND** the workflow SHALL NOT push the image to any registry (placeholder only)

### Requirement: Cargo caching

All workflows SHALL use caching for Cargo artifacts to speed up builds.

#### Scenario: Cache hit on unchanged dependencies
- **WHEN** the Cargo.lock file has not changed since the last workflow run
- **THEN** the workflow SHALL restore cached Cargo registry and target directory
- **AND** the build time SHALL be reduced compared to a clean build

#### Scenario: Cache miss on changed dependencies
- **WHEN** the Cargo.lock file has changed
- **THEN** the workflow SHALL perform a fresh dependency download
- **AND** the workflow SHALL save the new artifacts to the cache
