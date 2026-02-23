# Next Work Package

## Step: Add 4 algorithm primitives to WASM bindings

## Goal

Expand WASM bindings from 17/23 to 21/23 Tier 1 symbols by adding `alg_simhash`, `alg_minhash_256`,
`alg_cdc_chunks`, and `soft_hash_video_v0`. This replicates the same 4 functions just added to
Node.js, adapted for wasm-bindgen type mappings. WASM and C FFI are the furthest behind at 17/23 —
closing this gap brings WASM to parity with Node.js.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs` — add 4 wasm-bindgen functions
- **Modify**: `crates/iscc-wasm/tests/unit.rs` — add unit tests for all 4 functions
- **Reference**: `crates/iscc-napi/src/lib.rs` (Node.js equivalents — `alg_simhash` through
    `soft_hash_video_v0`), `crates/iscc-lib/src/simhash.rs` (`alg_simhash` signature),
    `crates/iscc-lib/src/minhash.rs` (`alg_minhash_256` signature), `crates/iscc-lib/src/cdc.rs`
    (`alg_cdc_chunks` signature), `crates/iscc-lib/src/lib.rs` (`soft_hash_video_v0` at line 530)

## Not In Scope

- Adding these 4 functions to C FFI bindings — separate follow-up step
- `DataHasher`/`InstanceHasher` streaming classes for any binding — separate step
- Structured return types for gen functions (returning objects instead of plain `.iscc` strings)
- Changes to the Rust core crate — all 4 functions are already exported and tested
- Adding `js-sys` as a dependency — use `serde_wasm_bindgen` (already a dependency) for complex type
    conversions

## Implementation Notes

### Type mappings

Each function wraps the corresponding `iscc_lib` function with wasm-bindgen type conversions. Key
difference from napi-rs: wasm-bindgen supports `&[u8]` and `&str` by reference directly, but nested
arrays (`Vec<Vec<u8>>`, `Vec<Vec<i32>>`) require `JsValue` + `serde_wasm_bindgen` deserialization.

1. **`alg_simhash`** — `(hash_digests: JsValue) -> Result<Vec<u8>, JsError>`

    - Rust signature: `alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8>`
    - Input is an array of `Uint8Array` from JS — deserialize with
        `serde_wasm_bindgen::from_value::<Vec<Vec<u8>>>(hash_digests)`
    - Pass `&deserialized` to `iscc_lib::alg_simhash()` — `Vec<u8>` implements `AsRef<[u8]>`
    - Return `Vec<u8>` directly (wasm-bindgen converts to `Uint8Array`)
    - Wrap in `Result` for the deserialization error path

2. **`alg_minhash_256`** — `(features: Vec<u32>) -> Vec<u8>`

    - Rust signature: `alg_minhash_256(features: &[u32]) -> Vec<u8>`
    - `Vec<u32>` works directly as wasm-bindgen parameter (accepts JS number array or Uint32Array)
    - Pass `&features` to the core function
    - Return `Vec<u8>` directly
    - Infallible — no error mapping needed

3. **`alg_cdc_chunks`** — `(data: &[u8], utf32: bool, avg_chunk_size: Option<u32>) -> JsValue`

    - Rust signature: `alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]>`
    - `&[u8]` works directly as wasm-bindgen parameter
    - Default `avg_chunk_size` to 1024 via `.unwrap_or(1024)`
    - The return type `Vec<&[u8]>` needs conversion: collect into `Vec<Vec<u8>>` (owned), then
        serialize with `serde_wasm_bindgen::to_value()` to return as a JS array of `Uint8Array`
    - Return `JsValue` since wasm-bindgen can't natively express `Vec<Vec<u8>>`

4. **`soft_hash_video_v0`** — `(frame_sigs: JsValue, bits: Option<u32>) -> Result<Vec<u8>, JsError>`

    - Rust signature: `soft_hash_video_v0(&[Vec<i32>], u32) -> IsccResult<Vec<u8>>`
    - Same `JsValue` + `serde_wasm_bindgen` pattern already used by `gen_video_code_v0`
    - Default `bits` to 64 via `.unwrap_or(64)`
    - Map error with `.map_err(|e| JsError::new(&e.to_string()))`
    - Return `Vec<u8>` directly

### Naming and placement

Place the 4 functions after the existing `sliding_window` function, grouped under an
`// ── Algorithm primitives ──` section comment (matching the Node.js lib.rs layout).

### Tests

Add to `tests/unit.rs` in a new section after `sliding_window` tests:

- **`alg_simhash`**: Test with known byte inputs, verify output length matches input digest length.
    Test empty input returns 32 zero bytes. Use `wasm_bindgen::JsValue` +
    `serde_wasm_bindgen::to_value()` to construct input arrays in tests
- **`alg_minhash_256`**: Feed known `u32` feature values, verify output is 32 bytes
- **`alg_cdc_chunks`**: Feed known data, verify chunks concatenate to original. Test empty input
    returns one empty chunk. Deserialize the `JsValue` return via `serde_wasm_bindgen::from_value()`
    for assertions
- **`soft_hash_video_v0`**: Feed frame signatures, verify output length is `bits/8`. Test empty
    input returns error

Use `wasm_bindgen_test` attribute (already in dev-dependencies). Note: WASM tests run in Node.js via
`wasm-pack test --node`.

## Verification

- `cargo build -p iscc-wasm` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `wasm-pack test --node crates/iscc-wasm` passes all tests (20 existing + new algorithm tests)
- `cargo test --workspace` passes with 268+ tests (no regressions in other crates)

## Done When

All verification criteria pass: 4 new wasm-bindgen functions compile, clippy is clean
workspace-wide, and all WASM tests (existing 20 + new algorithm primitive tests) pass via
`wasm-pack test --node`.
