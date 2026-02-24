## 2026-02-24 — Harden alg_simhash with length validation

**Done:** Split `alg_simhash` into a validated public API (`alg_simhash` → `IsccResult<Vec<u8>>`)
and an unchecked internal variant (`alg_simhash_inner`). The public function validates equal-length
digests before delegating. All 7 internal call sites in `lib.rs` switched to `alg_simhash_inner`.
All 4 binding crates updated to propagate the `Result` through their error conventions
(PyValueError, napi::Error, JsError, FFI null+last_error).

**Files changed:**

- `crates/iscc-lib/src/simhash.rs`: Added `alg_simhash` wrapper returning `IsccResult<Vec<u8>>` with
    digest-length validation; renamed original to `pub(crate) alg_simhash_inner`
- `crates/iscc-lib/src/lib.rs`: Changed 7 internal call sites (`meta_name_simhash`,
    `soft_hash_meta_v0`, `soft_hash_meta_v0_with_bytes`, `soft_hash_audio_v0` ×4,
    `soft_hash_codes_v0`) from `alg_simhash` to `alg_simhash_inner`
- `crates/iscc-py/src/lib.rs`: Changed `alg_simhash` binding to return `PyResult<Vec<u8>>`, mapping
    errors to `PyValueError`
- `crates/iscc-napi/src/lib.rs`: Changed `alg_simhash` binding to return `napi::Result<Buffer>`,
    mapping errors via `napi::Error::from_reason`
- `crates/iscc-wasm/src/lib.rs`: Changed `alg_simhash` binding to propagate `Result` via `JsError`
- `crates/iscc-ffi/src/lib.rs`: Changed `iscc_alg_simhash` to match on `Result`, setting last_error
    on `Err`
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: Updated 6 existing tests to `.unwrap()` the
    Result; added 2 new error-case tests (`mismatched_lengths_returns_error`,
    `mismatched_three_digests`)

**Verification:** All quality gates pass:

- 206 iscc-lib unit tests pass (including new `test_alg_simhash_mismatched_lengths_returns_error`)
- 30 integration tests pass (including 2 new mismatched-length tests)
- 62 iscc-ffi tests pass
- 147 Python tests pass
- 103 Node.js tests pass
- `mise run check` (all pre-commit hooks) clean

**Next:** The next candidate from the original plan is hardening `soft_hash_video_v0` (validate
frame_sigs non-empty, validate elements have ≥380 features) or hardening `soft_hash_codes_v0`
(validate code count ≥2). Alternatively, could tackle `alg_minhash_256` hardening or
`alg_cdc_chunks` parameter validation.

**Notes:** The work package listed 6 files (excluding tests), which exceeds the general 3-file
guideline, but each binding change was a true 1-line change as predicted by the define-next agent.
No shortcuts taken, no technical debt introduced. The inner/outer split pattern is clean and
idiomatic — internal callers skip validation (they construct digests themselves), while external API
callers get safety.
