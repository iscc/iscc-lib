# Next Work Package

## Step: Harden `alg_simhash` — return `IsccResult` on mismatched digest sizes

## Goal

Change `alg_simhash` from panicking on mismatched digest lengths to returning `IsccResult<Vec<u8>>`.
This fixes a [normal] public API robustness issue where Tier 1 callers (including all language
bindings) can trigger an index-out-of-bounds panic with adversarial input.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/simhash.rs` — rename existing impl to `pub(crate) fn alg_simhash_inner`,
        add new validated `pub fn alg_simhash` returning `IsccResult<Vec<u8>>`
    - `crates/iscc-lib/src/lib.rs` — update 7 internal call sites from `simhash::alg_simhash` to
        `simhash::alg_simhash_inner` (avoids cascading return-type changes to internal helpers)
    - `crates/iscc-py/src/lib.rs` — handle `Result` from `iscc_lib::alg_simhash` (1-line change)
    - `crates/iscc-napi/src/lib.rs` — handle `Result` (1-line change)
    - `crates/iscc-wasm/src/lib.rs` — handle `Result` (1-line change)
    - `crates/iscc-ffi/src/lib.rs` — handle `Result` via existing error-setting pattern (1-2 lines)
- **Reference**:
    - `crates/iscc-lib/src/simhash.rs` (current implementation, lines 8-45)
    - `crates/iscc-lib/src/lib.rs` (call sites at lines 67, 86, 110, 475, 483, 496, 605)
    - `.claude/context/issues.md` (issue description)

## Not In Scope

- Fixing `sliding_window` panic — that's a separate issue with its own return-type change; tackle in
    a follow-up step
- Changing return types of internal helper functions (`meta_name_simhash`, `soft_hash_meta_v0`,
    `soft_hash_text_v0`, `soft_hash_audio_v0`) — the `_inner` split avoids this cascade
- Updating documentation pages or README
- Performance optimizations to `alg_simhash` itself (e.g., SIMD bit counting)
- Updating the `.pyi` type stub for the Python binding — keep return type as `list[int]` for now;
    the PyO3 `map_err` wrapper handles the `Result` transparently

## Implementation Notes

**Core pattern — validated public + unchecked internal:**

1. In `simhash.rs`, extract the current `alg_simhash` body into `pub(crate) fn alg_simhash_inner`
    with the same signature (`-> Vec<u8>`). Internal callers always pass equal-length digests
    (blake3 32-byte or audio 4-byte), so no validation needed.

2. Add a new `pub fn alg_simhash` that:

    - Keeps the same generic signature: `(hash_digests: &[impl AsRef<[u8]>]) -> IsccResult<Vec<u8>>`
    - Validates: if `hash_digests` has 2+ elements, check all digest lengths equal the first's
        length. On mismatch, return `Err(IsccError::ValueError("..."))` with a message like "All hash
        digests must have equal length"
    - Delegates to `alg_simhash_inner` for the computation
    - Returns `Ok(result)`

3. In `lib.rs`, change the re-export line (`pub use simhash::alg_simhash;`) — this stays the same
    since the public function name is unchanged. Update 7 internal call sites from
    `simhash::alg_simhash(...)` to `simhash::alg_simhash_inner(...)`.

**Binding updates (all mechanical):**

- **Python** (`iscc-py`): change `iscc_lib::alg_simhash(&hash_digests)` to
    `iscc_lib::alg_simhash(&hash_digests).map_err(|e| PyValueError::new_err(e.to_string()))?`
    (function already returns `PyResult` or needs to)
- **Node.js** (`iscc-napi`): change to
    `iscc_lib::alg_simhash(&hash_digests).map_err(|e| napi::Error::from_reason(e.to_string()))?.into()`
    and update return type to `napi::Result<Buffer>`
- **WASM** (`iscc-wasm`): change `Ok(iscc_lib::alg_simhash(&digests))` to
    `Ok(iscc_lib::alg_simhash(&digests).map_err(|e| JsError::new(&e.to_string()))?)` (already
    returns `Result`)
- **C FFI** (`iscc-ffi`): wrap with existing `set_last_error` + return null buffer on error pattern

**Tests to add/update:**

- In `simhash.rs` inline tests: add a test that `alg_simhash` with mismatched-length digests returns
    `Err` (not panic). Keep existing tests but update to unwrap the `Ok(...)`.
- In `crates/iscc-lib/tests/test_algorithm_primitives.rs`: add an integration test for the error
    case.
- Existing conformance tests should continue passing unchanged (they always use valid inputs).

## Verification

- `cargo test -p iscc-lib` passes (all 205+ existing tests + new error-case tests)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `cargo test --workspace` passes (all crates compile and tests pass)
- `pytest` passes all Python tests
- `npm test --prefix crates/iscc-napi` passes all Node.js tests
- Calling `alg_simhash` with `vec![vec![1u8, 2], vec![1u8, 2, 3]]` returns `Err` (not panic)

## Done When

All verification criteria pass: `alg_simhash` returns `IsccResult` with proper validation, all
existing tests still pass across the workspace, and a new test confirms mismatched digests produce
an error instead of a panic.
