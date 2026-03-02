## 2026-03-02 — Review of: Expose `add_units`/`units` in Java/JNI binding (issue #21)

**Verdict:** PASS

**Summary:** Clean, focused implementation that adds `addUnits` parameter and `units` field to the
Java/JNI binding's `genSumCodeV0` across all three layers (Java class, native declaration, Rust JNI
bridge). The implementation correctly reuses the existing `build_string_array` helper and matches
the `isccDecompose` pattern for `unsafe { JObject::from_raw(arr) }`. All 65 Maven tests pass (62
existing + 3 new), clippy clean, all 14 pre-commit hooks pass.

**Verification:**

- [x] `cargo build -p iscc-jni` succeeds
- [x] `cargo clippy -p iscc-jni -- -D warnings` clean — no warnings
- [x] `cargo test -p iscc-jni` passes — 0 Rust-side tests (no JVM in test env)
- [x] `cd crates/iscc-jni/java && mvn test` passes — 65 tests, 0 failures, 0 errors
- [x] `mise run check` passes — 14/14 pre-commit hooks

**Issues found:**

- (none)

**Codex review:** Ran against the define-next commit (HEAD~1) instead of the advance commit (HEAD)
due to protocol template mismatch. Confirmed the 4 test method names exist in the test file. No
actionable findings about the actual implementation.

**Next:** Continue issue #21 binding propagation. Last remaining binding: Go (`packages/go`). Add
`addUnits bool` parameter to `GenSumCodeV0` and `Units []string` field to `SumCodeResult`. This is a
pure Go reimplementation — the function needs to collect the Data-Code and Instance-Code ISCC
strings during execution and return them in the result struct. After Go is done, issue #21 is
complete — update docs (`rust-api.md`, `architecture.md`) and close the issue.

**Notes:** Issue #21 progress: Rust core ✅ → Python ✅ → Node.js ✅ → WASM ✅ → C FFI ✅ → JNI ✅ → Go
(next). Only Go remains. The Go binding is a pure rewrite (not Rust FFI wrapper), so the advance
agent will need to modify `code_sum.go` and `code_sum_test.go` directly.
