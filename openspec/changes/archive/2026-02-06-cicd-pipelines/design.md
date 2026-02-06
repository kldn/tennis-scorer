## Context

The tennis-scorer project is a Cargo workspace with two crates (`tennis-scorer` and `tennis-scorer-api`) at `crates/`. It supports cross-compilation to watchOS via `build-watchos.sh`, which uses nightly Rust to build for Tier 3 targets (`aarch64-apple-watchos` and `aarch64-apple-watchos-sim`). The `.cargo/config.toml` enables `build-std = ["std"]` for these targets. The `cbindgen.toml` at the repository root configures C header generation for the FFI bridge.

There are no existing CI/CD pipelines. All checks are manual.

## Goals / Non-Goals

**Goals:**
- Automate Rust formatting, linting, testing, and release builds on every push/PR
- Automate watchOS cross-compilation verification on tags and manual dispatch
- Verify the cbindgen FFI header stays in sync with the Rust source
- Provide a placeholder API deploy pipeline for future use

**Non-Goals:**
- Deploying the API to a production environment (future work)
- Building the Xcode/Swift portion of the watchOS app in CI
- Publishing crates to crates.io
- Running integration or end-to-end tests

## Decisions

### D1: GitHub Actions runner selection

**Choice**: Use `ubuntu-latest` for the Rust CI and API deploy workflows. Use `macos-latest` for the watchOS build workflow.

**Rationale**:
- Ubuntu runners are faster to provision and cheaper on GitHub Actions
- Rust CI (fmt, clippy, test, build) does not require macOS-specific tooling
- watchOS cross-compilation requires the Apple SDK and Xcode tooling, which are only available on macOS runners
- The `build-watchos.sh` script uses `cargo +nightly` with `-Z build-std`, which requires the macOS linker for watchOS targets

**Alternatives considered**:
- Using macOS for all workflows: More expensive and slower to provision, with no benefit for standard Rust CI tasks

### D2: Nightly toolchain installation for watchOS

**Choice**: Install the nightly toolchain with `rust-src` component using `rustup` in the watchOS workflow. Do not install it in the Rust CI workflow.

**Rationale**:
- The `.cargo/config.toml` has `build-std = ["std"]` under `[unstable]`, which requires `rust-src`
- `build-watchos.sh` explicitly uses `cargo +nightly`
- The Rust CI workflow uses stable Rust and does not need nightly or `rust-src`

### D3: Caching strategy

**Choice**: Use `actions/cache` (or `Swatinem/rust-cache`) to cache `~/.cargo/registry`, `~/.cargo/git`, and the `target/` directory. Key the cache on the hash of `Cargo.lock`.

**Rationale**:
- Dependency downloads and compilation dominate CI build time
- Keying on `Cargo.lock` ensures the cache is invalidated when dependencies change
- The `target/` directory contains compiled artifacts that can be reused across runs
- `Swatinem/rust-cache` is a widely-used action that handles Rust-specific caching automatically

**Alternatives considered**:
- No caching: Simpler but significantly slower (full dependency download and compile on every run)
- Caching only registry: Misses the compiled dependency artifacts in `target/`

### D4: cbindgen header diff check approach

**Choice**: In the watchOS workflow, run `cbindgen` to regenerate the header into a temporary file, then `diff` it against the committed header. Fail the workflow if there is any difference.

**Rationale**:
- This ensures developers do not forget to regenerate the header after changing FFI-exposed types
- Using `diff` provides clear output showing exactly what changed
- The check runs in the watchOS workflow because that is where the header matters most (Swift interop)
- cbindgen is already a build-dependency of the `tennis-scorer` crate, so it is available via `cargo install cbindgen`

**Implementation**:
```
cbindgen --config cbindgen.toml --crate tennis-scorer --output /tmp/tennis_scorer.h
diff watchos/TennisScorer/tennis_scorer.h /tmp/tennis_scorer.h
```

### D5: API deploy pipeline scope

**Choice**: The API deploy workflow will only run `cargo test -p tennis-scorer-api` and build a Docker image locally. It will not push the image or deploy to any environment.

**Rationale**:
- The `tennis-scorer-api` crate currently has minimal functionality
- A full deployment pipeline requires infrastructure decisions (container registry, hosting) that are out of scope
- Building the Docker image verifies that the Dockerfile and dependencies are correct
- Pushing and deploying can be added later when the API is ready

## Risks / Trade-offs

- **[Risk] macOS runner cost**: macOS runners on GitHub Actions are more expensive than Linux runners. Limiting the watchOS workflow to tags and manual dispatch mitigates this.
- **[Risk] Nightly toolchain breakage**: The watchOS build depends on nightly Rust, which can have regressions. Pinning to a specific nightly date could improve reliability but adds maintenance burden.
- **[Trade-off] No Swift/Xcode build in CI**: The watchOS workflow only compiles the Rust library, not the full Xcode project. This is intentional to avoid the complexity of Xcode CI setup, but means Swift compilation errors will not be caught.
- **[Trade-off] API Docker build without push**: The deploy workflow builds but does not push the image. This validates the build process but does not test actual deployment.
