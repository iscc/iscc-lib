# Next Work Package

## Step: Add gen_sum_code_v0 to C FFI bindings

## Goal

Propagate `gen_sum_code_v0` to the C FFI crate (`crates/iscc-ffi/`), making it 32/32 Tier 1 symbols
in C. This is the first of three remaining bindings for issue #15.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-ffi/src/lib.rs` — add `IsccSumCodeResult` struct, `iscc_gen_sum_code_v0` extern "C"
        function, `iscc_free_sum_code_result` free function, Rust unit tests
    - `crates/iscc-ffi/tests/test_iscc.c` — add C test cases for `gen_sum_code_v0` (create temp file,
        call function, verify result, free, test NULL path error)
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` — existing patterns for `IsccDecodeResult`, `iscc_decode`,
        `iscc_free_decode_result`, `result_to_c_string`, `ptr_to_str`
    - `crates/iscc-lib/src/api.rs` — `gen_sum_code_v0` signature and `SumCodeResult` type
    - `crates/iscc-ffi/tests/test_iscc.c` — existing C test patterns

## Not In Scope

- Updating the cbindgen header file (auto-generated in CI, not checked in)
- Updating CI workflow (already tests C FFI with gcc + test_iscc)
- Java or Go bindings (separate steps)
- Updating README or documentation (wait until all 3 remaining bindings are done)
- Updating module docstring function count (cosmetic, can bundle with Java step)

## Implementation Notes

**Result struct pattern** — follow `IsccDecodeResult` for struct-return pattern:

```rust
#[repr(C)]
pub struct IsccSumCodeResult {
    pub ok: bool,
    pub iscc: *mut c_char,
    pub datahash: *mut c_char,
    pub filesize: u64,
}
```

Place the struct definition near `IsccDecodeResult` (around line 795). Add a helper
`null_sum_code_result()` that returns a zeroed-out error result (like the inline error returns in
`iscc_decode`).

**Function signature:**

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_gen_sum_code_v0(
    path: *const c_char,
    bits: u32,
    wide: bool,
) -> IsccSumCodeResult
```

Pattern: `clear_last_error()` → `ptr_to_str(path, "path")` →
`iscc_lib::gen_sum_code_v0(Path::new(path_str), bits, wide)` → convert `SumCodeResult` fields to C
types. On error, `set_last_error` and return `ok: false` result. On success, convert `iscc` and
`datahash` via `CString::new().into_raw()`.

**Free function:**

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_sum_code_result(result: IsccSumCodeResult) {
    unsafe { iscc_free_string(result.iscc) };
    unsafe { iscc_free_string(result.datahash) };
}
```

`iscc_free_string` already handles NULL, so this is safe even for error results.

**C test cases** — add 3 test cases after existing test #23:

1. **gen_sum_code_v0 with temp file**: Write "Hello World" to a temp file via `fopen`/`fwrite`, call
    `iscc_gen_sum_code_v0(path, 64, false)`, verify `ok == true`, `iscc` starts with `"ISCC:"`,
    `datahash` is non-NULL, `filesize == 11`, then `iscc_free_sum_code_result`. Clean up temp file
    with `remove()`.

2. **gen_sum_code_v0 NULL path**: Call with NULL path, verify `ok == false`, `iscc` is NULL.

3. **gen_sum_code_v0 nonexistent path**: Call with `"/nonexistent/file.bin"`, verify `ok == false`.

**Rust unit tests** — add tests in the `#[cfg(test)]` module:

1. Test `iscc_gen_sum_code_v0` with a real temp file (use `std::io::Write` + `std::env::temp_dir()`)
2. Test NULL path returns `ok: false`
3. Test `iscc_free_sum_code_result` with null strings (no-op safety)
4. Test result fields match `iscc_lib::gen_sum_code_v0` output

**Import note**: Add `use std::path::Path;` if not already imported. The function needs
`std::fs::File` or similar for temp file tests.

## Verification

- `cargo test -p iscc-ffi` passes (78 existing + ~4 new tests = ~82 tests)
- `cargo clippy -p iscc-ffi -- -D warnings` clean
- `cargo build -p iscc-ffi` succeeds
- `IsccSumCodeResult` struct and `iscc_gen_sum_code_v0` function are present in source
- C test program compiles and passes when run locally:
    `cargo build -p iscc-ffi && cbindgen --crate iscc-ffi -o crates/iscc-ffi/tests/iscc.h && gcc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/tests -L target/debug -liscc_ffi -lpthread -ldl -lm && LD_LIBRARY_PATH=target/debug /tmp/test_iscc`

## Done When

All verification criteria pass: Rust tests, clippy, and the C test program all succeed with the new
`gen_sum_code_v0` function returning correct results for a file-based input.
