# Handoff

## 2026-07-27 — Migrate both JVM test suites from JUnit 5.14.4 to JUnit 6.1.2

**Done:** Bumped JUnit to 6.1.2 in both JVM manifests — `junit-jupiter` 5.14.4 → 6.1.2 in the Kotlin
Gradle build and the JNI Maven pom, plus `junit-platform-launcher` 1.14.4 → 6.1.2 (JUnit 6 unifies
the Platform version number with Jupiter). Removed the `// held: JUnit 6.x deferred` comment,
rewrote the launcher-lockstep comment, and updated the four "JUnit 5" doc mentions plus the
`IsccLibTest.java` javadoc line.

**Files changed:**

- `packages/kotlin/build.gradle.kts`: junit-jupiter 6.1.2, junit-platform-launcher 6.1.2; held
    comment deleted; launcher comment now says the launcher version is identical to junit-jupiter
    under JUnit 6 (no more 1.x lockstep)
- `crates/iscc-jni/java/pom.xml`: junit-jupiter `<version>` 5.14.4 → 6.1.2 (only edit)
- `packages/kotlin/CLAUDE.md`: "Uses JUnit 5" → "Uses JUnit 6" (test-patterns bullet)
- `crates/iscc-jni/CLAUDE.md`: lines 30/80/86/87 — "JUnit 5" → "JUnit 6", `junit-jupiter` pin 5.14.4
    → 6.1.2
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: javadoc "Uses JUnit 5" →
    "Uses JUnit 6" (test file, free of the file budget; no other test-source change anywhere)

**Verification:**

- Pre-bump Maven baseline recorded on the unmodified tree (per Implementation Notes):
    `Tests run: 82, Failures: 0, Errors: 0, Skipped: 0` (69 IsccLibTest + 13 UnicodeBoundaryTest)
- `grep -rn "5\.14\.4\|1\.14\.4\|held: JUnit"` over both manifests → no match; both pin `6.1.2`
    (build.gradle.kts lines 22/27, pom.xml line 48)
- `cargo build -p iscc-uniffi` → 0; `packages/kotlin/gradlew -p packages/kotlin clean test` → 0,
    BUILD SUCCESSFUL, 6 actionable tasks; `TEST-…ConformanceTest.xml`
    `tests="9" skipped="0" failures="0" errors="0"`, `TEST-…UnicodeBoundaryTest.xml`
    `tests="13"   skipped="0" failures="0" errors="0"`
- `cargo build -p iscc-jni` → 0; post-bump `mvn test -f crates/iscc-jni/java/pom.xml` → 0,
    `Tests run: 82, Failures: 0, Errors: 0, Skipped: 0` — N identical pre/post (69 + 13), so no
    silent `@TestFactory` collapse
- `grep -rn "JUnit 5\|5\.14\.4\|1\.14\.4" crates/iscc-jni/CLAUDE.md packages/kotlin/CLAUDE.md` → no
    match
- `git status --porcelain -- .crap-baseline.json .iai-baseline.json` → empty (no Rust source
    touched)
- `mise run format` then `mise run check` → exit 0, no tracked file modified by hooks

**Next:** The `jni` 0.22 migration (JNIEnv → Env/EnvUnowned rewrite of `crates/iscc-jni/src/lib.rs`
per upstream `docs/0.22-MIGRATION.md`) is now cleanly bisectable — the Java-side test framework is
settled and green at 82 cases. Alternatively `magnus` 0.8 for the Ruby slice. Both are dedicated
steps per the deps-refresh ledger.

**Notes:**

- Maven case totals for the ledger: **N = 82 before, 82 after** (69 + 13 per class), zero skips both
    runs. Kotlin totals unchanged at 9 + 13.
- Surefire 3.5.6 resolved the aligned 6.1.2 launcher itself — the fallback explicit
    `junit-platform-launcher` pom dependency from the Implementation Notes was NOT needed.
- No floor moves confirmed in practice: JDK 17.0.19 + KGP 2.4.10 built and ran both suites; both
    artifacts stay test-scoped, nothing consumer-visible changes.
- One deviation from next.md's verification text: the Gradle wrapper lives at
    `packages/kotlin/gradlew`, so the literal `./gradlew -p packages/kotlin clean test` fails with
    "no such file" from repo root. Ran `packages/kotlin/gradlew -p packages/kotlin clean test` (same
    invocation CI uses via `working-directory`). Gradle reported 6 actionable tasks (the iter-165
    handoff saw 8 for `clean build`; `clean test` is a smaller task graph — not a red flag).
- No `clean` bind-mount flake this run (no concurrent build).
