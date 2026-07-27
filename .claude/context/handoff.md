# Handoff

## 2026-07-27 — Review of: Bump the Kotlin package's Gradle wrapper from 8.12.1 to 9.6.1

**Verdict:** PASS

**Summary:** The wrapper is on Gradle 9.6.1, `clean build` is green with the same 9 + 13 test cases,
and the KGP "Deprecated Gradle Version" diagnostic is gone. Provenance is independently established:
the committed `gradle-wrapper.jar` matches Gradle's published wrapper checksum, and a fresh
regeneration in a throwaway directory reproduced all four wrapper artifacts byte-for-byte. Every
Not-In-Scope item held (KGP still 2.4.10, `pom.xml` byte-untouched, no `distributionSha256Sum`, no
workflow edits).

**Verification:**

- [x] `gradle-wrapper.properties` contains `gradle-9.6.1-bin.zip`;
    `gradlew -p packages/kotlin --version` → `Gradle 9.6.1`, launcher JVM 17.0.19
- [x] `cargo build -p iscc-uniffi` → 0, `gradlew -p packages/kotlin clean build` → 0, BUILD
    SUCCESSFUL, 8 actionable tasks executed
- [x] `TEST-…ConformanceTest.xml` `tests="9" skipped="0" failures="0" errors="0"`;
    `TEST-…UnicodeBoundaryTest.xml` `tests="13" skipped="0" failures="0" errors="0"`
- [x] `grep -in deprecat` over the captured `clean build` log → zero matches (no KGP diagnostic, no
    residual "Deprecated Gradle features" notice)
- [x] `grep -rn "Gradle 8" packages/kotlin/CLAUDE.md packages/kotlin/build.gradle.kts` → no match; a
    repo-wide sweep for `Gradle 8|gradle-8|8.12.1` finds nothing outside agent memory/context
- [x] `mise run check` → exit 0, 18 hooks passed, no tracked file modified

**Probes beyond next.md (3):**

- **Jar provenance:** committed blob sha256 `497c8c2a…a9c7` equals
    `services.gradle.org/distributions/gradle-9.6.1-wrapper.jar.sha256`. A binary blob is the one
    thing no reviewer can eyeball; it is now pinned to the publisher's own checksum.
- **Independent regeneration:** `gradlew -p /tmp/wraptest wrapper --gradle-version 9.6.1` (needs a
    `settings.gradle.kts` in the target dir, else the task fails) reproduced `gradlew`,
    `gradle-wrapper.properties` and the jar identically, and `gradlew.bat` identically after
    LF-normalizing the 82 CRLF lines Gradle emits. The `retries`/`retryBackOffMs` keys really are
    tool-generated defaults, not hand-added.
- **Release-only publish path:** `gradlew publishMavenPublicationToStagingRepository` → exit 0 under
    Gradle 9; pom + Gradle module metadata + jar/sources/javadoc staged with md5/sha1/sha256/sha512,
    `signMavenPublication` correctly SKIPPED without `MAVEN_GPG_PASSPHRASE`. `release.yml` is never
    exercised by CI, so this closes the only unverified Gradle-9 surface.

**Issues found:**

- (none filed) Scope note: the diff touches 5 non-test/non-doc files, 4 of which are the single
    `wrapper` task's output that next.md enumerated as "regenerated … not hand-edited". That is a
    fan-out in substance — one mechanical change, zero per-file decisions, verified by regeneration
    — so it is not scope creep, but next.md should use the literal `**Fan-out:**` label for a
    tool-generated file set so the >3-file cap resolves without a judgment call.
- `packages/kotlin/.kotlin/sessions` (KGP session dir) is untracked and not covered by
    `packages/kotlin/.gitignore`. It only materializes while a build runs, so it can make a
    concurrent `git status` / "no file modified" assertion look dirty. Cosmetic, pre-existing.
- Pre-existing, unchanged: `gradlew.bat` is stored LF per `.gitattributes` + the `mixed-line-ending`
    hook although Gradle emits CRLF. The 9.6.1 script's flow now ends in `call :exitWithErrorLevel`
    at EOF; Windows is never exercised by CI, so if a Windows contributor reports a broken
    `gradlew.bat`, an `*.bat text eol=crlf` attribute is the first thing to try.

**Codex review:** No findings. It independently verified that the wrapper artifacts match the
official 9.6.1 release and that the package builds and tests clean on JDK 17.

**Next:** JUnit 6.x migration — the last JVM item in "Dependency review and refresh". It spans
`packages/kotlin/build.gradle.kts` (junit-jupiter 5.14.4 → 6.x **and** junit-platform-launcher
1.14.4 → 6.x, which stops being a 1.x number) and `crates/iscc-jni/java/pom.xml`. Two build systems,
so define-next should decide whether that is one step or two; the Gradle side is now on a supported
base, so any launcher-injection failure there is attributable to JUnit alone. Expected totals: 9 and
13 test cases (Kotlin), JNI Maven suite unchanged.

**Notes:**

- Gradle 9 writes `build/reports/problems` at the *end* of every build, so a concurrent build makes
    `clean` fail with "Unable to delete directory … New files were found". My first run hit exactly
    this against the background Codex review; re-running sequentially was green. Not a defect —
    re-run before believing a Gradle failure on this bind mount.
- Two benign Gradle 9 notices remain in build output: an `[Incubating] Problems report` line and a
    configuration-cache suggestion. Configuration cache is deliberately out of scope.
- `compileKotlin` still warns "Expression is unused" at `iscc_uniffi.kt:1042/1045` — pre-existing,
    in the UniFFI-generated file, unchanged by this bump.
- No Rust source, public API or benchmarked path touched; CRAP and iai baselines correctly unmoved.
    CI has not yet run this sha — the `kotlin` job (`./gradlew test`, temurin 17, fresh checkout)
    downloads 9.6.1 on its first run.
