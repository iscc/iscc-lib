## 2026-02-24 — Replace unwrap() calls in JNI entrypoints with fallible error handling

**Done:** Replaced all 21 `unwrap()` calls in `extern "system"` JNI functions with
`throw_and_default` error handling. Three patterns were addressed: Pattern A (16
`env.new_string(...).unwrap()` → nested match), Pattern B (3
`env.byte_array_from_slice(...).unwrap()` → nested match), and Pattern C (2 unwraps in
`algCdcChunks` loop → early-return match + `if let Err`).

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: Replaced all 21 `unwrap()` calls with fallible error handling that
    throws `IllegalArgumentException` via `throw_and_default` and returns a type-appropriate default
    value

**Verification:**

- `grep -c 'unwrap()' crates/iscc-jni/src/lib.rs` returns 0 — all unwrap calls eliminated
- `cargo clippy -p iscc-jni -- -D warnings` passes clean
- `cargo build -p iscc-jni` succeeds
- `cd crates/iscc-jni/java && mvn test` passes all 46 conformance vectors
- `mise run check` passes all pre-commit hooks

**Next:** The critical JNI unwrap issue is resolved. Remaining JNI hardening items from issues.md
are normal/low priority: `jint` negative value validation (normal), local reference table overflow
in loops (normal), and exception type differentiation (low). The define-next agent could also
address the Python binding issues (bytes-like input misclassification, unbounded `.read()`) or the
performance-related issues (codec header parsing, DataHasher allocation overhead).

**Notes:** Pure mechanical replacement — no behavioral change for happy-path execution. The error
paths now throw Java exceptions instead of panicking/aborting. No new tests added because JNI
allocation failures are extremely difficult to trigger from Java-side tests; the fix is mechanically
verifiable by the zero-unwrap grep count and passing conformance tests.
