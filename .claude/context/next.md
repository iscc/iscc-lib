# Next Work Package

## Step: Scaffold WASM bindings crate with all 9 gen functions

## Goal

Create the `crates/iscc-wasm/` binding crate with wasm-bindgen wrappers for all 9 `gen_*_v0`
functions, following the same thin-wrapper pattern as the napi and PyO3 crates. This is the first
step toward browser-compatible ISCC support.

## Scope

- **Create**: `crates/iscc-wasm/Cargo.toml`, `crates/iscc-wasm/src/lib.rs`
- **Modify**: `Cargo.toml` (root — add `crates/iscc-wasm` to workspace members, add `wasm-bindgen`
    and `serde-wasm-bindgen` to workspace dependencies)
- **Reference**: `crates/iscc-napi/src/lib.rs` (thin-wrapper pattern), `crates/iscc-napi/Cargo.toml`
    (crate config pattern), `notes/02-language-bindings.md` (WASM architecture),
    `crates/iscc-lib/src/lib.rs` (core API signatures)

## Implementation Notes

### Crate setup (`crates/iscc-wasm/Cargo.toml`)

```toml
[package]
name = "iscc-wasm"
version.workspace = true
edition.workspace = true
publish = false          # Published via npm, not crates.io

[lib]
crate-type = ["cdylib"]

[dependencies]
iscc-lib = { path = "../iscc-lib" }
wasm-bindgen = { workspace = true }
serde-wasm-bindgen = { workspace = true }
serde = { workspace = true }
js-sys = { workspace = true }
```

### Root `Cargo.toml` changes

Add to `[workspace]` members: `"crates/iscc-wasm"`

Add to `[workspace.dependencies]`:

```toml
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
js-sys = "0.3"
```

### Binding wrappers (`crates/iscc-wasm/src/lib.rs`)

Follow the identical thin-wrapper pattern from the napi crate. Each function:

1. Accepts wasm-bindgen-compatible types
2. Converts to Rust core types
3. Calls `iscc_lib::gen_*_v0`
4. Maps errors via `JsValue` (use `JsError::new(&e.to_string())`)

**Type mappings for wasm-bindgen:**

| Core Rust type | WASM binding type | Conversion                                                     |
| -------------- | ----------------- | -------------------------------------------------------------- |
| `&str`         | `&str`            | Direct — wasm-bindgen handles this                             |
| `Option<&str>` | `Option<String>`  | `.as_deref()`                                                  |
| `&[u8]`        | `&[u8]`           | Direct — wasm-bindgen handles this                             |
| `&[i32]`       | `Vec<i32>`        | Pass `&cv`                                                     |
| `u32`          | `Option<u32>`     | `.unwrap_or(64)` for bits, `.unwrap_or(false)` for wide        |
| `&[&str]`      | `JsValue`         | `serde_wasm_bindgen::from_value::<Vec<String>>()` then convert |
| `&[Vec<i32>]`  | `JsValue`         | `serde_wasm_bindgen::from_value::<Vec<Vec<i32>>>()`            |

**Functions requiring `JsValue` for complex types:**

- `gen_video_code_v0` — `frame_sigs: JsValue` (array of arrays of i32)
- `gen_mixed_code_v0` — `codes: JsValue` (array of strings)
- `gen_iscc_code_v0` — `codes: JsValue` (array of strings)

**Functions with simple wasm-bindgen types (no JsValue needed):**

- `gen_meta_code_v0(name, description, meta, bits)`
- `gen_text_code_v0(text, bits)`
- `gen_image_code_v0(pixels, bits)`
- `gen_audio_code_v0(cv, bits)`
- `gen_data_code_v0(data, bits)`
- `gen_instance_code_v0(data, bits)`

Use `wasm_bindgen::JsError` for error returns (provides `impl From<JsError> for JsValue`), so the
return type is `Result<String, JsError>`.

For `JsValue` parameters, deserialize and map deserialization errors to `JsError` as well.

### Important: wasm32 target

Before verifying, install the wasm32 target: `rustup target add wasm32-unknown-unknown`

The crate must compile with: `cargo check -p iscc-wasm --target wasm32-unknown-unknown`

Note: `cargo check -p iscc-wasm` (without `--target`) will also work for basic Rust type checking,
but the definitive check requires the wasm32 target.

## Verification

- `crates/iscc-wasm/Cargo.toml` exists with correct dependencies
- `crates/iscc-wasm/src/lib.rs` contains all 9 `gen_*_v0` wasm-bindgen wrappers
- Root `Cargo.toml` lists `crates/iscc-wasm` in workspace members
- `wasm-bindgen`, `serde-wasm-bindgen`, and `js-sys` are in `[workspace.dependencies]`
- `cargo check -p iscc-wasm --target wasm32-unknown-unknown` succeeds (after `rustup target add`)
- `cargo test -p iscc-lib` still passes (143 tests — no regression)
- `cargo clippy -p iscc-lib -- -D warnings` is clean
- `cargo fmt --all --check` is clean

## Done When

The `crates/iscc-wasm/` crate exists with all 9 wasm-bindgen function wrappers, compiles for the
wasm32-unknown-unknown target, and all existing Rust verification criteria pass without regression.
