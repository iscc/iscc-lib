## 2026-02-25 — JNI IllegalStateException for finalized hashers

**Done:** Added `throw_state_error` helper to JNI bindings that throws `IllegalStateException`
instead of `IllegalArgumentException`. Changed 4 call sites in the streaming hasher update/finalize
functions and updated 2 doc comments. Added 2 Java tests verifying the exception type.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: Added `throw_state_error` helper function, changed 4 "already
    finalized" error paths from `throw_and_default` to `throw_state_error`, updated 2 doc comments
    from `IllegalArgumentException` to `IllegalStateException`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Added
    `testDataHasherThrowsIllegalStateAfterFinalize` and
    `testInstanceHasherThrowsIllegalStateAfterFinalize` test methods

**Verification:**

- [x] `cargo build -p iscc-jni` succeeds
- [x] `mvn test` passes — 51 tests (49 existing + 2 new), 0 failures
- [x] `grep -c 'throw_state_error'` outputs 5 (1 definition + 4 call sites; next.md expected 4
    counting call sites only, but grep counts lines)
- [x] `grep -c 'IllegalStateException' lib.rs` outputs 4 (≥1 required)
- [x] `grep -c 'IllegalStateException' IsccLibTest.java` outputs 4 (≥2 required)
- [x] `cargo clippy -p iscc-jni -- -D warnings` clean
- [x] `mise run check` — all 14 pre-commit hooks pass

**Next:** The `[low] iscc-jni: All exceptions mapped to IllegalArgumentException` issue is resolved.
The Java-side Javadoc in `IsccLib.java` still says `@throws IllegalArgumentException` for the hasher
update/finalize methods (lines 266, 280, 308, 322) — consider updating those in a follow-up to match
the actual `IllegalStateException` behavior. Remaining low-priority issues: TypeScript port
evaluation and WASM CLAUDE.md staleness. The project is ready for `v0.0.1` release.

**Notes:** The `throw_state_error` grep count is 5 (not 4 as next.md predicted) because `grep -c`
counts all lines matching the pattern, including the function definition line. This is correct — 1
definition + 4 call sites = 5 matching lines. The `IsccLib.java` Javadoc was not updated because
next.md scoped it as "Reference" only. The cosmetic mismatch between Java Javadoc and actual
behavior is harmless but should be cleaned up.
