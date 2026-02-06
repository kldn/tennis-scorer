## 1. Create workspace directory structure

- [x] 1.1 Create `crates/tennis-scorer/` directory
- [x] 1.2 Move `src/` to `crates/tennis-scorer/src/`
- [x] 1.3 Create `crates/tennis-scorer/Cargo.toml` with package metadata, lib config, and dependencies from current root Cargo.toml
- [x] 1.4 Convert root `Cargo.toml` to workspace manifest with `members = ["crates/*"]` and `resolver = "3"`

## 2. Scaffold API crate

- [x] 2.1 Create `crates/tennis-scorer-api/Cargo.toml` with `tennis-scorer` path dependency
- [x] 2.2 Create `crates/tennis-scorer-api/src/lib.rs` with a basic import from `tennis-scorer` to verify dependency resolution

## 3. Update build configuration

- [x] 3.1 Update `build-watchos.sh` to use `-p tennis-scorer` flag
- [x] 3.2 Update `cbindgen.toml` parse paths to find crate under `crates/tennis-scorer`

## 4. Verify everything works

- [x] 4.1 Run `cargo build` from workspace root — all members compile
- [x] 4.2 Run `cargo test -p tennis-scorer` — all 55 tests pass
- [x] 4.3 Run cbindgen to regenerate header — output matches current header
