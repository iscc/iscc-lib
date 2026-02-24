## 2026-02-24 — Fix WASM `alg_cdc_chunks` silent null return

**Done:** Changed `alg_cdc_chunks` return type from `JsValue` to `Result<JsValue, JsError>` and
replaced `unwrap_or(JsValue::NULL)` with proper error propagation via `map_err`. Updated 3 test call
sites to `.unwrap()` the new `Result`.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Changed `alg_cdc_chunks` return type to `Result<JsValue, JsError>`,
    replaced silent null fallback with `map_err` error propagation
- `crates/iscc-wasm/tests/unit.rs`: Added `.unwrap()` to 3 `alg_cdc_chunks` call sites (lines 216,
    227, 236)

**Verification:**

- `wasm-pack test --node crates/iscc-wasm` passes all 54 tests (9 conformance + 45 unit, 0 failures)
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `grep 'unwrap_or(JsValue::NULL)' crates/iscc-wasm/src/lib.rs` returns 0 matches
- `grep 'Result<JsValue, JsError>' crates/iscc-wasm/src/lib.rs` returns 1 match
- `mise run check` passes all 14 pre-commit hooks

**Next:** The WASM silent null issue is resolved. Remaining normal-priority candidates: FFI video
frame copy, codec header parsing optimization, DataHasher allocation overhead. Low priority: alg_dct
power-of-two, alg_wtahash bounds check, JNI exception types, Python __version__, Python module
docstring, WASM conformance_selftest size, WASM stale CLAUDE.md, TypeScript port evaluation.

**Notes:** The test count is 54 (9 conformance + 45 unit), not 56 as estimated in next.md — the
estimate may have included doc-tests or counted differently. All tests pass. The fix is fully
mechanical — no behavioral change for successful serialization, errors now propagate as JS
exceptions instead of silently returning null.
