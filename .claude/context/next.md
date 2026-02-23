# Next Work Package

## Step: Add DataHasher/InstanceHasher streaming classes to WASM bindings

## Goal

Add the final 2 Tier 1 symbols (`DataHasher`, `InstanceHasher`) to the WASM bindings, bringing them
from 21/23 to 23/23 — full Tier 1 parity with Python and Node.js. WASM is the natural next target
because `#[wasm_bindgen]` supports class methods similarly to napi-rs.

## Scope

- **Modify**: `crates/iscc-wasm/src/lib.rs` — add `WasmDataHasher` and `WasmInstanceHasher` structs
    with `#[wasm_bindgen]` constructor, `update`, and `finalize` methods
- **Modify**: `crates/iscc-wasm/tests/unit.rs` — add test cases for both streaming classes
- **Reference**: `crates/iscc-lib/src/streaming.rs` (Rust core `DataHasher`/`InstanceHasher` API),
    `crates/iscc-napi/src/lib.rs` (Node.js binding pattern — structurally identical to what WASM
    needs), `crates/iscc-wasm/tests/conformance.rs` (existing WASM test patterns)

## Not In Scope

- Adding streaming hashers to C FFI bindings — separate step (requires opaque pointer lifecycle
    pattern which is more involved)
- Structured return types for gen functions (returning full result objects instead of `.iscc`
    strings) — tracked separately
- Changes to the Rust core crate — `DataHasher`/`InstanceHasher` are already fully implemented and
    tested
- Adding per-crate CLAUDE.md files or other documentation — stay within defined scope

## Implementation Notes

### Class pattern

Follow the napi-rs `Option<Inner>` pattern adapted for wasm-bindgen. Each class wraps the core Rust
type in `Option<T>` to enforce finalize-once semantics:

```rust
#[wasm_bindgen]
pub struct DataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

#[wasm_bindgen]
impl DataHasher {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { inner: Some(iscc_lib::DataHasher::new()) }
    }

    pub fn update(&mut self, data: &[u8]) -> Result<(), JsError> {
        self.inner
            .as_mut()
            .ok_or_else(|| JsError::new("DataHasher already finalized"))
            .map(|h| h.update(data))
    }

    pub fn finalize(&mut self, bits: Option<u32>) -> Result<String, JsError> {
        let hasher = self.inner
            .take()
            .ok_or_else(|| JsError::new("DataHasher already finalized"))?;
        hasher.finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
```

Key wasm-bindgen differences from napi-rs:

- Use `#[wasm_bindgen(constructor)]` for the JS constructor (not `#[napi(constructor)]`)
- Unlike napi-rs, wasm-bindgen has no `ObjectFinalize` trait conflict — the method can be named
    `finalize` directly without `js_name` renaming
- `&[u8]` for `update()` input (wasm-bindgen supports references directly, unlike napi-rs `Buffer`)
- Error type is `JsError` (not `napi::Error`)
- WASM structs can use the same name as the core types (`DataHasher` not `WasmDataHasher`) since
    they're in a separate crate. But if clippy or naming conflict arises, prefix with `Wasm`

### InstanceHasher

Same pattern as DataHasher. `finalize` returns just the `.iscc` string, consistent with how
`gen_instance_code_v0` is bound in the current WASM bindings.

### Test cases (unit.rs)

Add tests using `wasm_bindgen_test` macro matching the existing test style in `unit.rs`:

1. **basic usage** — construct, update with data, finalize → returns valid ISCC string starting with
    `"ISCC:"`
2. **matches gen function** — verify streaming result matches one-shot `gen_data_code_v0` /
    `gen_instance_code_v0` for same input
3. **multi-update** — split data across multiple `update()` calls, verify same result as one-shot
4. **empty data** — construct and immediately finalize → produces valid ISCC
5. **finalize-once** — calling `finalize()` twice should error (wasm-bindgen test panics can be
    checked with `std::panic::catch_unwind` or by testing the `Result` return)
6. **default bits** — `finalize()` with `None` uses 64-bit default

Note: WASM tests use `wasm_bindgen_test` and run via `wasm-pack test --node`. The tests import from
`iscc_wasm::` (the crate) directly, not through JS imports. The `finalize()` method returns
`Result<String, JsError>` so error cases can be checked via `.is_err()` in Rust tests.

### Important: wasm-bindgen finalize naming

Check whether `finalize` conflicts with JavaScript's `FinalizationRegistry` or any wasm-bindgen
internal. It should be fine because `finalize` is just a regular method name in wasm-bindgen (not a
lifecycle hook like in napi-rs). If there IS a conflict, use `#[wasm_bindgen(js_name = "finalize")]`
on a differently-named Rust method.

## Verification

- `cargo build -p iscc-wasm --target wasm32-unknown-unknown` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test --workspace` passes with 280+ tests (no regressions to non-WASM crates)
- `wasm-pack test --node crates/iscc-wasm` passes all tests (31 existing + ~10 new)

## Done When

All verification criteria pass: both `DataHasher` and `InstanceHasher` classes are exported from the
WASM module, work correctly with the `new() → update() → finalize()` pattern, produce results
matching the one-shot gen functions, and all existing + new tests pass.
