# Next Work Package

## Step: Bootstrap Rust workspace with core crate skeleton

## Goal

Create the minimal Rust workspace that compiles and passes `cargo test -p iscc-lib`: a virtual
workspace root `Cargo.toml` and a `crates/iscc-lib/` core crate with public stub functions for all 9
`gen_*_v0` ISCC entrypoints. This unblocks every subsequent implementation step.

## Scope

- **Create**: `Cargo.toml` (workspace root — virtual workspace, no `[package]`)
- **Create**: `crates/iscc-lib/Cargo.toml` (core lib crate, no PyO3 or binding deps)
- **Create**: `crates/iscc-lib/src/lib.rs` (public stubs for all 9 `gen_*_v0` functions + one smoke
    test per function)

## Reference

- `notes/01-workspace-structure.md` — workspace layout, `workspace.dependencies` pattern,
    `[workspace.package]` block
- `notes/04-api-compatibility-safety.md` — Tier 1 API surface, error model
- deepwiki `iscc/iscc-core` — for actual function signatures (use `ask_question` to query "What are
    the input types and return types of gen_meta_code_v0, gen_text_code_v0, gen_image_code_v0,
    gen_audio_code_v0, gen_video_code_v0, gen_mixed_code_v0, gen_data_code_v0, gen_instance_code_v0,
    gen_iscc_code_v0?")

## Implementation Notes

### Workspace root `Cargo.toml`

Virtual workspace (no `[package]`). Only one member for now:

```toml
[workspace]
resolver = "2"
members = ["crates/iscc-lib"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
authors = ["Titusz Pan <tp@py7.de>"]
license = "Apache-2.0"
repository = "https://github.com/iscc/iscc-lib"
homepage = "https://lib.iscc.codes"
description = "High-performance Rust implementation of ISO 24138:2024 (ISCC)"

[workspace.dependencies]
thiserror = "2"

[profile.release]
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### `crates/iscc-lib/Cargo.toml`

```toml
[package]
name = "iscc-lib"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
description.workspace = true
keywords = ["iscc", "content-id", "media", "fingerprint", "iso"]
categories = ["multimedia", "encoding", "cryptography"]

[dependencies]
thiserror.workspace = true
```

### `crates/iscc-lib/src/lib.rs`

Structure: module-level docstring, a public `IsccError` enum (using `thiserror`), a public
`IsccResult<T>` type alias, and 9 public stub functions. Each stub should
`Err(IsccError::NotImplemented)` for now (do NOT use `todo!()` — it panics and would fail tests;
return a Result instead).

Function signatures — query deepwiki iscc-core or use these minimal stubs based on the standard:

```rust
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    extra: Option<&str>,
    bits: u32,
) -> IsccResult<String> { ... }

pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String> { ... }
pub fn gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<String> { ... }
pub fn gen_audio_code_v0(data: &[f32], bits: u32) -> IsccResult<String> { ... }
pub fn gen_video_code_v0(frames: &[&[u8]], bits: u32) -> IsccResult<String> { ... }
pub fn gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<String> { ... }
pub fn gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<String> { ... }
pub fn gen_instance_code_v0(data: &[u8]) -> IsccResult<String> { ... }
pub fn gen_iscc_code_v0(codes: &[&str]) -> IsccResult<String> { ... }
```

**Important**: Check deepwiki `iscc/iscc-core` for the exact Python signatures to get parameter
names and types right — especially for `gen_meta_code_v0` which has string-typed inputs that map
well to `&str`. The signatures above are reasonable approximations; adjust based on what you find.

The `IsccError` enum:

```rust
#[derive(Debug, thiserror::Error)]
pub enum IsccError {
    #[error("not implemented")]
    NotImplemented,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
pub type IsccResult<T> = Result<T, IsccError>;
```

### Tests

Add a `#[cfg(test)]` module in `lib.rs` with one smoke test per function that just checks the stub
returns `Err(IsccError::NotImplemented)`. This proves the test harness works and every function is
callable from test code.

Example:

```rust
#[test]
fn test_gen_meta_code_v0_stub() {
    assert!(matches!(
        gen_meta_code_v0("test", None, None, None, 64),
        Err(IsccError::NotImplemented)
    ));
}
```

## Verification

- `cargo check --workspace` exits 0 with no errors
- `cargo test -p iscc-lib` exits 0 and all 9 stub tests pass
- `cargo clippy -p iscc-lib -- -D warnings` exits 0 (no warnings)
- `cargo fmt --check` exits 0 (code is properly formatted)
- No `unsafe` blocks in any created file

## Done When

The advance agent is done when all five verification criteria pass cleanly.
