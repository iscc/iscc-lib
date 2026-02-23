# Next Work Package

## Step: Add 8 simple functions to WASM bindings

## Goal

Expand the WASM bindings from 9 gen-only functions to 17 Tier 1 symbols by adding the same 8 simple
functions already present in Node.js: `text_clean`, `text_remove_newlines`, `text_trim`,
`text_collapse`, `encode_base64`, `iscc_decompose`, `conformance_selftest`, `sliding_window`. This
mirrors the Node.js expansion completed last iteration and is the lowest-risk binding expansion
step.

## Scope

- **Create**: `crates/iscc-wasm/tests/unit.rs` — unit tests for the 8 new functions
- **Modify**: `crates/iscc-wasm/src/lib.rs` — add 8 `#[wasm_bindgen]` functions
- **Reference**: `crates/iscc-napi/src/lib.rs` (Node.js equivalents of the same 8 functions),
    `crates/iscc-lib/src/utils.rs` (text utility signatures), `crates/iscc-lib/src/codec.rs`
    (`encode_base64`, `iscc_decompose` signatures), `crates/iscc-lib/src/conformance.rs`
    (`conformance_selftest` signature), `crates/iscc-lib/src/simhash.rs` (`sliding_window`
    signature)

## Not In Scope

- Algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`) — these require complex
    type mappings (`Vec<Vec<u8>>`, `JsValue` deserialization) and should be a separate step
- `soft_hash_video_v0` — has the same complex type issues as algorithm primitives
- `DataHasher`/`InstanceHasher` streaming classes — WASM class support via wasm-bindgen is a
    distinct pattern requiring its own step
- Structured return types (returning objects instead of strings) — deferred to a future step
- Changing existing gen function signatures or return types
- Node.js or C FFI binding expansion

## Implementation Notes

Follow the existing wasm-bindgen thin-wrapper pattern established by the 9 gen functions.

### wasm-bindgen patterns (from learnings)

- `&str` and `&[u8]` work directly in wasm-bindgen (unlike napi-rs which needs owned
    `String`/`Buffer`)
- Error mapping uses `JsError::new(&e.to_string())`
- `Vec<String>` is supported natively by wasm-bindgen (auto-converts to JS `Array<string>`)

### Functions to add (8 total)

Append after the existing `gen_iscc_code_v0`. Group: text utils → encoding → codec → conformance →
sliding_window.

1. **`text_clean`**: `&str → String` — direct passthrough, no Result

    ```rust
    #[wasm_bindgen]
    pub fn text_clean(text: &str) -> String {
        iscc_lib::text_clean(text)
    }
    ```

2. **`text_remove_newlines`**: `&str → String` — same pattern

3. **`text_trim`**: `&str, u32 → String` — `nbytes` is `usize` in Rust, use `u32` for WASM, cast

    ```rust
    #[wasm_bindgen]
    pub fn text_trim(text: &str, nbytes: u32) -> String {
        iscc_lib::text_trim(text, nbytes as usize)
    }
    ```

4. **`text_collapse`**: `&str → String` — same as text_clean

5. **`encode_base64`**: `&[u8] → String` — `&[u8]` works directly in wasm-bindgen

    ```rust
    #[wasm_bindgen]
    pub fn encode_base64(data: &[u8]) -> String {
        iscc_lib::encode_base64(data)
    }
    ```

6. **`iscc_decompose`**: `&str → Result<Vec<String>, JsError>`

    ```rust
    #[wasm_bindgen]
    pub fn iscc_decompose(iscc_code: &str) -> Result<Vec<String>, JsError> {
        iscc_lib::iscc_decompose(iscc_code)
            .map_err(|e| JsError::new(&e.to_string()))
    }
    ```

7. **`conformance_selftest`**: `() → bool`

    ```rust
    #[wasm_bindgen]
    pub fn conformance_selftest() -> bool {
        iscc_lib::conformance_selftest()
    }
    ```

8. **`sliding_window`**: `&str, u32 → Result<Vec<String>, JsError>` — pre-validate `width < 2`

    ```rust
    #[wasm_bindgen]
    pub fn sliding_window(seq: &str, width: u32) -> Result<Vec<String>, JsError> {
        if width < 2 {
            return Err(JsError::new("Sliding window width must be 2 or bigger."));
        }
        Ok(iscc_lib::sliding_window(seq, width as usize))
    }
    ```

### Tests (`tests/unit.rs`)

Create a separate test file to keep conformance tests focused on gen function vectors. Use
`#[wasm_bindgen_test]` for all tests. Test cases:

- **text_clean**: NFKC normalization (e.g., `"Ⅷ"` → `"VIII"`), control char removal, empty string
- **text_remove_newlines**: multi-line → single line, consecutive spaces collapsed
- **text_trim**: truncation at byte boundary, multi-byte char not split, result trimmed
- **text_collapse**: lowercased, no whitespace/punctuation, empty string
- **encode_base64**: known input/output pair (e.g., `&[0,1,2]` → known base64url string)
- **iscc_decompose**: decompose a known ISCC-CODE into units, error on invalid input
- **conformance_selftest**: returns `true`
- **sliding_window**: known n-grams, error on `width < 2`

### Build and test commands

```bash
# Build and test WASM
wasm-pack test --node crates/iscc-wasm
```

## Verification

- `cargo build -p iscc-wasm` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `wasm-pack test --node crates/iscc-wasm` passes (existing 9 conformance + new unit tests)
- All 8 new functions are present as `#[wasm_bindgen]` exports in `lib.rs`
- `conformance_selftest()` callable and returns `true` from WASM tests
- `cargo test --workspace` passes with 250+ tests (no regressions)

## Done When

All verification criteria pass: the 8 new wasm-bindgen functions compile, clippy is clean, and all
WASM tests (conformance + unit) pass via `wasm-pack test --node`.
