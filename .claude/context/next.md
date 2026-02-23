# Next Work Package

## Step: Add DataHasher/InstanceHasher streaming classes to Node.js bindings

## Goal

Add the final 2 Tier 1 symbols (`DataHasher`, `InstanceHasher`) to the Node.js bindings, bringing
them from 21/23 to 23/23 — full Tier 1 parity with Python. Node.js is the easiest starting point for
streaming classes because napi-rs has native `#[napi]` class support with methods.

## Scope

- **Modify**: `crates/iscc-napi/src/lib.rs` — add `NapiDataHasher` and `NapiInstanceHasher` classes
    with `#[napi]` constructor, `update`, and `finalize` methods
- **Modify**: `crates/iscc-napi/__tests__/functions.test.mjs` — add test cases for both streaming
    classes
- **Reference**: `crates/iscc-lib/src/streaming.rs` (Rust core `DataHasher`/`InstanceHasher` API),
    `crates/iscc-py/src/lib.rs` (Python binding pattern with `Option<Inner>` for finalize-once
    semantics), `crates/iscc-napi/__tests__/conformance.test.mjs` (test data loading pattern)

## Not In Scope

- Adding streaming hashers to WASM or C FFI bindings — separate steps after Node.js
- Structured return types for gen functions (returning full result objects instead of `.iscc`
    strings) — tracked separately
- Streaming conformance tests in the Node.js conformance test file — unit tests in
    `functions.test.mjs` are sufficient for this step (conformance is already verified in Rust core)
- Changes to the Rust core crate — `DataHasher`/`InstanceHasher` are already fully implemented and
    tested

## Implementation Notes

### Class pattern

Follow the PyO3 `Option<Inner>` pattern adapted for napi-rs. Each class wraps the core Rust type in
`Option<T>` to enforce finalize-once semantics:

```rust
#[napi(js_name = "DataHasher")]
struct NapiDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}

#[napi]
impl NapiDataHasher {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self { inner: Some(iscc_lib::DataHasher::new()) }
    }

    #[napi]
    pub fn update(&mut self, data: Buffer) -> napi::Result<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| napi::Error::from_reason("DataHasher already finalized"))
            .map(|h| h.update(&data))
    }

    #[napi(js_name = "finalize")]
    pub fn finalize_code(&mut self, bits: Option<u32>) -> napi::Result<String> {
        let hasher = self.inner
            .take()
            .ok_or_else(|| napi::Error::from_reason("DataHasher already finalized"))?;
        hasher.finalize(bits.unwrap_or(64))
            .map(|r| r.iscc)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }
}
```

Key napi-rs class notes:

- Use `#[napi(constructor)]` (not `#[new]` like PyO3) for the JS constructor
- The `finalize` name may conflict with napi-rs `ObjectFinalize` trait — name the Rust method
    `finalize_code` with `#[napi(js_name = "finalize")]` to expose it as `finalize` in JS
- `Buffer` input for `update()` (napi-rs type, accepts both `Buffer` and `Uint8Array` from JS)
- Return `String` (just the `.iscc` field) from `finalize`, matching the existing gen function
    pattern in Node.js bindings
- `bits` parameter as `Option<u32>` with `.unwrap_or(64)` default — napi-rs doesn't support native
    defaults

### InstanceHasher

Same pattern as DataHasher. `finalize` returns just the `.iscc` string (not `datahash`/`filesize`),
consistent with how `gen_instance_code_v0` is bound in the current Node.js bindings.

### Test cases (functions.test.mjs)

Add a `describe('DataHasher')` and `describe('InstanceHasher')` block with these tests:

1. **basic usage** — construct, update with data, finalize → returns valid ISCC string starting with
    `"ISCC:"`
2. **matches gen function** — verify streaming result matches one-shot `gen_data_code_v0` /
    `gen_instance_code_v0` for same input
3. **multi-update** — split data across multiple `update()` calls, verify same result as one-shot
4. **empty data** — construct and immediately finalize → produces valid ISCC
5. **finalize-once** — calling `update()` after `finalize()` throws; calling `finalize()` twice
    throws
6. **default bits** — `finalize()` without argument uses 64-bit default

### Import

The test file needs to import `DataHasher` and `InstanceHasher` alongside existing function imports.
napi-rs classes are exported as constructors from the native module.

## Verification

- `cargo build -p iscc-napi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test --workspace` passes with 280+ tests (no regressions)
- `node --test crates/iscc-napi/__tests__/functions.test.mjs` passes all tests (existing + ~12 new)
- `node -e "const m = require('./crates/iscc-napi'); const d = new m.DataHasher(); d.update(Buffer.from('hello')); console.log(d.finalize())"`
    prints an ISCC string

## Done When

All verification criteria pass: both `DataHasher` and `InstanceHasher` classes are importable from
the Node.js native module, work correctly with the `new() → update() → finalize()` pattern, and all
existing + new tests pass.
