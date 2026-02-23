## 2026-02-23 — Add DataHasher/InstanceHasher opaque pointer FFI to C bindings

**Done:** Added 8 `extern "C"` functions exposing `DataHasher` and `InstanceHasher` streaming types
via the opaque pointer lifecycle pattern (new/update/finalize/free). This brings C FFI from 21/23 to
23/23 Tier 1 parity, completing full parity across all four binding targets (Python, Node.js, WASM,
C FFI).

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `FfiDataHasher` and `FfiInstanceHasher` wrapper structs with
    `Option<Inner>` for finalize-once semantics. Added 8 `extern "C"` functions
    (`iscc_data_hasher_new/update/finalize/free` and
    `iscc_instance_hasher_new/update/finalize/free`). Added 12 Rust unit tests covering basic
    lifecycle, gen-function equivalence, multi-update, empty-data, finalize-twice error, and
    free-NULL safety for both hasher types.
- `crates/iscc-ffi/tests/test_iscc.c`: Added 4 C test sections (7 new assertions): DataHasher basic
    lifecycle, InstanceHasher empty-data finalize, DataHasher multi-update consistency, and
    free-NULL safety for both types.

**Verification:**

- `cargo test -p iscc-ffi` passes: 62 tests (50 existing + 12 new)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test --workspace` passes: 292 tests across all crates (no regressions)
- C test program compiles and passes: 30/30 (20 existing + 10 new assertions)
- `mise run check` passes all 14 pre-commit hooks

**Next:** All four binding targets now have 23/23 Tier 1 parity. The natural next steps are:
structured return types for gen functions (returning full result objects beyond just the `.iscc`
string), or splitting `iscc-ffi/src/lib.rs` into submodules (now ~1,700 lines).

**Notes:** The implementation follows the exact pattern specified in next.md: `Option<Inner>`
wrapper structs (not `#[repr(C)]`), `Box::into_raw`/`Box::from_raw` lifecycle, bool returns for
update, `*mut c_char` for finalize. All FFI functions call `clear_last_error()` at entry. NULL
pointer checks on the hasher parameter use `set_last_error` and return appropriate error values. The
`update` functions handle zero-length data with empty slices (matching the existing pattern in
`iscc_alg_cdc_chunks`). No surprises or deviations from the work package.
