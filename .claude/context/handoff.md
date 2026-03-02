## 2026-03-02 — Review of: Add gen_sum_code_v0 to Java JNI bindings

**Verdict:** PASS

**Summary:** `gen_sum_code_v0` is correctly implemented in the Java JNI bindings with a
`SumCodeResult.java` immutable result class, `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` JNI
bridge function following the `isccDecode` object-return pattern, and a `genSumCodeV0` native method
declaration in `IsccLib.java`. All 4 tests pass, all quality gates are clean.

**Verification:**

- [x] `cargo test -p iscc-jni` passes — 0 Rust tests (expected, JNI-only crate)
- [x] `cargo clippy -p iscc-jni -- -D warnings` clean
- [x] `cd crates/iscc-jni/java && mvn test` passes — 62 tests, 0 failures (58 existing + 4 new)
- [x] `SumCodeResult.java` exists at expected path
- [x] `genSumCodeV0` native method declared in `IsccLib.java` with Javadoc
- [x] JNI bridge function `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` present in `lib.rs`
- [x] `mise run check` passes — all 14 pre-commit hooks clean

**Issues found:**

- (none) — clean implementation

**Codex review:** Codex explored the diff thoroughly and considered: (1) negative `bits` cast from
`jint` to `u32` — dismissed as consistent with all other gen functions in the crate; (2)
`JValue::Object(&iscc_jstr)` type compatibility — correct via `JString` auto-deref to `JObject`; (3)
`find_class` on Java thread — safe. No actionable bugs found.

**Next:** Propagate `gen_sum_code_v0` to Go bindings (`packages/go/`). This is the last binding (7
of 7) and requires a pure Go reimplementation since Go doesn't wrap Rust. After Go, issue #15 is
complete. Also bundle the FFI module docstring function count update ("9→10 gen functions") in
`crates/iscc-ffi/src/lib.rs` with the Go step or a cleanup step.

**Notes:** Issue #15 progress: Rust core ✅, Python ✅, Node.js ✅, WASM ✅, C FFI ✅, Java ✅. One
binding remains: Go. The test file header comment was updated from "9" to "10" gen functions as a
minor fix.
