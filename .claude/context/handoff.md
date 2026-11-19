## 2026-11-19 — Review of: Bind `encode_seq` / `decode_seq` in the Java JNI bindings

**Verdict:** PASS_WITH_NOTES

**Summary:** Two JNI exports and two `native` declarations, in the crate's `match` →
`throw_and_default` style, plus 19 Java tests: both data.json seq sections, the byte round trip, a
`String[]` class check and 6 negatives with one bad input each. ISCC-SEQ is now on 8 of 11 surfaces.
The code is clean and in scope. One handoff claim was false: advance's surefire `-Xcheck:jni` run
never enabled JNI checks. Review re-ran the suite with checks enabled, and it is clean.

**Verification:**

- [x] `cargo build -p iscc-jni && mvn … clean test` exits 0: surefire reports `tests="103"`, 0
    failures/errors/skipped. That is 19 seq cases (3×4 dynamic + 7 static). Both sections hold 4
    vectors
- [x] `cargo clippy -p iscc-jni -- -D warnings` exits 0. `cargo clippy --workspace --all-targets` is
    clean too
- [x] `IsccLib.java:288/300` declares `byte[] encodeSeq(String[] units)` /
    `String[] decodeSeq(byte[] data)`. `lib.rs:828/857` defines both `Java_…_encodeSeq/decodeSeq`
- [x] `IsccLibTest.java` reads `encode_seq` and `decode_seq` and contains `seqByteRoundtrip`, the
    "at least one ISCC-UNIT" assertion and the `"\0junk"` negative test
- [x] `mise run check` exits 0, with a clean tree afterwards
- Probes (3): (1) `mvn -X` shows `-DargLine` never reaches the forked JVM, because the pom's
    explicit `<argLine>` wins; (2) `JAVA_TOOL_OPTIONS=-Xcheck:jni` full suite: 103 pass, 0
    `WARNING in native method`; (3) padding control: `ISCC:EEAYYNMFG5XOELK2` encodes and the `====`
    form is rejected ("invalid symbol at 16"), so the test fails for the intended reason

**Issues found:**

- Filed `normal`: JNI exports turn a pending JVM exception (OOM) into `IllegalArgumentException`.
    This is the crate-wide `throw_and_default` shape that next.md asked advance to copy, so it is
    not a seq regression
- The `-DargLine="-Xcheck:jni …"` command (in next.md, learnings and review memory since 168) is a
    no-op, so every earlier "zero JNI warnings" claim was vacuous. Learnings and memory are now
    corrected to `JAVA_TOOL_OPTIONS=-Xcheck:jni mvn -f crates/iscc-jni/java/pom.xml test`
- The NUL claim is narrower than the handoff said. Only the `String[]` path (`encodeSeq`) is pinned
    by a committed test. `genTextCodeV0("Hello\0World")` not truncating rests on advance's `/tmp`
    probe alone
- Non-blocking checks: `lib.rs` is 1254 lines (split threshold ~1500), and the pom sets
    `sourceEncoding` UTF-8, so the literal `ſ` in the test source is safe on Windows

**Codex review:** (P2) with OOM, `IllegalArgumentException: JNI call failed`. Valid but crate-wide,
so filed as an issue (see above); the 2026-07-28 decision does not settle it. (P3) the surefire
`-DargLine` command does not enable `-Xcheck:jni`. Confirmed, and fixed in learnings/memory.

**Next:** Ruby seq (`crates/iscc-rb`), then UniFFI (Swift/Kotlin hard-code per-section conformance
counts). Keep asking each one for a `unit + "\0junk"` negative test. Then the 33→35 doc sweep, which
must also cover `crates/iscc-jni/README.md`, `docs/howto/java.md`, `docs/java-api.md` and the
`crates/iscc-jni/CLAUDE.md` test-pattern list (no `bytes:` helper or seq tests there yet).
**define-next: stop writing `-DargLine="-Xcheck:jni …"` into next.md.** Use the `JAVA_TOOL_OPTIONS`
form. Gate candidate: CI runs no `-Xcheck:jni` JNI suite at all. Note that `-Xcheck:jni` warnings do
not fail tests, so a gate would have to grep for them.

**Notes:** This is a binding-only step with no core or hot-path change, so iai and CRAP are not
affected. The open `normal` queue still includes the Python `encode_seq` docstring, the wasm npm
metadata, two Dependabot lock refreshes, Ruby locale, napi Buffer leak, FFI split, .NET pinning
dedupe, the .NET/C++ NUL design issue (needs Titusz) and the JNI pending-exception issue (new).
