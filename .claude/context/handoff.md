## 2026-02-24 — Review of: Replace unwrap() calls in JNI entrypoints with fallible error handling

**Verdict:** PASS

**Summary:** All 21 `unwrap()` calls in `extern "system"` JNI functions replaced with
`throw_and_default` error handling. The change is purely mechanical — three patterns (A: 16
`new_string` calls, B: 3 `byte_array_from_slice` calls, C: 2 loop-body calls in `algCdcChunks`) all
handled uniformly. Zero behavioral change on happy path; error paths now throw Java exceptions
instead of aborting the JVM.

**Verification:**

- [x] `grep -c 'unwrap()' crates/iscc-jni/src/lib.rs` returns 0 — all 21 unwrap calls eliminated
- [x] `cargo clippy -p iscc-jni -- -D warnings` passes clean
- [x] `cargo build -p iscc-jni` succeeds
- [x] `cd crates/iscc-jni/java && mvn test` passes all 46 conformance vectors
- [x] `mise run check` passes all 14 pre-commit hooks
- [x] No quality gate circumvention (no `#[allow]`, no test skipping, no threshold changes)

**Issues found:**

- (none) — clean mechanical replacement, no scope creep

**Next:** The critical JNI unwrap issue is resolved (deleted from issues.md). Remaining work
candidates by priority: (1) normal-priority issues — `jint` negative value validation, JNI local
reference overflow, Python bytes-like input misclassification, Python unbounded `.read()`, napi
version skew, napi npm packaging, napi unnecessary clone, wasm silent null return, FFI video frame
copy; (2) low-priority issues — `alg_dct` power-of-two, `alg_wtahash` bounds check, JNI exception
type differentiation, Python `__version__`, Python module docstring, WASM conformance_selftest size,
WASM stale CLAUDE.md, TypeScript port evaluation; (3) performance issues — codec header parsing,
DataHasher allocation overhead. The Python binding issues (bytes-like misclassification + unbounded
read) are a natural pair for a single iteration.

**Notes:** The advance agent's work was exemplary for this type of mechanical fix — exactly 21
removals confirmed by grep, all following the documented patterns from next.md. No new
error-handling infrastructure was needed since `throw_and_default` already existed.
