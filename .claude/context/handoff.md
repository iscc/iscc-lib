## 2026-03-01 — Add gen_sum_code_v0 to Node.js bindings

**Done:** Added `NapiSumCodeResult` struct and `gen_sum_code_v0` napi function to the Node.js
binding crate. The function reads a file path, computes Data-Code + Instance-Code in a single pass,
and returns a structured object with `iscc`, `datahash`, and `filesize` fields. Tests were already
pre-written from a previous failed iteration — only the Rust binding code was missing.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added `NapiSumCodeResult` struct (`#[napi(object)]`) with
    `iscc:   String`, `datahash: String`, `filesize: i64`, and `gen_sum_code_v0` napi function with
    `path: String, bits: Option<u32>, wide: Option<bool>` parameters
- `crates/iscc-napi/__tests__/functions.test.mjs`: No changes needed — 6 test cases were already in
    place from a previous iteration (equivalence, result shape, file not found, default params, wide
    mode, filesize correctness)

**Verification:** All quality gates pass:

- `cargo build -p iscc-napi` — clean
- `cargo clippy -p iscc-napi -- -D warnings` — clean
- `npm run build && npm test` — 132 tests pass (126 existing + 6 gen_sum_code_v0), 0 failures
- `mise run check` — all 14 hooks pass
- gen_sum_code_v0 callable from JavaScript, returns object with correct `iscc`, `datahash`,
    `filesize` fields

**Next:** Propagate `gen_sum_code_v0` to WASM bindings (`crates/iscc-wasm/`). WASM needs a different
approach since file I/O isn't available — will need to accept `Uint8Array` data directly or use the
streaming hasher pattern. Check if the core API supports a data-based variant.

**Notes:** The test file and imports were already prepared by a previous iteration that failed
before completing the Rust binding code. This iteration only needed to add the `NapiSumCodeResult`
struct and `gen_sum_code_v0` function to `lib.rs`. The `filesize` field uses `i64` (not `u64`)
because napi-rs doesn't support `u64` directly — files up to 2^53 bytes (~9 PB) are representable as
safe JS integers.
