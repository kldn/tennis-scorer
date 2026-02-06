## Context

The project is currently a single Rust crate at the repository root. We need to add a sibling API crate that depends on the core scoring engine. Cargo workspaces are the standard Rust mechanism for multi-crate repositories.

The key constraint is that the watchOS build pipeline, Xcode project, and cbindgen header generation must continue working after restructuring.

## Goals / Non-Goals

**Goals:**
- Convert to Cargo workspace with `crates/tennis-scorer/` and `crates/tennis-scorer-api/`
- All existing tests pass unchanged
- watchOS build script continues to produce static libraries
- cbindgen header generation works

**Non-Goals:**
- Implementing the API backend (just scaffold an empty crate)
- Changing any scoring logic
- Modifying the Watch App Swift code

## Decisions

### 1. Directory layout: `crates/` subdirectory

**Decision**: Place workspace members under `crates/`.

```
tennis-scorer/
├── Cargo.toml          # [workspace] members = ["crates/*"]
├── crates/
│   ├── tennis-scorer/      # core library (moved from root)
│   │   ├── Cargo.toml
│   │   └── src/
│   └── tennis-scorer-api/  # empty placeholder
│       ├── Cargo.toml
│       └── src/lib.rs
├── .cargo/config.toml  # stays at workspace root
├── build-watchos.sh    # updated paths
├── cbindgen.toml       # updated crate path
├── include/            # stays at workspace root
├── WatchApp/           # unchanged
└── openspec/           # unchanged
```

**Rationale**: `crates/` is a widely-used Rust convention. Keeps root clean. Glob pattern `crates/*` auto-includes new crates.

### 2. Workspace-level target directory stays at root

**Decision**: Use the default workspace behavior — `target/` at workspace root.

**Rationale**: Cargo workspaces already share a single `target/` directory. Since our current `target/` is at the repo root, output paths like `target/aarch64-apple-watchos-sim/release/libtennis_scorer.a` remain identical. The Xcode project library search paths (`$(PROJECT_DIR)/../target/...`) don't need changing.

### 3. Root Cargo.toml uses `workspace.members` glob

```toml
[workspace]
members = ["crates/*"]
resolver = "3"
```

**Rationale**: Glob pattern means adding future crates (e.g., `tennis-scorer-cli`) requires no Cargo.toml edit.

### 4. API crate is a minimal placeholder

The `tennis-scorer-api` crate starts with just:
```toml
[package]
name = "tennis-scorer-api"
version = "0.1.0"
edition = "2024"

[dependencies]
tennis-scorer = { path = "../tennis-scorer" }
```

And a `src/lib.rs` with a basic import to verify the dependency works.

## Risks / Trade-offs

- **[Risk] cbindgen path resolution** → cbindgen needs `--manifest-dir` or updated `[parse]` config to find the crate under `crates/`. Mitigation: update `cbindgen.toml` parse paths and test.

- **[Risk] Xcode project breaks** → Library search paths use `$(PROJECT_DIR)/../target/...` which should still work since workspace `target/` is at repo root. Mitigation: verify after conversion (manual Xcode build).

- **[Risk] `build-watchos.sh` path assumptions** → Script uses `cargo +nightly build` without `-p`. In a workspace, this builds all members. Mitigation: add `-p tennis-scorer` flag.
