# Next Work Package

## Step: Expose `add_units`/`units` in WASM binding

## Goal

Add `addUnits` parameter and `units` field to the WASM binding's `gen_sum_code_v0`, continuing issue
#21 propagation. After this, WASM callers can request individual Data-Code and Instance-Code ISCC
strings alongside the composite ISCC-CODE.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-wasm/src/lib.rs` — add `add_units: Option<bool>` param to `gen_sum_code_v0`; add
        `units: Option<Vec<String>>` field to `WasmSumCodeResult`; conditionally populate `units` when
        `add_units` is `true`
    - `crates/iscc-wasm/tests/unit.rs` — add 3 new tests: units enabled returns 2-element array, units
        disabled (default) returns `None`, content verification of unit strings
- **Reference**:
    - `crates/iscc-napi/src/lib.rs` — Node.js binding pattern for `add_units` (completed last
        iteration)
    - `crates/iscc-lib/src/code_sum.rs` — Rust core `gen_sum_code_v0` for units logic reference

## Not In Scope

- Updating docs pages (`docs/rust-api.md`, `docs/architecture.md`) — deferred until all 4 remaining
    bindings are updated
- C FFI, JNI, or Go binding changes — those are separate subsequent steps
- Refactoring the WASM inline implementation to delegate to `iscc_lib::gen_sum_code_v0` — WASM
    operates on `&[u8]` (no filesystem), so the inline approach is correct

## Implementation Notes

The WASM binding has its own inline `gen_sum_code_v0` (lines 180-209 of `lib.rs`) that uses
`DataHasher`/`InstanceHasher` directly instead of calling `iscc_lib::gen_sum_code_v0`. This is
because WASM has no filesystem access — it takes `&[u8]` instead of `&Path`.

**Changes to `WasmSumCodeResult` struct** (line ~165):

```rust
pub struct WasmSumCodeResult {
    pub iscc: String,
    pub datahash: String,
    pub filesize: f64,
    pub units: Option<Vec<String>>,  // NEW
}
```

The `Option<Vec<String>>` with `#[wasm_bindgen(getter_with_clone)]` will appear as
`string[] | undefined` in TypeScript — same pattern as NAPI's `Option<Vec<String>>`.

**Changes to `gen_sum_code_v0` function** (line ~180):

1. Add 4th parameter: `add_units: Option<bool>`
2. Default to `false` via `unwrap_or(false)`
3. When `add_units` is true, capture `data_result.iscc` and `instance_result.iscc` into
    `Some(vec![data_iscc, instance_iscc])` BEFORE passing them to `gen_iscc_code_v0`
4. Important: the borrow-before-move pattern — clone or capture the ISCC strings before they're
    borrowed by `gen_iscc_code_v0`. Since WASM already borrows `&data_result.iscc` and
    `&instance_result.iscc` for the `gen_iscc_code_v0` call, you can clone into the units vec after
    that call, or capture refs beforehand

**Test patterns** (in `tests/unit.rs`):

- `test_gen_sum_code_v0_units_enabled`: call with `add_units=Some(true)`, assert `units` is
    `Some(vec)` with exactly 2 elements, each starting with `"ISCC:"`
- `test_gen_sum_code_v0_units_disabled`: call with `add_units=None` (default), assert
    `units.is_none()`
- `test_gen_sum_code_v0_units_content`: call with `add_units=Some(true)`, separately compute
    `gen_data_code_v0` and `gen_instance_code_v0` on same data, assert `units[0]` matches data code
    and `units[1]` matches instance code

**Existing test updates**: The 6 existing `test_gen_sum_code_v0_*` tests pass `None` for the first 3
args (bits, wide). They will need a 4th `None` arg (add_units). Update all call sites:
`gen_sum_code_v0(data, bits, wide, None)`.

## Verification

- `cd crates/iscc-wasm && cargo clippy -p iscc-wasm -- -D warnings` clean
- `cd crates/iscc-wasm && wasm-pack test --node` passes (75 existing + 3 new = 78 tests)
- `gen_sum_code_v0(data, None, None, Some(true))` returns `WasmSumCodeResult` with `units`
    containing exactly 2 ISCC strings
- `gen_sum_code_v0(data, None, None, None)` returns `WasmSumCodeResult` with `units == None`
- All 6 existing `test_gen_sum_code_v0_*` tests still pass (updated call sites)

## Done When

All verification criteria pass: clippy clean, 78 wasm-bindgen tests pass, units field correctly
populated when `add_units=true` and `None` when omitted/false.
