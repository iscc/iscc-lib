## 2026-11-20 — JNI hasher null-handle guard and OOM/pending-exception mapping

**Done:** `throw_and_default` is now type-driven through a `JavaError` trait (impls for
`jni::errors::Error`, `iscc_lib::IsccError`, `str`). `JniError::NoMemory` throws a fresh
`OutOfMemoryError` and every other error throws `IllegalArgumentException`. Both throw helpers
return the default without throwing while an exception is pending. All 90 `&e.to_string()` sites now
pass `&e`. The four hasher `*Update`/`*Finalize` functions reject a zero handle with
`IllegalArgumentException` before the `unsafe` deref. The docs that said a finalized hasher throws
`IllegalArgumentException` now say `IllegalStateException`.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: `JavaError` trait + impls; `exception_check` guard in
    `throw_and_default` and `throw_state_error`; OOM class selection; 90 call sites `&e`; one
    `&format!(…)` site became `format!(…).as_str()` (no `String` impl); 4 `ptr == 0` guards;
    reworded `// SAFETY:` comments and the hasher doc comments; module `//!` "Error handling"
    paragraph
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: hasher `@throws` javadoc only
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: 4 null-handle tests +
    `allocationFailureSurfacesAsOutOfMemoryError` (child JVM)
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/OomProbe.java` (new, test source): child
    `main` that runs `encodeSeq` over 600,000 copies of one 34-byte Data-Code unit, so the result
    `byte[]` is ~20 MB under `-Xmx16m`. It prints the class name of the exception the call threw
- `docs/java-api.md`, `docs/howto/java.md`: finalized → `IllegalStateException`, zero handle →
    `IllegalArgumentException`, Error Handling section adds the `OutOfMemoryError` line
- `crates/iscc-jni/CLAUDE.md`: type-mapping row, Error Handling bullets (trait, guard, null handle,
    OOM test), `OomProbe.java` added to the module layout

**Verification:**

- Fail-first: on the unfixed `lib.rs` (fresh `cargo build -p iscc-jni`),
    `mvn … clean test -Dtest='IsccLibTest#allocationFailureSurfacesAsOutOfMemoryError'` exited 1
    with `expected: <java.lang.OutOfMemoryError> but was: <java.lang.IllegalArgumentException>`. I
    did not run the null-handle tests on the unfixed build, because they would crash the JVM with a
    SIGSEGV
- Fixed: `cargo build -p iscc-jni` then `mvn -f crates/iscc-jni/java/pom.xml clean test` exited 0
    with `Tests run: 113, Failures: 0, Errors: 0, Skipped: 0` (IsccLibTest 108, DataEdgeTest 5). I
    ran it again after the javadoc edits: still 113/0
- `JAVA_TOOL_OPTIONS=-Xcheck:jni mvn … clean test` exited 0 with 113 tests. The log has 2
    `Picked up JAVA_TOOL_OPTIONS` lines and 0 `WARNING in native method` lines
- `grep -n 'e.to_string())' crates/iscc-jni/src/lib.rs` prints nothing. `grep -c 'ptr == 0'` gives
    4\. `exception_check` appears at lines 79 and 97, one in each helper
- `grep -l IllegalStateException` over the three doc files lists all three. The only
    `IllegalArgumentException` mentions near "finaliz" are the zero-handle clauses
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` and `cargo fmt -p iscc-jni --check` are
    clean. `mise run check` exited 0

**Next:** The `iscc-ffi` `lib.rs` split (move `mod tests` out), the other remaining `normal` issue
in this area.

**Notes:**

- The tests do not exercise the pending-exception guard (`Error::JavaException` from
    `jni_call_post_check_ex!` calls such as `set_region`) directly: I found no deterministic
    trigger. The OOM path does cover the classification, and `-Xcheck:jni` in the child JVM would
    flag a `ThrowNew` over a pending exception
- The OOM probe builds its input before the probed call and catches only around `encodeSeq`, so an
    OOM while building the input cannot be mistaken for a pass. The child uses `java.home/bin/java`,
    not `ProcessHandle…command()`. That path is equivalent and needs no `Optional`. The child's
    `-cp` is surefire's `java.class.path`, a manifest-only booter jar, and it worked
- The OOM message is jni's `Display` text "JNI call failed". Java callers catch the class, not the
    message
- The `*Free` functions are unchanged. A stale non-zero handle is still undetectable, as next.md
    ruled out of scope
- No core Rust, public API or benchmarked path changed, so CRAP and iai were not run
