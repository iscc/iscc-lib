# Next Work Package

## Step: Add gen_sum_code_v0 to WASM bindings

## Goal

Propagate `gen_sum_code_v0` to the WASM binding crate (`crates/iscc-wasm/`), completing 32/32 Tier 1
symbols in WASM. Since WASM has no filesystem access, the function accepts `Uint8Array` bytes
instead of a file path and composes the ISCC-SUM internally. This advances issue #15.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-wasm/src/lib.rs` — add `WasmSumCodeResult` struct + `gen_sum_code_v0` function
    - `crates/iscc-wasm/tests/unit.rs` — add tests for `gen_sum_code_v0`
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines ~960-998 — core `gen_sum_code_v0` logic (single-pass file I/O
        feeding DataHasher + InstanceHasher, then gen_iscc_code_v0 composition)
    - `crates/iscc-lib/src/types.rs` — `SumCodeResult` struct definition
    - `crates/iscc-lib/src/streaming.rs` — `DataHasher` and `InstanceHasher` API
    - `crates/iscc-wasm/src/lib.rs` lines ~240-269 — `IsccDecodeResult` pattern for structured
        wasm_bindgen returns with `#[wasm_bindgen(getter_with_clone)]`

## Not In Scope

- Adding `gen_sum_code_v0` to C FFI, Java, or Go bindings (separate future steps)
- Conformance test vectors for gen_sum_code_v0 (none exist in data.json — test via equivalence)
- Updating documentation, READMEs, or docs site (defer until all bindings complete)
- Adding a bytes-based variant to the Rust core API (the WASM wrapper composes internally)
- Changing the return type of existing WASM functions to return structured results

## Implementation Notes

**Return type — `WasmSumCodeResult`:**

Use `#[wasm_bindgen(getter_with_clone)]` (same pattern as `IsccDecodeResult`):

```rust
#[wasm_bindgen(getter_with_clone)]
pub struct WasmSumCodeResult {
    pub iscc: String,
    pub datahash: String,
    pub filesize: f64,
}
```

**Why `f64` for filesize:** In wasm-bindgen, `u64` maps to JS `BigInt`, which causes friction for
web developers (can't mix with regular numbers, `JSON.stringify` fails, arithmetic requires explicit
conversion). `f64` maps naturally to JS `number` and handles files up to 2^53 bytes (~9 PB) — far
beyond any browser WASM use case. This matches the pragmatic approach for web consumers.

**Function — `gen_sum_code_v0`:**

Since WASM has no filesystem, accept `data: &[u8]` (maps to `Uint8Array` in JS). Replicate the core
logic without file I/O by composing from streaming hashers:

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
        filesize: instance_result.filesize as f64,
    })
}
```

**Placement:** Insert `WasmSumCodeResult` struct near `IsccDecodeResult` (~line 240). Insert
`gen_sum_code_v0` function after `gen_iscc_code_v0` (~line 161).

**Tests** (add to `tests/unit.rs`, minimum 4, target 6):

1. **Equivalence**: Feed same bytes to `gen_sum_code_v0` and separately to `gen_data_code_v0` +
    `gen_instance_code_v0` + `gen_iscc_code_v0`. Verify `iscc` matches.
2. **Result shape**: Verify `iscc` starts with `"ISCC:"`, `datahash` starts with `"1e20"`,
    `filesize` equals input length.
3. **Empty input**: Verify `gen_sum_code_v0(&[], None, None)` succeeds and matches empty-data
    equivalents.
4. **Default params**: Verify `None` bits/wide produce same as explicit `Some(64)`/`Some(false)`.
5. **Wide mode**: With `bits=128`, verify wide and non-wide produce different `iscc` values but same
    `datahash` and `filesize`.
6. **Filesize**: Verify `filesize` equals `data.len() as f64` for known test input.

## Verification

- `cargo build -p iscc-wasm --target wasm32-unknown-unknown` compiles without errors
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `wasm-pack test --node crates/iscc-wasm` passes (existing + 4-6 new gen_sum_code_v0 tests)
- `gen_sum_code_v0` is exported and returns `WasmSumCodeResult` with `iscc`, `datahash`, `filesize`
- `mise run check` passes (all pre-commit hooks clean)

## Done When

All five verification criteria pass — `gen_sum_code_v0` is fully functional in the WASM binding with
structured result return and at least 4 new tests.
