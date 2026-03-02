## 2026-03-02 — Expose `add_units`/`units` in C FFI binding (issue #21)

**Done:** Added `add_units` parameter to `iscc_gen_sum_code_v0` and `units` field (`char **`,
NULL-terminated array) to `IsccSumCodeResult` in the C FFI layer. When `add_units=true`, the result
includes Data-Code and Instance-Code ISCC strings; when `false`, `units` is NULL. The free function
properly cleans up the units array via `iscc_free_string_array`. Regenerated `iscc.h` header via
cbindgen.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `units: *mut *mut c_char` to `IsccSumCodeResult`,
    `units: ptr::null_mut()` to `null_sum_code_result()`, `add_units: bool` parameter to
    `iscc_gen_sum_code_v0` with `vec_to_c_string_array` conversion, cleanup in
    `iscc_free_sum_code_result`. Updated 4 existing tests (added `false` as 4th arg), added 3 new
    tests (units_enabled, units_disabled, units_content with decode verification).
- `crates/iscc-ffi/tests/test_iscc.c`: Updated 3 call sites (tests 24-26) from 3 to 4 args. Added 2
    new C tests (27: units=true verification with array walk, 28: units=false NULL check). Total: 65
    C tests.
- `crates/iscc-ffi/include/iscc.h`: Regenerated via cbindgen — `iscc_IsccSumCodeResult` now has
    `char **units`, `iscc_gen_sum_code_v0` takes 4th `bool add_units` parameter.

**Verification:**

- `cargo test -p iscc-ffi`: 85 tests passed (82 existing + 3 new units tests)
- `cargo clippy -p iscc-ffi -- -D warnings`: clean, no warnings
- `cargo build -p iscc-ffi`: builds successfully
- C header regeneration + freshness: `cbindgen` output matches committed header
- C test compilation: `gcc` compiles successfully against updated header and shared library
- C test execution: `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` — 65 passed, 0 failed
- `mise run check`: all 14 pre-commit hooks pass

**Next:** Continue issue #21 binding propagation. Next target is Java/JNI (`crates/iscc-jni`): add
`addUnits` boolean parameter to the JNI `genSumCodeV0` method, extend `SumCodeResult.java` with a
`String[] units` field, update the JNI bridge to convert `Vec<String>` to `jobjectArray`, and update
Maven tests.

**Notes:** The error cleanup path in `iscc_gen_sum_code_v0` handles the case where
`vec_to_c_string_array` returns NULL — both `iscc` and `datahash` C strings are freed before
returning the null result. The `iscc_free_string_array` already handles NULL gracefully (no-op), so
the free function is safe regardless of whether units was populated.
