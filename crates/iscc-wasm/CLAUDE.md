# CLAUDE.md — iscc-wasm

Browser-compatible WASM bindings for iscc-lib, published as `@iscc/wasm` on npm.

## Crate Role

- Spoke crate in the hub-and-spoke model: depends on `iscc-lib` (pure Rust core), adds
    `wasm-bindgen` annotations
- Thin wrapper layer only -- all logic lives in `iscc-lib`; this crate translates types and maps
    errors
- `publish = false` in Cargo.toml -- published to npm via `wasm-pack`, never to crates.io
- Stability tier: "best-effort" -- versioned by npm package version, not SemVer-governed like the
    Rust core

## Module Layout

```
crates/iscc-wasm/
  Cargo.toml          # cdylib + rlib, wasm-bindgen deps, publish = false
  src/
    lib.rs            # All #[wasm_bindgen] exports (single file, flat)
  tests/
    conformance.rs    # 9 gen_*_v0 functions vs vendored data.json vectors
    unit.rs           # Text utils, encoding, codec, algorithm primitives
```

- `src/lib.rs` contains every exported function. No submodules -- keep it flat until the file
    exceeds ~500 lines.
- Tests use `wasm_bindgen_test` and run in a Node.js WASM runtime.

## Type Mapping (Rust to JS via wasm-bindgen)

| Rust type            | JS type                | How                                                 |
| -------------------- | ---------------------- | --------------------------------------------------- |
| `&str`               | `string`               | Direct wasm-bindgen conversion                      |
| `String`             | `string`               | Direct wasm-bindgen conversion                      |
| `Option<String>`     | `string \| undefined`  | wasm-bindgen handles Option natively                |
| `Option<u32>`        | `number \| undefined`  | wasm-bindgen handles Option natively                |
| `Option<bool>`       | `boolean \| undefined` | wasm-bindgen handles Option natively                |
| `&[u8]`              | `Uint8Array`           | Direct wasm-bindgen conversion                      |
| `Vec<u8>`            | `Uint8Array`           | Direct wasm-bindgen conversion                      |
| `Vec<String>`        | `string[]`             | Direct wasm-bindgen conversion                      |
| `Vec<i32>`           | `Int32Array`           | Direct wasm-bindgen conversion                      |
| `Vec<u32>`           | `Uint32Array`          | Direct wasm-bindgen conversion                      |
| `Vec<Vec<i32>>`      | `JsValue`              | Use `serde_wasm_bindgen::from_value` to deserialize |
| `Vec<Vec<u8>>`       | `JsValue`              | Use `serde_wasm_bindgen::to_value` to serialize     |
| `Result<T, JsError>` | throws on Err          | `JsError::new(&e.to_string())`                      |

**Key rules:**

- Use `serde_wasm_bindgen` for nested collections (`Vec<Vec<T>>`, `Vec<String>` from JS arrays).
    Never use `serde_json` in the WASM path -- it adds ~50KB.
- Errors are always `Result<T, JsError>`. Map `iscc_lib::Error` with
    `.map_err(|e| JsError::new(&e.to_string()))`.
- Functions return the `iscc` string field from the core result struct, not the full struct.

## Browser vs Node.js Targets

`wasm-pack` supports three build targets:

- `--target bundler` -- for webpack/vite (default for browser apps)
- `--target web` -- native ESM in browsers, no bundler needed
- `--target nodejs` -- Node.js fallback (prefer `iscc-napi` for Node.js server use)

All targets produce from the same source. No `#[cfg]` divergence is needed.

The crate targets `wasm32-unknown-unknown` (browser WASM). WASI is not supported. Evaluate the WASM
Component Model only when WIT tooling is stable, browsers support components natively, and `jco`
output size is competitive with `wasm-pack`.

## Build Commands

```bash
# Build for bundler (webpack/vite)
wasm-pack build crates/iscc-wasm --target bundler

# Build for native ESM in browsers
wasm-pack build crates/iscc-wasm --target web

# Build for Node.js
wasm-pack build crates/iscc-wasm --target nodejs

# Release build with size optimization
wasm-pack build crates/iscc-wasm --release --target bundler
```

The release profile uses `wasm-opt = ["-Os"]` for size optimization (configured via
`[package.metadata.wasm-pack.profile.release]` in Cargo.toml -- add this if missing).

## Test Commands

```bash
# Run all WASM tests in Node.js runtime
wasm-pack test --node crates/iscc-wasm

# Run specific test file
wasm-pack test --node crates/iscc-wasm -- --test conformance
wasm-pack test --node crates/iscc-wasm -- --test unit
```

- Tests use `#[wasm_bindgen_test]` attribute, not `#[test]`.
- Conformance vectors are loaded at compile time via
    `include_str!("../../iscc-lib/tests/data.json")`.
- Each conformance test function asserts an exact count of test cases to catch missing vectors.
- No network access in tests. All test data is vendored.

## Publishing (npm via wasm-pack)

- Package name: `@iscc/wasm` under the `@iscc` npm scope
- Auth: `NPM_TOKEN` repository secret (OIDC trusted publishing not available for npm)
- Workflow: tag-triggered CI builds with `wasm-pack build --release`, publishes via
    `wasm-pack publish`
- `package.json` must exist in the crate root with `name`, `version`, and `files` fields

## Exported API Surface

All 22 Tier 1 symbols are bound. Every `#[wasm_bindgen]` function in `lib.rs` maps 1:1 to an
`iscc_lib` public function:

- **9 gen functions:** `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`,
    `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`,
    `gen_instance_code_v0`, `gen_iscc_code_v0`
- **4 text utils:** `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- **4 algorithm primitives:** `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`
- **1 soft hash:** `soft_hash_video_v0`
- **1 encoding:** `encode_base64`
- **1 codec:** `iscc_decompose`
- **1 diagnostic:** `conformance_selftest`

`DataHasher` and `InstanceHasher` (streaming types) are not yet bound. Binding stateful types
requires `#[wasm_bindgen]` on a struct with constructor/method annotations.

## Common Pitfalls

- **Do not use `serde_json` at runtime.** Use `serde_wasm_bindgen` for all JS-to-Rust and Rust-to-JS
    conversions. `serde_json` is a dev-dependency only (for parsing test vectors).
- **Do not add logic in this crate.** All computation belongs in `iscc-lib`. This crate is a
    translation layer.
- **Do not expose Rust structs directly.** Return primitive types (`String`, `Vec<u8>`,
    `Vec<String>`) or `JsValue`. The gen functions return the `.iscc` string field, not the full
    result struct.
- **Do not panic across WASM boundary.** Every fallible path must return `Result<T, JsError>`.
    Panics in WASM abort the runtime.
- **Do not add `#[wasm_bindgen]` to internal helpers.** Only Tier 1 API functions get the
    annotation.
- **Do not use `Option<&str>`.** wasm-bindgen does not support `Option<&str>` -- use
    `Option<String>` and convert with `.as_deref()`.
- **`Vec<i32>` in function signatures becomes `Int32Array`.** If JS callers pass a regular array
    instead of a typed array, use `JsValue` + `serde_wasm_bindgen::from_value` for the parameter.
    See `gen_audio_code_v0` (takes `Vec<i32>`) vs `gen_video_code_v0` (takes `JsValue`).
- **Conformance test counts are hardcoded.** When upstream `data.json` adds test cases, update the
    `assert_eq!(tested, N, ...)` counts in `conformance.rs`.
- **`crate-type` must include `cdylib`.** The `rlib` target is included for test compilation. Do not
    remove either.
