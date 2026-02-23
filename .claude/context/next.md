# Next Work Package

## Step: Add text utils and simple functions to Node.js bindings

## Goal

Expand Node.js bindings from 9 gen functions to 17 Tier 1 symbols by adding the 8 simplest non-class
functions: 4 text utilities, `encode_base64`, `iscc_decompose`, `conformance_selftest`, and
`sliding_window`. These all have straightforward napi-rs type mappings and follow the established
thin-wrapper pattern.

## Scope

- **Modify**: `crates/iscc-napi/src/lib.rs` — add 8 `#[napi]` function wrappers
- **Create**: `crates/iscc-napi/__tests__/functions.test.mjs` — unit tests for the 8 new functions
- **Reference**: `crates/iscc-napi/src/lib.rs` — existing 9 gen wrappers (pattern to follow)
- **Reference**: `crates/iscc-py/src/lib.rs` — Python equivalents for semantics/signatures
- **Reference**: `crates/iscc-lib/src/lib.rs` — Rust core API signatures

## Not In Scope

- Structured/object returns for the 9 gen functions (they return strings now; dict returns are a
    separate step)
- Algorithm primitives with complex types (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`) — these need `Buffer`/`Vec<Buffer>` mappings and belong in the next step
- Streaming hashers (`DataHasher`, `InstanceHasher`) — these require `#[napi]` class support and
    belong in a later step
- Updating `package.json` test script to run multiple test files (the `node --test` glob pattern
    `__tests__/*.test.mjs` is fine; update only if needed)
- WASM or C FFI binding expansion

## Implementation Notes

### napi-rs patterns (from learnings)

- Use owned `String` (not `&str`) for string parameters
- Use `Buffer` (not `&[u8]`) for byte parameters
- Use `Option<T>` with `.unwrap_or()` for default parameter values (napi has no native defaults)
- Use `#[napi(js_name = "snake_case")]` on every function to prevent auto-camelCase conversion
- Error mapping: `napi::Error::from_reason(e.to_string())`

### Functions to add (8 total)

1. **`text_clean`**: `String → String` — trivial passthrough

    ```rust
    #[napi(js_name = "text_clean")]
    pub fn text_clean(text: String) -> String {
        iscc_lib::text_clean(&text)
    }
    ```

2. **`text_remove_newlines`**: `String → String` — same pattern

3. **`text_trim`**: `String, u32 → String` — note `nbytes` is `usize` in Rust, use `u32` in napi

    ```rust
    #[napi(js_name = "text_trim")]
    pub fn text_trim(text: String, nbytes: u32) -> String {
        iscc_lib::text_trim(&text, nbytes as usize)
    }
    ```

4. **`text_collapse`**: `String → String` — same as text_clean

5. **`encode_base64`**: `Buffer → String`

    ```rust
    #[napi(js_name = "encode_base64")]
    pub fn encode_base64(data: Buffer) -> String {
        iscc_lib::encode_base64(data.as_ref())
    }
    ```

6. **`iscc_decompose`**: `String → napi::Result<Vec<String>>`

    ```rust
    #[napi(js_name = "iscc_decompose")]
    pub fn iscc_decompose(iscc_code: String) -> napi::Result<Vec<String>> {
        iscc_lib::iscc_decompose(&iscc_code)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
    ```

7. **`conformance_selftest`**: `() → bool` — simplest possible wrapper

    ```rust
    #[napi(js_name = "conformance_selftest")]
    pub fn conformance_selftest() -> bool {
        iscc_lib::conformance_selftest()
    }
    ```

8. **`sliding_window`**: `String, u32 → napi::Result<Vec<String>>` — pre-validate width < 2

    ```rust
    #[napi(js_name = "sliding_window")]
    pub fn sliding_window(seq: String, width: u32) -> napi::Result<Vec<String>> {
        if width < 2 {
            return Err(napi::Error::from_reason(
                "Sliding window width must be 2 or bigger.",
            ));
        }
        Ok(iscc_lib::sliding_window(&seq, width as usize))
    }
    ```

### Tests (`functions.test.mjs`)

Use `node:test` + `node:assert` (same as conformance tests, zero dependencies). Import from
`../index.js`. Test cases:

- **text_clean**: NFKC normalization (e.g., `"Ⅷ"` → `"VIII"`), control char removal, newline
    normalization, empty string
- **text_remove_newlines**: multi-line → single line, consecutive spaces collapsed
- **text_trim**: truncation at byte boundary, multi-byte char not split, result trimmed
- **text_collapse**: lowercased, no whitespace/punctuation, empty string
- **encode_base64**: known input/output pair (e.g., `Buffer.from([0,1,2])` → known base64url string)
- **iscc_decompose**: decompose a known ISCC-CODE, error on invalid input
- **conformance_selftest**: returns `true`
- **sliding_window**: known n-grams, error on width < 2

### Build and test commands

```bash
# Build the napi addon
cd crates/iscc-napi && npm run build:debug && cd ../..

# Run all Node.js tests (both files)
node --test crates/iscc-napi/__tests__/*.test.mjs
```

### Update test script if needed

If `package.json` `test` script only runs `conformance.test.mjs`, update it to glob:
`"test": "node --test __tests__/*.test.mjs"`

## Verification

- `cargo build -p iscc-napi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `node --test crates/iscc-napi/__tests__/*.test.mjs` passes (existing 46 conformance + new unit
    tests)
- All 8 new functions are importable:
    `import { text_clean, text_remove_newlines, text_trim,   text_collapse, encode_base64, iscc_decompose, conformance_selftest, sliding_window } from   '../index.js'`
    works in test file
- `conformance_selftest()` returns `true` from Node.js

## Done When

All verification criteria pass and the Node.js bindings expose 17 Tier 1 symbols (9 existing gen
functions + 8 new functions) with unit tests for all new functions.
