## 2026-03-02 — Review of: Expose `add_units`/`units` in WASM binding

**Verdict:** PASS

**Summary:** Clean, minimal implementation that adds `add_units: Option<bool>` parameter and
`units: Option<Vec<String>>` field to the WASM binding's `gen_sum_code_v0`. The borrow-before-move
pattern is correctly applied — `gen_iscc_code_v0` borrows the ISCC strings, then they're moved into
the units vec. All quality gates pass, all tests green.

**Verification:**

- [x] `cargo clippy -p iscc-wasm -- -D warnings` clean — no warnings
- [x] `wasm-pack test --node` passes — 69 unit tests (70 with conformance feature, 79 total with
    conformance.rs)
- [x] `gen_sum_code_v0(data, None, None, Some(true))` returns `WasmSumCodeResult` with `units`
    containing exactly 2 ISCC strings — verified via `test_gen_sum_code_v0_units_enabled`
- [x] `gen_sum_code_v0(data, None, None, None)` returns `WasmSumCodeResult` with `units == None` —
    verified via `test_gen_sum_code_v0_units_disabled`
- [x] All 6 existing `test_gen_sum_code_v0_*` tests still pass (updated call sites with 4th `None`)
- [x] `mise run check` passes 14/14 hooks

**Issues found:**

- (none)

**Next:** Continue issue #21 binding propagation. Next target is C FFI (`crates/iscc-ffi`): add
`add_units` as a C `bool` (or `uint8_t`) parameter to `iscc_gen_sum_code_v0`, extend
`IsccSumCodeResult` with a units string array, regenerate the C header, and update tests. After C
FFI, remaining bindings are Java/JNI and Go.

**Notes:** Issue #21 progress: Rust core ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI (next) → JNI →
Go. The handoff claimed 78 total tests but actual count is 79 (9 conformance + 70 unit including 1
conformance_selftest behind feature gate). Minor counting discrepancy, no quality concern. Test
count in next.md's "75 existing" was slightly off (actual was 76 with conformance).
