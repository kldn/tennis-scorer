## Why

The project needs a Rust API backend (`tennis-scorer-api`) that shares types with the core scoring engine. Currently the project is a single crate — it cannot be referenced as a dependency by a sibling crate. Converting to a Cargo workspace enables multiple crates to coexist and share code, which is a prerequisite for the API backend and any future Rust crates.

## What Changes

- Convert root `Cargo.toml` from a single package to a workspace definition
- Move existing source code into `crates/tennis-scorer/` with its own `Cargo.toml`
- Move `.cargo/config.toml` to workspace root (stays in place)
- Update `build-watchos.sh` to target the workspace member via `-p tennis-scorer`
- Update `cbindgen.toml` paths to point to the new crate location
- Scaffold an empty `crates/tennis-scorer-api/` crate (placeholder for future API backend)
- Update Xcode project library search paths to reflect new `target/` output structure
- **BREAKING**: Library output path changes from `target/<target>/release/` to the same path (Cargo workspace shares the `target/` directory at workspace root)

## Capabilities

### New Capabilities
- `workspace-structure`: Project organized as a Cargo workspace with multiple crates sharing a common target directory

### Modified Capabilities
(none — no spec-level behavior changes, this is a structural refactoring)

## Impact

- **Cargo.toml**: Root becomes workspace manifest; new crate-level Cargo.toml created
- **Source files**: All `src/*.rs` files move to `crates/tennis-scorer/src/`
- **Build scripts**: `build-watchos.sh` needs path updates
- **cbindgen.toml**: Crate path reference update
- **Xcode project**: Library search paths may need verification (workspace `target/` is at root, same as before)
- **CI/CD**: Future workflows reference workspace structure
- **.cargo/config.toml**: Stays at workspace root (Cargo looks for it there)
