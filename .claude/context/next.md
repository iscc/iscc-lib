# Next Work Package

## Step: Scaffold napi-rs crate with all 9 gen functions

## Goal

Create the `crates/iscc-napi/` crate exposing all 9 `gen_*_v0` functions as a native Node.js addon
via napi-rs. This is the first step toward the `@iscc/lib` npm package target and establishes the
Node.js binding layer following the same thin-wrapper pattern used by the PyO3 bindings.

## Scope

- **Create**: `crates/iscc-napi/Cargo.toml`, `crates/iscc-napi/build.rs`,
    `crates/iscc-napi/src/lib.rs`, `crates/iscc-napi/package.json`
- **Modify**: root `Cargo.toml` (add workspace member + napi dependencies)
- **Reference**: `crates/iscc-py/src/lib.rs` (binding wrapper pattern), `crates/iscc-py/Cargo.toml`
    (crate structure), `notes/02-language-bindings.md` (napi-rs architecture)

## Implementation Notes

1. **Root `Cargo.toml`** — add `"crates/iscc-napi"` to `workspace.members` and add napi workspace
    dependencies:

    ```toml
    napi = { version = "3", default-features = false, features = ["napi6"] }
    napi-derive = "3"
    napi-build = "2"
    ```

2. **`crates/iscc-napi/Cargo.toml`** — follow the iscc-py pattern:

    ```toml
    [package]
    name = "iscc-napi"
    version.workspace = true
    edition.workspace = true
    publish = false          # Published via npm, not crates.io

    [lib]
    crate-type = ["cdylib"]

    [dependencies]
    iscc-lib = { path = "../iscc-lib" }
    napi = { workspace = true }
    napi-derive = { workspace = true }

    [build-dependencies]
    napi-build = { workspace = true }
    ```

3. **`crates/iscc-napi/build.rs`** — minimal napi build script:

    ```rust
    fn main() {
        napi_build::setup();
    }
    ```

4. **`crates/iscc-napi/src/lib.rs`** — expose all 9 functions using `#[napi]` attribute macro.
    Follow the same thin-wrapper pattern as the PyO3 bindings. Key type mappings from Rust to napi:

    - `&str` → `String` (napi uses owned strings)
    - `&[u8]` → `napi::bindgen_prelude::Buffer` for byte data
    - `Vec<i32>` → `Vec<i32>` (napi supports this directly)
    - `Vec<Vec<i32>>` → `Vec<Vec<i32>>`
    - `Vec<String>` → `Vec<String>`
    - `Option<String>` → `Option<String>`
    - Return type: `napi::Result<String>` for error propagation

    Error handling: convert `iscc_lib::IsccError` to `napi::Error` via
    `map_err(|e| napi::Error::from_reason(e.to_string()))`.

    Function naming: use `#[napi]` which auto-converts Rust snake_case to JS camelCase. To keep the
    Python-compatible snake_case names, use `#[napi(js_name = "gen_meta_code_v0")]`.

    For `gen_data_code_v0` and `gen_instance_code_v0`: accept `Buffer` and call `.as_ref()` to get
    `&[u8]`.

    For `gen_image_code_v0`: accept `Buffer` (pixels as raw bytes) and call `.as_ref()`.

    For `gen_audio_code_v0`: accept `Vec<i32>` and pass `&cv`.

    For `gen_video_code_v0`: accept `Vec<Vec<i32>>` and convert to `&[Vec<i32>]` for the Rust API
    (check if the Rust API takes `&[Vec<i32>]` or `&[&[i32]]` — match what the Rust core expects).

    For `gen_mixed_code_v0` and `gen_iscc_code_v0`: accept `Vec<String>`, convert to `Vec<&str>` via
    `.iter().map(|s| s.as_str()).collect()`, same as PyO3 bindings.

    Default parameter values: napi-rs does not support default parameters natively like PyO3. Use
    `Option<u32>` for `bits` with `.unwrap_or(64)` in the function body. Similarly, use
    `Option<bool>` for `wide` with `.unwrap_or(false)`.

5. **`crates/iscc-napi/package.json`** — minimal npm package config:

    ```json
    {
      "name": "@iscc/lib",
      "version": "0.1.0",
      "license": "Apache-2.0",
      "main": "index.js",
      "types": "index.d.ts",
      "napi": {
        "name": "iscc-lib",
        "triples": {}
      },
      "scripts": {
        "build": "napi build --platform --release",
        "build:debug": "napi build --platform"
      },
      "devDependencies": {
        "@napi-rs/cli": "^3"
      }
    }
    ```

6. **Check the Rust API for `gen_video_code_v0`** — read `crates/iscc-lib/src/lib.rs` to verify the
    exact Rust type signature before implementing the napi wrapper.

## Verification

- `cargo build -p iscc-napi` compiles without errors
- `cargo clippy -p iscc-napi -- -D warnings` is clean
- `cargo fmt --all --check` passes
- `cargo test -p iscc-lib` still passes (143 tests — core unaffected)
- `uv run pytest tests/` still passes (49 tests — Python bindings unaffected)
- All 9 `gen_*_v0` functions are present in `src/lib.rs` with `#[napi]` attributes

## Done When

`cargo build -p iscc-napi` compiles successfully with all 9 `gen_*_v0` functions exposed via
`#[napi]`, clippy is clean, and existing Rust + Python tests still pass.
