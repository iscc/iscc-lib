## 2026-11-20 — Review of: JNI hasher null-handle guard and OOM/pending-exception mapping

**Verdict:** PASS

**Summary:** Both `normal` JNI issues are closed. `throw_and_default` now takes the error itself
through a private `JavaError` trait: a JVM allocation failure throws `OutOfMemoryError`, everything
else `IllegalArgumentException`, and neither throw helper throws while an exception is pending. The
four hasher `*Update`/`*Finalize` functions reject a zero handle before the `unsafe` deref. The docs
and javadoc now name `IllegalStateException` for a finalized hasher. One code file changed
(`lib.rs`); `IsccLib.java` is javadoc only.

**Verification:**

- [x] `cargo build -p iscc-jni` then `mvn -f crates/iscc-jni/java/pom.xml clean test` exits 0 — 113
    tests, 0 failures (IsccLibTest 108, DataEdgeTest 5); the four `…RejectsNullHandle` tests and
    `allocationFailureSurfacesAsOutOfMemoryError` are in the surefire report
- [x] Same run with `JAVA_TOOL_OPTIONS=-Xcheck:jni` exits 0 — 113 tests, 2
    `Picked up JAVA_TOOL_OPTIONS` lines, 0 `WARNING in native method` lines
- [x] `grep -n 'e.to_string())' crates/iscc-jni/src/lib.rs` prints nothing; `ptr == 0` shows the
    four guards (lines 1156, 1191, 1255, 1290); `exception_check` is in both helpers (79, 97)
- [x] `grep -l IllegalStateException` lists `docs/java-api.md`, `docs/howto/java.md` and
    `IsccLib.java`; the only `IllegalArgumentException` mentions near "finaliz" are the zero-handle
    clauses
- [x] `cargo clippy -p iscc-jni --all-targets -- -D warnings` and `cargo fmt -p iscc-jni --check`
    are clean; workspace clippy is clean too
- [x] `mise run check` exits 0

Two probes beyond next.md:

- **Pending-exception guard, measured.** A shadow `IsccDecodeResult` with a throwing constructor,
    put first on a hand-run JVM's `-cp`, makes `isccDecode` reach the guard. At HEAD the caller sees
    the constructor's exception. With the guard removed (mutant built in `/tmp`) the caller sees
    `IllegalArgumentException: Java exception was thrown`. So the guard works, and the handoff's "no
    deterministic trigger" is not true
- **Release smoke-test path.** `release.yml` → `test-jni` runs `mvn test` with the library loaded
    from classpath resources, not `java.library.path`. `OomProbe` run that way under `-Xmx16m` still
    prints `java.lang.OutOfMemoryError`

**Issues found:**

- Filed `low`, untriaged: "JNI pending-exception guard has no test". With the guard removed
    `mvn clean test` stays green. The entry describes the trigger above
- The advance handoff said `-Xcheck:jni` would flag a `ThrowNew` over a pending exception. HotSpot
    17 printed no warning in the mutant run, so the test's `WARNING in native method` assertion does
    not cover that case. Only the exception class shows it (now in learnings.md)
- Not filed: the OOM test takes 8 s of the 8.4 s `IsccLibTest` run; `proc.waitFor()` has no timeout;
    `assertTrue(!…)` could be `assertFalse`; the `OutOfMemoryError` message is jni's "JNI call
    failed"
- Not filed: `specs/java-bindings.md` still says "`mvn test` passes (65 tests)". The open "Derived
    facts…" issue covers hand-written counts in specs

**Codex review:** no actionable findings. Codex ran the JNI build, clippy, fmt and both Maven runs
(113 tests, clean under `-Xcheck:jni`).

**Next:** the `iscc-ffi` `lib.rs` split (`normal`): move `mod tests` to `src/tests.rs`, keep
`iscc.h` and `NativeMethods.g.cs` byte-identical. The other two open `normal` issues are
`[human]`-sourced: "Derived facts in guidance and docs…" and "`data-edge.json` harnesses accept a
value that only resembles the expected one".

**Notes:**

- Resolved and deleted from issues.md: "JNI streaming-hasher functions dereference Java-supplied
    handle without null guard" and "JNI exports report a pending JVM exception as
    `IllegalArgumentException`"
- decisions.md has a new entry for the error mapping. The `OutOfMemoryError` is a fresh instance,
    not the JVM's original, because jni 0.22 clears the original before the crate sees it
- No core Rust, public API or benchmarked path changed, so CRAP, iai and semver were not run
- learnings.md is at its 200-line budget: the next entry needs an archive move first
