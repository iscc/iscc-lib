## 2026-03-02 — Add gen_sum_code_v0 to C FFI bindings

**Done:** Added `IsccSumCodeResult` struct, `iscc_gen_sum_code_v0` extern "C" function, and
`iscc_free_sum_code_result` free function to the C FFI crate. Added 4 Rust unit tests and 3 C test
cases (temp file, NULL path, nonexistent path).

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `IsccSumCodeResult` repr(C) struct, `null_sum_code_result()`
    helper, `iscc_gen_sum_code_v0(path, bits, wide)` extern "C" function,
    `iscc_free_sum_code_result(result)` free function, and 4 Rust tests
    (`test_gen_sum_code_v0_basic`, `test_gen_sum_code_v0_null_path`,
    `test_gen_sum_code_v0_free_null_strings`, `test_gen_sum_code_v0_matches_lib`)
- `crates/iscc-ffi/tests/test_iscc.c`: Added 3 C test cases (#24 temp file, #25 NULL path, #26
    nonexistent path)

**Verification:**

- `cargo test -p iscc-ffi`: 82 tests passed (78 existing + 4 new), 0 failed
- `cargo clippy -p iscc-ffi -- -D warnings`: clean
- `cargo build -p iscc-ffi`: success
- C test program compiled and ran: 57 passed (49 existing + 8 new assertions), 0 failed
- `mise run check`: all 14 pre-commit hooks passed

**Next:** Propagate `gen_sum_code_v0` to Java bindings (`crates/iscc-jni/`) per issue #15. After
Java, continue with Go (pure Go reimplementation). Then update module docstring function count.

**Notes:** The implementation follows the `IsccDecodeResult` pattern exactly: `#[repr(C)]` struct
with `ok: bool`, heap-allocated string fields, and a dedicated free function that delegates to
`iscc_free_string`. The `iscc_gen_sum_code_v0` function properly handles partial allocation failure
(if `datahash` CString conversion fails after `iscc` was already allocated, it frees `iscc` before
returning the error result). The C test uses `/tmp/iscc_c_test_sum.bin` as a temp file path, which
is cleaned up after the test.
