## Why

The tennis-scorer project has grown into a multi-crate Cargo workspace with a Rust core, a C FFI bridge, and an Apple Watch frontend. There is currently no automated CI/CD pipeline. All quality checks (formatting, linting, tests, watchOS cross-compilation) are performed manually by developers, which is error-prone and slows down the feedback loop. Adding GitHub Actions CI/CD pipelines will catch regressions early, verify that the FFI header stays in sync with the Rust source, and lay the groundwork for future API deployment.

## What Changes

- Add a **Rust CI workflow** that runs on every push and pull request: checks formatting with `cargo fmt`, lints with `cargo clippy`, runs tests with `cargo test`, and builds a release binary.
- Add a **watchOS build verification workflow** that runs on version tags and manual dispatch: cross-compiles the Rust library for watchOS targets using the nightly toolchain, and verifies that the cbindgen-generated FFI header is up-to-date.
- Add an **API deploy workflow** that runs on push to main: runs tests for the API crate and builds a Docker image as a placeholder for future deployment.

## Capabilities

### New Capabilities

- `github-actions-ci`: Automated CI/CD pipelines using GitHub Actions for Rust quality checks, watchOS build verification, and API deployment preparation.

### Modified Capabilities

None.

## Impact

- **Repository root**: New `.github/workflows/` directory with three workflow YAML files.
- **No code changes**: These workflows only add CI/CD configuration; they do not modify existing Rust, Swift, or build scripts.
- **Developer workflow**: Pull requests will require passing CI checks before merge. watchOS builds can be verified automatically on tagged releases.
