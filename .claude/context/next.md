# Next Work Package

## Step: Propagate 7 Tier 1 symbols to WASM bindings

## Goal

Add the 7 missing Tier 1 symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4
algorithm constants) to the WASM binding crate, bringing it from 23/30 to 30/30 Tier 1 parity with
Rust core, Python, and Node.js.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-wasm/src/lib.rs` — add 4 constant getter functions, `encode_component`,
        `iscc_decode` (with `IsccDecodeResult` struct), and `json_to_data_url`
    - `crates/iscc-wasm/tests/unit.rs` — add tests for all 7 new symbols
- **Reference**:
    - `crates/iscc-napi/src/lib.rs` — Node.js implementation of same 7 symbols (pattern to mirror)
    - `crates/iscc-napi/__tests__/codec.test.mjs` — Node.js test patterns for reference
    - `crates/iscc-wasm/CLAUDE.md` — WASM-specific type mappings and conventions
    - `crates/iscc-wasm/src/lib.rs` — existing wasm-bindgen patterns

## Not In Scope

- Updating `crates/iscc-wasm/CLAUDE.md` API surface list (defer to review or later step)
- Propagating symbols to C FFI, Java JNI, or Go bindings (separate future steps)
- Returning full result structs from gen functions (they continue returning `.iscc` string only)
- Adding `gen_meta_code_v0` dict/object support for the `meta` parameter (issue #5)
- Publishing `@iscc/wasm` to npm

## Implementation Notes

### Constants

wasm-bindgen does not support `#[wasm_bindgen]` on `pub const`. Export 4 getter functions instead:

```rust
#[wasm_bindgen(js_name = "META_TRIM_NAME")]
pub fn meta_trim_name() -> u32 {
    iscc_lib::META_TRIM_NAME as u32
}
```

Same pattern for `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`. The `as u32` cast is
safe — all values fit within u32 (max is 4_194_304). Place these at the top of the file in a
`// ── Constants ──` section, right after the `use` imports.

### encode_component

```rust
#[wasm_bindgen]
pub fn encode_component(
    mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8],
) -> Result<String, JsError> {
    iscc_lib::encode_component(mtype, stype, version, bit_length, digest)
        .map_err(|e| JsError::new(&e.to_string()))
}
```

Note: wasm-bindgen accepts `&[u8]` directly (unlike napi-rs which needs `Buffer`).

### iscc_decode

Return a `#[wasm_bindgen(getter_with_clone)]` struct with public fields. The `digest` field is
`Vec<u8>` (mapped to `Uint8Array` in JS). `getter_with_clone` is needed because `Vec<u8>` is not
`Copy`. Define the struct:

```rust
#[wasm_bindgen(getter_with_clone)]
pub struct IsccDecodeResult {
    pub maintype: u8,
    pub subtype: u8,
    pub version: u8,
    pub length: u8,
    pub digest: Vec<u8>,
}
```

The function:

```rust
#[wasm_bindgen]
pub fn iscc_decode(iscc: &str) -> Result<IsccDecodeResult, JsError> {
    let (mt, st, vs, li, digest) =
        iscc_lib::iscc_decode(iscc).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(IsccDecodeResult {
        maintype: mt, subtype: st, version: vs, length: li, digest,
    })
}
```

Place this struct and function in the `// ── Codec ──` section near `iscc_decompose`.

### json_to_data_url

```rust
#[wasm_bindgen]
pub fn json_to_data_url(json: &str) -> Result<String, JsError> {
    iscc_lib::json_to_data_url(json).map_err(|e| JsError::new(&e.to_string()))
}
```

Note: takes `&str` (not owned `String`) — wasm-bindgen supports `&str` directly, unlike napi-rs.
Place in the `// ── Encoding ──` section near `encode_base64`.

### Tests

Add tests in `crates/iscc-wasm/tests/unit.rs` using `#[wasm_bindgen_test]`:

- 4 constant tests: verify each returns expected value (128, 4096, 4194304, 13)
- `encode_component`: encode a known Meta-Code component, verify result string is valid base32
- `iscc_decode`: decode a known ISCC unit, verify all 5 fields (maintype, subtype, version, length,
    digest bytes)
- `encode_component` + `iscc_decode` roundtrip: encode then decode, verify fields match inputs
- `json_to_data_url`: basic JSON → `data:application/json;base64,...`, JSON-LD with `@context` →
    `data:application/ld+json;base64,...`

Aim for ~12-15 new `#[wasm_bindgen_test]` functions covering all 7 symbols.

## Verification

- `wasm-pack test --node crates/iscc-wasm` passes (54 existing + new tests)
- `wasm-pack test --node crates/iscc-wasm --features conformance` passes
- `cargo clippy -p iscc-wasm --all-targets -- -D warnings` clean
- `grep -c '#\[wasm_bindgen' crates/iscc-wasm/src/lib.rs` shows at least 32 (was 25, +7 new
    annotations)

## Done When

All verification criteria pass — WASM crate exports all 30 Tier 1 symbols with tests covering the 7
newly added symbols.
