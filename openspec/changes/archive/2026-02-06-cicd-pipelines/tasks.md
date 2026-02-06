## 1. Rust CI Workflow

- [x] 1.1 Create `.github/workflows/rust.yml` with push and pull_request triggers
- [x] 1.2 Add step to install stable Rust toolchain with `rustfmt` and `clippy` components
- [x] 1.3 Add Cargo caching using `Swatinem/rust-cache` or `actions/cache`
- [x] 1.4 Add step for `cargo fmt --check`
- [x] 1.5 Add step for `cargo clippy -- -D warnings`
- [x] 1.6 Add step for `cargo test`
- [x] 1.7 Add step for `cargo build --release`

## 2. watchOS Build Workflow

- [x] 2.1 Create `.github/workflows/watchos.yml` with tag (`v*`) and `workflow_dispatch` triggers
- [x] 2.2 Configure macOS runner (`macos-latest`)
- [x] 2.3 Add step to install nightly Rust toolchain with `rust-src` component
- [x] 2.4 Add Cargo caching for nightly/watchOS target builds
- [x] 2.5 Add step to run `build-watchos.sh` for cross-compilation
- [x] 2.6 Add step to verify `libtennis_scorer.a` artifacts exist for both watchOS targets
- [x] 2.7 Add step to install cbindgen (`cargo install cbindgen`)
- [x] 2.8 Add step to regenerate FFI header and diff against committed header

## 3. API Deploy Workflow

- [x] 3.1 Create `.github/workflows/deploy.yml` with push-to-main trigger
- [x] 3.2 Add step to install stable Rust toolchain
- [x] 3.3 Add Cargo caching
- [x] 3.4 Add step for `cargo test -p tennis-scorer-api`
- [x] 3.5 Add step to build Docker image (placeholder, no push)
- [ ] 3.6 Create a minimal `Dockerfile` for the API crate

## 4. Verification

- [ ] 4.1 Push a test branch and verify Rust CI workflow triggers and passes
- [ ] 4.2 Trigger watchOS workflow manually and verify cross-compilation succeeds
- [ ] 4.3 Verify FFI header diff check passes when header is in sync
- [ ] 4.4 Verify FFI header diff check fails when header is intentionally stale
- [ ] 4.5 Verify API deploy workflow triggers on push to main
