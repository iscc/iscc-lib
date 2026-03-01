# Next Work Package

## Step: Add gen_sum_code_v0 to WASM bindings

## Goal

Propagate `gen_sum_code_v0` to the WASM binding crate (`crates/iscc-wasm/`), making it 32/32 Tier 1
symbols in WASM. WASM has no filesystem access, so the function accepts `Uint8Array` bytes directly
and composes the ISCC-SUM internally.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-wasm/src/lib.rs` — add `WasmSumCodeResult` struct and `gen_sum_code_v0` function
    - `crates/iscc-wasm/tests/unit.rs` — add unit tests for `gen_sum_code_v0`
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 960–998 — Rust core `gen_sum_code_v0` implementation
    - `crates/iscc-lib/src/types.rs` — `SumCodeResult` struct definition
    - `crates/iscc-wasm/src/lib.rs` lines 240–269 — `IsccDecodeResult` struct pattern for wasm_bindgen
        structs with multiple fields

## Not In Scope

- Adding `gen_sum_code_v0` to C FFI, Java, or Go bindings (separate future steps)
- Adding WASM conformance tests for `gen_sum_code_v0` (no conformance vectors exist for this
    function — it's tested via equivalence with `gen_data_code_v0` + `gen_instance_code_v0`)
- Updating documentation, READMEs, or docs site (deferred until all bindings have the function)
- Adding a bytes-based variant to the Rust core API (the WASM wrapper composes internally)

## Implementation Notes

**Struct — `WasmSumCodeResult`:**

Use `#[wasm_bindgen(getter_with_clone)]` (same pattern as `IsccDecodeResult` at line 240):

```rust
#[wasm_bindgen(getter_with_clone)]
pub struct WasmSumCodeResult {
    pub iscc: String,
    pub datahash: String,
    pub filesize: u64,
}
```

Note: `wasm_bindgen` supports `u64` (unlike napi-rs), so use `u64` directly for `filesize`.

**Function — `gen_sum_code_v0`:**

Since WASM has no filesystem, accept `data: &[u8]` instead of a file path. Replicate the core logic
without file I/O:

```rust
#[wasm_bindgen]
pub fn gen_sum_code_v0(
    data: &[u8],
    bits: Option<u32>,
    wide: Option<bool>,
) -> Result<WasmSumCodeResult, JsError> {
    let bits = bits.unwrap_or(64);
    let wide = wide.unwrap_or(false);

    let mut data_hasher = iscc_lib::DataHasher::new();
    let mut instance_hasher = iscc_lib::InstanceHasher::new();

    data_hasher.update(data);
    instance_hasher.update(data);

    let data_result = data_hasher.finalize(bits).map_err(|e| JsError::new(&e.to_string()))?;
    let instance_result = instance_hasher.finalize(bits).map_err(|e| JsError::new(&e.to_string()))?;

    let iscc_result = iscc_lib::gen_iscc_code_v0(
        &[&data_result.iscc, &instance_result.iscc],
        wide,
    ).map_err(|e| JsError::new(&e.to_string()))?;

    Ok(WasmSumCodeResult {
        iscc: iscc_result.iscc,
        datahash: instance_result.datahash,
        filesize: instance_result.filesize,
    })
}
```

Place the function right after `gen_iscc_code_v0` (around line 161), and the struct right after
`IsccDecodeResult` (around line 252).

**Tests** (add to `tests/unit.rs`):

1. **Equivalence test**: Feed same bytes to `gen_sum_code_v0` and separately to
    `gen_data_code_v0`/`gen_instance_code_v0` + `gen_iscc_code_v0`. Verify `iscc` matches.
2. **Result shape**: Verify `iscc` starts with `"ISCC:"`, `datahash` starts with `"1e20"`,
    `filesize` equals input length.
3. **Empty input**: Verify `gen_sum_code_v0(&[], None, None)` succeeds and matches empty-data
    equivalents.
4. **Default params**: Verify `None` bits/wide produce same as explicit `Some(64)`/`Some(false)`.
5. **Wide mode**: With `bits=128`, verify wide and non-wide produce different `iscc` values but same
    `datahash` and `filesize`.
6. **Filesize**: Verify `filesize` equals `data.len() as u64` for known input.

## Verification

- `cargo build -p iscc-wasm --target wasm32-unknown-unknown` compiles without errors
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `wasm-pack test --node crates/iscc-wasm` passes (70 existing + 6 new tests = 76 total)
- `gen_sum_code_v0` is exported and returns `WasmSumCodeResult` with `iscc`, `datahash`, `filesize`

## Done When

All four verification criteria pass with `gen_sum_code_v0` fully functional in the WASM binding.
