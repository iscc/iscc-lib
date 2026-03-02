# Next Work Package

## Step: Add `units` support to `gen_sum_code_v0` in Rust core (issue #21)

## Goal

Add an `add_units: bool` parameter to `gen_sum_code_v0` and a `units: Option<Vec<String>>` field to
`SumCodeResult`. When `add_units` is true, the result includes the individual Data-Code and
Instance-Code ISCC strings at the requested `bits` precision. This enables `iscc-sdk` to obtain all
three codes (ISCC-SUM, Data-Code, Instance-Code) from a single optimized file read instead of three
separate calls. This step implements the Rust core change and fixes all Rust-based binding call
sites to maintain compilation.

## Scope

- **Create**: none
- **Modify**:
    - `crates/iscc-lib/src/types.rs` — add `pub units: Option<Vec<String>>` field to `SumCodeResult`
    - `crates/iscc-lib/src/lib.rs` — add `add_units: bool` parameter to `gen_sum_code_v0`, populate
        `units` conditionally
    - `crates/iscc-py/src/lib.rs` — pass `false` for new `add_units` parameter (1-line fix)
    - `crates/iscc-napi/src/lib.rs` — pass `false` for new `add_units` parameter (1-line fix)
    - `crates/iscc-ffi/src/lib.rs` — pass `false` for new `add_units` parameter (1-line fix)
    - `crates/iscc-jni/src/lib.rs` — pass `false` for new `add_units` parameter (1-line fix)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 960-998 (current `gen_sum_code_v0` implementation)
    - `crates/iscc-lib/src/types.rs` lines 95-105 (current `SumCodeResult` definition)
    - `crates/iscc-lib/src/lib.rs` lines 2110-2247 (existing unit tests)
    - `crates/iscc-lib/benches/benchmarks.rs` lines 184-214 (benchmark calls)

## Not In Scope

- Exposing `add_units` parameter in any binding's public API (Python `__init__.py`, Node.js, WASM,
    JNI, FFI, Go). Bindings always pass `false` for now; API exposure is a separate step.
- Updating `WasmSumCodeResult` or WASM's inline `gen_sum_code_v0` (WASM has its own implementation
    that doesn't call `iscc_lib::gen_sum_code_v0` — needs separate scoping)
- Updating Go's pure Go implementation (separate step with its own logic)
- Documentation updates for the new parameter/field
- Adding `units` to C FFI's `IsccSumCodeResult` struct or header
- Changing any binding's result type to include units

## Implementation Notes

The current `gen_sum_code_v0` already computes `data_result.iscc` and `instance_result.iscc` (lines
988-989) and uses them to call `gen_iscc_code_v0` (line 991). These strings are then discarded. The
change is minimal:

1. **`types.rs`**: Add `pub units: Option<Vec<String>>` to `SumCodeResult`. Keep `#[non_exhaustive]`
    attribute. Add a doc comment explaining the field contains `[Data-Code, Instance-Code]` ISCC
    strings at the requested bit precision.

2. **`lib.rs`**: Add `add_units: bool` as the 4th parameter. In the result construction:

    ```rust
    Ok(SumCodeResult {
        iscc: iscc_result.iscc,
        datahash: instance_result.datahash,
        filesize: instance_result.filesize,
        units: if add_units {
            Some(vec![data_result.iscc, instance_result.iscc])
        } else {
            None
        },
    })
    ```

    Note: when `add_units` is false, `data_result.iscc` and `instance_result.iscc` are still computed
    (needed for `gen_iscc_code_v0`) but are dropped rather than cloned into the result. Since
    `data_result` and `instance_result` are consumed by value, use `data_result.iscc` directly
    (moved, not cloned) when `add_units` is true. Reorder the result construction so
    `gen_iscc_code_v0` borrows the strings before they're potentially moved.

3. **Binding call-site fixes**: Each of the 4 Rust binding crates has exactly one call to
    `iscc_lib::gen_sum_code_v0(path, bits, wide)`. Add `, false` as the 4th argument. These are
    mechanical 1-line changes to keep all crates compiling. Exact locations:

    - `crates/iscc-py/src/lib.rs:335` — `iscc_lib::gen_sum_code_v0(..., bits, wide)`
    - `crates/iscc-napi/src/lib.rs:232` — `iscc_lib::gen_sum_code_v0(..., bits, wide)`
    - `crates/iscc-ffi/src/lib.rs:847` — `iscc_lib::gen_sum_code_v0(..., bits, wide)`
    - `crates/iscc-jni/src/lib.rs:414` — `iscc_lib::gen_sum_code_v0(..., bits, wide)`

4. **Benchmarks**: `crates/iscc-lib/benches/benchmarks.rs` also calls `gen_sum_code_v0` — update
    those calls to pass `false` as the 4th argument.

5. **Tests**: Add 2-3 new tests in `lib.rs`:

    - `test_gen_sum_code_v0_units_enabled` — verify `units` is `Some` with exactly 2 elements, first
        starting with `"ISCC:"` and decoding to MainType::Data, second to MainType::Instance
    - `test_gen_sum_code_v0_units_disabled` — verify `units` is `None` when `add_units` is `false`
    - Update all 7 existing `test_gen_sum_code_v0_*` tests to pass `false` for `add_units`

## Verification

- `cargo test -p iscc-lib` passes (310 existing + 2-3 new tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo build -p iscc-py` compiles successfully
- `cargo build -p iscc-napi` compiles successfully
- `cargo build -p iscc-ffi` compiles successfully
- `cargo build -p iscc-jni` compiles successfully
- `cargo bench -p iscc-lib --no-run` compiles successfully

## Done When

All verification criteria pass: Rust core has `add_units` parameter with `units` field working
correctly, all binding crates compile, and new tests validate both enabled and disabled paths.
