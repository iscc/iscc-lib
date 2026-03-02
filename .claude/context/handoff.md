## 2026-03-02 — Expose `add_units`/`units` in WASM binding

**Done:** Added `add_units: Option<bool>` parameter to `gen_sum_code_v0` and
`units: Option<Vec<String>>` field to `WasmSumCodeResult` in the WASM binding. When `add_units` is
`true`, the result includes individual Data-Code and Instance-Code ISCC strings. Updated all 6
existing test call sites with the 4th `None` argument and added 3 new tests for units
enabled/disabled/content verification.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added `units: Option<Vec<String>>` to `WasmSumCodeResult` struct;
    added `add_units: Option<bool>` parameter to `gen_sum_code_v0` with `unwrap_or(false)` default;
    conditionally populates units after `gen_iscc_code_v0` borrow (borrow-before-move pattern)
- `crates/iscc-wasm/tests/unit.rs`: Updated 6 existing `gen_sum_code_v0` test calls from 3 args to 4
    (adding `None`); added 3 new tests: `test_gen_sum_code_v0_units_enabled`,
    `test_gen_sum_code_v0_units_disabled`, `test_gen_sum_code_v0_units_content`

**Verification:** All quality gates pass. `cargo clippy -p iscc-wasm -- -D warnings` clean.
`wasm-pack test --node` passes 78 tests (9 conformance + 69 unit). `mise run check` passes 14/14
hooks. Full workspace `cargo test` passes 258 core + 82 FFI + 53 integration tests.

**Next:** Continue binding propagation for issue #21. Remaining bindings: C FFI (`crates/iscc-ffi`),
JNI (`crates/iscc-jni`), Go (`packages/go`). C FFI is a good next target — it requires `add_units`
as a C `bool` parameter and returning `units` as a string array via the FFI bridge.

**Notes:** The WASM implementation follows the same borrow-before-move pattern as the core
`gen_sum_code_v0` — `gen_iscc_code_v0` borrows `&data_result.iscc` and `&instance_result.iscc`, then
they're moved into the `units` vec afterward (no clone needed). The `Option<Vec<String>>` with
`#[wasm_bindgen(getter_with_clone)]` maps to `string[] | undefined` in TypeScript, matching the NAPI
pattern.
