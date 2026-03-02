## 2026-03-02 — Expose `add_units`/`units` in Node.js binding

**Done:** Added `add_units: Option<bool>` parameter to `gen_sum_code_v0` and
`units: Option<Vec<String>>` field to `NapiSumCodeResult` in the Node.js NAPI binding. Wired through
to `iscc_lib::gen_sum_code_v0`. Added 3 new tests covering enabled, disabled/default, and ISCC
prefix verification for units.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added `units: Option<Vec<String>>` field to `NapiSumCodeResult`
    struct, added `add_units: Option<bool>` 4th parameter to `gen_sum_code_v0` function (defaults to
    `false`), mapped `result.units` into the return struct.
- `crates/iscc-napi/__tests__/functions.test.mjs`: Added 3 tests to `gen_sum_code_v0` describe
    block: `addUnits=true` returns 2-element array, default returns `undefined`, and units contain
    valid `ISCC:` prefixed strings.

**Verification:**

- `cargo build -p iscc-napi` — compiles clean
- `cargo clippy -p iscc-napi -- -D warnings` — clean
- `cd crates/iscc-napi && npm run build && npm test` — 135 tests pass (132 existing + 3 new), 0
    failures
- `gen_sum_code_v0(path, 64, false, true)` returns object with `units` array of 2 ISCC strings —
    confirmed via test
- `gen_sum_code_v0(path)` returns object with `units === undefined` — confirmed via test
- `mise run check` — 14/14 hooks pass

**Next:** Continue binding propagation for issue #21. Remaining bindings: WASM (`crates/iscc-wasm`),
C FFI (`crates/iscc-ffi`), JNI (`crates/iscc-jni`), Go (`packages/go`). WASM is a good next target
since it uses a similar pattern to NAPI (struct + function wrapper).

**Notes:** Clean implementation with no surprises. The `#[napi(object)]` derive handles
`Option<Vec<String>>` as `string[] | undefined` in TypeScript automatically — no special
serialization needed. The existing `bits: Option<u32>` and `wide: Option<bool>` params already
demonstrated the `Option<T>` pattern for optional JS params, so `add_units: Option<bool>` follows
the same convention exactly.
