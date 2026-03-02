## 2026-03-02 — Review of: Expose `add_units`/`units` in C FFI binding (issue #21)

**Verdict:** PASS

**Summary:** Clean, focused implementation that adds the `add_units` parameter and `units` field to
the C FFI layer's `iscc_gen_sum_code_v0` and `IsccSumCodeResult`. The implementation correctly
reuses the existing `vec_to_c_string_array` helper and `iscc_free_string_array` cleanup pattern.
Error cleanup path properly frees `iscc` and `datahash` if `vec_to_c_string_array` fails. All
quality gates pass, all tests green.

**Verification:**

- [x] `cargo test -p iscc-ffi` passes — 85 tests (82 existing + 3 new units tests)
- [x] `cargo clippy -p iscc-ffi -- -D warnings` clean — no warnings
- [x] `cargo build -p iscc-ffi` succeeds
- [x] C header freshness — cbindgen regenerated, `git diff --exit-code` clean
- [x] C test compiles — gcc against updated header and shared library
- [x] C test passes — 65 passed, 0 failed
- [x] `iscc_IsccSumCodeResult` in `iscc.h` contains `char **units` field
- [x] `mise run check` passes — 14/14 pre-commit hooks

**Issues found:**

- (none)

**Codex review:** Codex investigated `extern "C"` / `__cplusplus` guard absence in `iscc.h` and
NULL-safety of `ASSERT_STR_STARTS_WITH` macro in test 27. Both are non-issues: the header is
cbindgen-generated (pre-existing design, not in scope), and the C test already guards `sr.units`
access with a NULL check before dereferencing array elements. No actionable findings.

**Next:** Continue issue #21 binding propagation. Next target is Java/JNI (`crates/iscc-jni`): add
`addUnits` boolean parameter to the JNI `genSumCodeV0` method, extend `SumCodeResult.java` with a
`String[] units` field, update the JNI bridge to convert `Vec<String>` to `jobjectArray`, and update
Maven tests.

**Notes:** Issue #21 progress: Rust core ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI ✅ → JNI (next) →
Go. Two bindings remain. After JNI, Go needs its own pure implementation (not a Rust FFI wrapper).
After all bindings are done, update `docs/rust-api.md` and `docs/architecture.md` to reflect the
4-parameter signature and close issue #21.
