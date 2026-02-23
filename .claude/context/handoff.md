## 2026-02-23 — Review of: Add 8 simple functions to C FFI bindings

**Verdict:** PASS

**Summary:** All 8 Tier 1 functions plus `iscc_free_string_array` were added to the C FFI crate,
bringing it from 11 to 20 exported symbols (parity with Node.js/WASM on simple functions). Code is
clean, well-documented, follows existing patterns exactly, and all 38 tests pass. No quality gate
circumvention detected.

**Verification:**

- [x] `cargo build -p iscc-ffi` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `cargo test -p iscc-ffi` passes all 38 tests (20 existing + 18 new, exceeds 32+ target)
- [x] `cargo test --workspace` passes with 268 tests (no regressions)
- [x] All 8 new functions present as `#[unsafe(no_mangle)] pub ... extern "C"` in `lib.rs`
- [x] `iscc_conformance_selftest` returns true in tests
- [x] `iscc_free_string_array(ptr::null_mut())` does not crash
- [x] `iscc_decompose` returns valid NULL-terminated array for `ISCC:AAAYPXW445FTYNJ3`
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Next:** All three non-Python binding crates now have the same 8 simple Tier 1 functions (20
symbols each). Good candidates for the next step:

1. **Algorithm primitives** — `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0` across all binding crates (complex type signatures requiring design work)
2. **Streaming hashers** — `DataHasher`/`InstanceHasher` classes in C FFI (opaque struct pattern),
    Node.js, and WASM
3. **Structured result types** — return full result objects/dicts from gen functions (currently all
    non-Python bindings only extract `.iscc` string)
4. **Update state.md** — state.md still says C FFI is at 11/23; it should reflect 20/23 (or
    equivalently 17 Tier 1 symbols + 3 infrastructure)

**Notes:** The `string_to_c` helper nicely reduces duplication vs the existing `result_to_c_string`
(which handles `Result<String, IsccError>` while `string_to_c` handles infallible `String`). The
`vec_to_c_string_array` + `iscc_free_string_array` pair is a clean, standard C pattern with proper
cleanup on partial allocation failure.
