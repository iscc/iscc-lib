## 2026-02-23 — Add 8 simple functions to C FFI bindings

**Done:** Added 8 `extern "C"` functions to the C FFI crate plus `iscc_free_string_array` for array
memory management. This brings C FFI from 11 to 20 exported symbols, achieving parity with
Node.js/WASM on Tier 1 coverage (15 Tier 1 + 3 infrastructure + `iscc_free_string_array` + the
existing `iscc_free_string` = 20 total).

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `string_to_c` helper (infallible String-to-CString),
    `vec_to_c_string_array` helper (Vec<String> to NULL-terminated C string array), 4 text utility
    FFI functions (`iscc_text_clean`, `iscc_text_remove_newlines`, `iscc_text_trim`,
    `iscc_text_collapse`), `iscc_encode_base64`, `iscc_conformance_selftest`, `iscc_decompose`,
    `iscc_sliding_window`, and `iscc_free_string_array`. Also added 18 tests (38 total, up from 20).

**Verification:**

- `cargo build -p iscc-ffi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test -p iscc-ffi` passes all 38 tests (20 existing + 18 new)
- `cargo test --workspace` passes with 268 tests (no regressions)
- All 8 new functions present as `#[unsafe(no_mangle)] pub ... extern "C"` in `lib.rs`
- `iscc_conformance_selftest` returns true in tests
- `iscc_free_string_array(ptr::null_mut())` does not crash
- `iscc_decompose` returns valid NULL-terminated array for `ISCC:AAAYPXW445FTYNJ3`
- All 14 pre-commit hooks pass (`mise run check`)

**Next:** All three non-Python binding crates (Node.js, WASM, C FFI) now have the same 8 simple Tier
1 functions. Good candidates for next steps:

1. **Algorithm primitives** — `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0` across all binding crates (complex type signatures requiring design work)
2. **Streaming hashers** — `DataHasher`/`InstanceHasher` classes in C FFI (opaque struct pattern),
    Node.js, and WASM
3. **Structured result types** — return full result objects/dicts from gen functions (currently all
    non-Python bindings only extract `.iscc` string)

**Notes:** The `vec_to_c_string_array` + `iscc_free_string_array` pair follows the standard C
pattern (NULL-terminated array like argv). On allocation failure in `vec_to_c_string_array`,
already-allocated strings are freed before returning NULL, preventing memory leaks. The
`shrink_to_fit` + `mem::forget` pattern ensures the Vec capacity exactly matches its length, making
`Vec::from_raw_parts` in `iscc_free_string_array` safe. Note: `cargo fmt` reformatted
`ptr_to_optional_str` signature slightly (line wrapping) — this is a cosmetic-only change from the
formatter.
