## 2026-03-02 — Add `units` support to `gen_sum_code_v0` in Rust core (issue #21)

**Done:** Added `add_units: bool` parameter to `gen_sum_code_v0` and `units: Option<Vec<String>>`
field to `SumCodeResult`. When `add_units` is true, the result includes individual Data-Code and
Instance-Code ISCC strings. Updated all 4 Rust binding crates, 1 FFI test, and benchmarks to pass
`false` for the new parameter. Added 2 new tests validating both enabled and disabled paths.

**Files changed:**

- `crates/iscc-lib/src/types.rs`: Added `pub units: Option<Vec<String>>` field to `SumCodeResult`
    with doc comment
- `crates/iscc-lib/src/lib.rs`: Added `add_units: bool` as 4th parameter to `gen_sum_code_v0`,
    conditionally populates `units` field. Updated 7 existing tests to pass `false`, added 2 new
    tests (`test_gen_sum_code_v0_units_enabled`, `test_gen_sum_code_v0_units_disabled`)
- `crates/iscc-py/src/lib.rs`: Added `, false` to `iscc_lib::gen_sum_code_v0` call (line 335)
- `crates/iscc-napi/src/lib.rs`: Added `, false` to `iscc_lib::gen_sum_code_v0` call (line 232)
- `crates/iscc-ffi/src/lib.rs`: Added `, false` to two `iscc_lib::gen_sum_code_v0` calls (extern fn
    - test)
- `crates/iscc-jni/src/lib.rs`: Added `, false` to `iscc_lib::gen_sum_code_v0` call (line 414)
- `crates/iscc-lib/benches/benchmarks.rs`: Added `black_box(false)` 4th arg to both benchmark calls

**Verification:**

- `cargo test -p iscc-lib`: 312 tests pass (258 lib + 31 integration + 22 streaming + 1 doctest). 2
    new tests added (was 310)
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo build -p iscc-py`: compiles
- `cargo build -p iscc-napi`: compiles
- `cargo build -p iscc-ffi`: compiles
- `cargo build -p iscc-jni`: compiles
- `cargo bench -p iscc-lib --no-run`: compiles
- `mise run check`: 14/14 hooks pass

**Next:** Expose `add_units` parameter in binding APIs (Python first, then Node.js, C FFI, JNI).
WASM needs separate scoping since it has its own implementation that doesn't call
`iscc_lib::gen_sum_code_v0`. Go needs a pure Go implementation of the `units` feature.

**Notes:** The implementation borrows `data_result.iscc` and `instance_result.iscc` for
`gen_iscc_code_v0`, then moves them into the `units` vec when `add_units` is true. When `add_units`
is false, the strings are dropped (no clone needed). The `#[non_exhaustive]` attribute on
`SumCodeResult` ensures the new field doesn't break downstream match patterns.
