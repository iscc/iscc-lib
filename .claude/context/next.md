# Next Work Package

## Step: Add 8 simple functions to C FFI bindings

## Goal

Expand C FFI from 11 to 20 exported symbols by adding the same 8 Tier 1 functions already in Node.js
and WASM: 4 text utilities, `encode_base64`, `iscc_decompose`, `conformance_selftest`, and
`sliding_window`. This brings C FFI to parity with Node.js/WASM on Tier 1 coverage (15 Tier 1 + 3
infrastructure + `iscc_free_string_array` for array returns = 20 total).

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` — add 8 `extern "C"` functions + `vec_to_c_string_array`
    helper + `iscc_free_string_array` export + inline tests
- **Reference**: `crates/iscc-wasm/src/lib.rs` (the 8 functions just added in WASM),
    `crates/iscc-napi/src/lib.rs` (Node.js equivalents), `crates/iscc-lib/src/utils.rs` (text util
    signatures), `crates/iscc-lib/src/codec.rs` (`encode_base64`, `iscc_decompose`),
    `crates/iscc-lib/src/conformance.rs` (`conformance_selftest`), `crates/iscc-lib/src/simhash.rs`
    (`sliding_window`)

## Not In Scope

- Algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `soft_hash_video_v0`) —
    these have complex type signatures requiring separate design work
- `DataHasher`/`InstanceHasher` streaming classes — C FFI opaque struct patterns are a distinct step
- Structured return types for gen functions (returning objects instead of ISCC strings)
- Changes to the C test program (`tests/test_ffi.c`) — that can be expanded in a future step
- Changes to any other binding crate (Node.js, WASM, Python)
- cbindgen configuration changes — it should auto-generate headers for the new exports

## Implementation Notes

### Group 1: Scalar-returning functions (6 functions)

These follow the existing C FFI pattern exactly. Convert string results via `CString::new` then
`into_raw`, using `set_last_error` plus `ptr::null_mut()` on failure.

Create a small `string_to_c` helper to avoid repeating the CString conversion match block. It takes
a `String`, converts to `CString`, and returns a raw char pointer (or sets error and returns null).

1. `iscc_text_clean`: takes a C string pointer, returns heap-allocated C string. Use `ptr_to_str`
    then `string_to_c(iscc_lib::text_clean(text))`
2. `iscc_text_remove_newlines`: same pattern as `iscc_text_clean`
3. `iscc_text_trim`: takes a C string pointer and `nbytes: usize`, returns heap-allocated C string
4. `iscc_text_collapse`: same pattern as `iscc_text_clean`
5. `iscc_encode_base64`: takes a byte pointer and length, returns heap-allocated C string. Follows
    the byte-buffer pattern from `iscc_gen_data_code_v0`. Check for null pointer on data
6. `iscc_conformance_selftest`: takes no parameters, returns `bool`. Trivial, no unsafe needed. Just
    call `iscc_lib::conformance_selftest()` directly

### Group 2: Array-returning functions (2 functions)

`iscc_decompose` and `sliding_window` return `Vec<String>`. For C FFI, use a NULL-terminated string
array — the standard C pattern (like `argv`).

Infrastructure needed:

- `vec_to_c_string_array`: converts `Vec<String>` to a heap-allocated NULL-terminated array of C
    strings. Converts each String to CString, collects raw pointers, appends a NULL terminator, then
    `shrink_to_fit` plus `as_mut_ptr` plus `mem::forget` to hand ownership to the caller
- `iscc_free_string_array`: exported `extern "C"` function. Walks the array freeing each CString via
    `CString::from_raw`, counts elements, then reconstructs the Vec with `Vec::from_raw_parts` using
    `count + 1` (including NULL terminator) for both len and capacity (safe because `shrink_to_fit`
    ensures capacity equals length)

Functions:

- `iscc_decompose`: takes a C string pointer, returns NULL-terminated array of C strings on success,
    NULL on error (check `iscc_last_error()`)
- `iscc_sliding_window`: takes a C string pointer and `width: u32`, returns NULL-terminated array of
    C strings. Pre-validate width less than 2 to avoid Rust panic across FFI boundary (same pattern
    as WASM/Python bindings). Use `u32` for width (not `usize`) for C interop

### Naming convention

All new symbols use `iscc_` prefix matching existing convention: `iscc_text_clean`,
`iscc_text_remove_newlines`, `iscc_text_trim`, `iscc_text_collapse`, `iscc_encode_base64`,
`iscc_conformance_selftest`, `iscc_decompose`, `iscc_sliding_window`, `iscc_free_string_array`.

### Tests

Add inline tests in the existing `#[cfg(test)] mod tests` block. Reuse the `c_ptr_to_string` helper.
Add a new `c_ptr_to_string_vec` helper for array returns that walks the NULL-terminated array. Test
cases per function:

- text_clean: known NFKC normalization, NULL input returns NULL
- text_remove_newlines: multi-line to single line, NULL returns NULL
- text_trim: truncation, NULL returns NULL
- text_collapse: lowercased and stripped, NULL returns NULL
- encode_base64: known byte input to known base64url output, NULL returns NULL
- conformance_selftest: returns true
- iscc_decompose: decompose known ISCC-CODE, invalid input returns NULL, NULL returns NULL
- sliding_window: known n-grams, width less than 2 returns NULL, NULL returns NULL
- free_string_array: NULL is no-op (like free_string)

## Verification

- `cargo build -p iscc-ffi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test -p iscc-ffi` passes all existing 20 tests plus new tests (expect 32+ total)
- `cargo test --workspace` passes with 250+ tests (no regressions)
- All 8 new functions present as `#[unsafe(no_mangle)] pub ... extern "C"` in `lib.rs`
- `iscc_conformance_selftest` returns true in tests
- `iscc_free_string_array(ptr::null_mut())` does not crash
- `iscc_decompose` returns valid NULL-terminated array for a known ISCC-CODE

## Done When

All verification criteria pass: 8 new `extern "C"` functions plus `iscc_free_string_array` compile,
clippy is clean workspace-wide, and all C FFI tests (existing plus new) pass via
`cargo test -p iscc-ffi`.
