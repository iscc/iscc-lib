# Next Work Package

## Step: Propagate 7 new Tier 1 symbols to Node.js bindings

## Goal

Add `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 algorithm constants
(`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) to the Node.js
napi-rs bindings, bringing them from 23/30 to 30/30 Tier 1 symbols.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-napi/src/lib.rs` — add 3 `#[napi]` functions + 4 `#[napi]` constants
    - `crates/iscc-napi/__tests__/functions.test.mjs` — add tests for the 7 new symbols
- **Reference**:
    - `crates/iscc-py/src/lib.rs` lines 395–433 — Python wrappers for the same 3 functions
    - `crates/iscc-lib/src/lib.rs` lines 170–267 — Rust core API signatures
    - `tests/test_new_symbols.py` — Python test patterns for encode_component, iscc_decode,
        json_to_data_url (roundtrips, error cases, etc.)

## Not In Scope

- Issue #5 dict meta propagation (accepting `object` for `meta` in `gen_meta_code_v0`) — separate
    step after all 7 symbols are propagated to all bindings
- TypeScript `const enum` wrappers for MT/ST/VS — not required by the target (Python has IntEnums as
    a drop-in extension; other bindings use raw `u8` integers per spec)
- Propagation to WASM, C FFI, Java, or Go — each gets its own step
- Changing `gen_*_v0` return types from `String` to structured objects — out of scope

## Implementation Notes

### Constants

napi-rs v2 supports `#[napi]` on `pub const`. Since `iscc_lib` constants are `usize`, cast to `u32`
for JavaScript compatibility:

```rust
#[napi(js_name = "META_TRIM_NAME")]
pub const META_TRIM_NAME: u32 = iscc_lib::META_TRIM_NAME as u32;
```

If `#[napi]` on const doesn't work (napi-rs version issue), fall back to exporting them as
module-level getter functions:

```rust
#[napi(js_name = "META_TRIM_NAME")]
pub fn meta_trim_name() -> u32 { iscc_lib::META_TRIM_NAME as u32 }
```

### encode_component

Thin wrapper following the established napi pattern. `digest` is `Buffer`:

```rust
#[napi(js_name = "encode_component")]
pub fn encode_component(
    mtype: u8, stype: u8, version: u8, bit_length: u32, digest: Buffer,
) -> napi::Result<String> {
    iscc_lib::encode_component(mtype, stype, version, bit_length, digest.as_ref())
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
```

### iscc_decode

Returns a JS object (not a tuple — JavaScript has no tuples). Use `#[napi(object)]` struct:

```rust
#[napi(object)]
pub struct IsccDecodeResult {
    pub maintype: u8,
    pub subtype: u8,
    pub version: u8,
    pub length: u8,
    pub digest: Buffer,
}
```

The function destructures the Rust tuple into this struct. Field names match the Python tuple
semantics: `maintype`, `subtype`, `version`, `length` (the length_index), `digest`.

### json_to_data_url

Simple pass-through, same as Python wrapper. Takes `String` (not `&str` — napi-rs convention):

```rust
#[napi(js_name = "json_to_data_url")]
pub fn json_to_data_url(json: String) -> napi::Result<String> {
    iscc_lib::json_to_data_url(&json)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
```

### Tests

Add to `functions.test.mjs` following the existing `describe`/`it` pattern with `node:test` +
`node:assert`. Import the 7 new symbols. Test categories:

1. **Constants**: verify values match (128, 4096, 4194304, 13), verify `typeof` is `'number'`
2. **encode_component**: roundtrip (encode then decompose to verify), error on invalid mtype (255),
    error on digest too short, error on ISCC mtype (5)
3. **iscc_decode**: roundtrip with encode_component, returned object has correct fields (`maintype`,
    `subtype`, `version`, `length`, `digest`), `digest` is a Buffer, error on invalid input string
4. **json_to_data_url**: plain JSON → `data:application/json;base64,...`, JSON-LD with `@context` →
    `data:application/ld+json;base64,...`, error on invalid JSON string

### Build regeneration

After modifying `lib.rs`, run `napi build --platform --release` (from `crates/iscc-napi/`) to
regenerate `index.js` and `index.d.ts`. These are gitignored build artifacts but needed for tests.
The advance agent must build before testing.

## Verification

- `cd crates/iscc-napi && npm test` passes (103 existing + new tests, 0 failures)
- `node -e "const m = require('./crates/iscc-napi'); console.log([m.META_TRIM_NAME, m.META_TRIM_DESCRIPTION, m.IO_READ_SIZE, m.TEXT_NGRAM_SIZE].join(','))"`
    prints `128,4096,4194304,13`
- `node -e "const m = require('./crates/iscc-napi'); console.log(typeof m.encode_component)"` prints
    `function`
- `node -e "const m = require('./crates/iscc-napi'); console.log(typeof m.iscc_decode)"` prints
    `function`
- `node -e "const m = require('./crates/iscc-napi'); console.log(typeof m.json_to_data_url)"` prints
    `function`
- `cargo clippy -p iscc-napi --all-targets -- -D warnings` is clean

## Done When

All verification criteria pass — 30/30 Tier 1 symbols are accessible from Node.js with tests
covering the 7 new symbols.
