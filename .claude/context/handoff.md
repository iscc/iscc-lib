# Handoff

## 2026-07-27 — Review of: Migrate both JVM test suites from JUnit 5.14.4 to JUnit 6.1.2

**Verdict:** PASS

**Summary:** JUnit moved to 6.1.2 in both JVM manifests (`junit-jupiter` 5.14.4 → 6.1.2 in Gradle
and Maven, `junit-platform-launcher` 1.14.4 → 6.1.2 under JUnit 6's unified numbering), with the
stale `// held:` and lockstep comments rewritten and five doc/javadoc mentions updated. Two non-test
non-doc files touched, no test-logic change, every suite green at its fixture-derived case total.

**Verification:**

- [x] `grep -rn "5\.14\.4\|1\.14\.4\|held: JUnit"` over both manifests → no match; pins at
    `build.gradle.kts:22/27` and `pom.xml:48` are `6.1.2`
- [x] `cargo build -p iscc-uniffi` + `packages/kotlin/gradlew -p packages/kotlin clean test` → exit
    0; `ConformanceTest.xml tests="9"`, `UnicodeBoundaryTest.xml tests="13"`, both
    `skipped="0" failures="0" errors="0"`
- [x] `cargo build -p iscc-jni` + `mvn test` → exit 0,
    `Tests run: 82, Failures: 0, Errors: 0, Skipped: 0` (69 + 13), equal to the pre-bump total the
    advance agent recorded
- [x] `grep -rn "JUnit 5\|5\.14\.4\|1\.14\.4" crates/iscc-jni/CLAUDE.md packages/kotlin/CLAUDE.md` →
    no match
- [x] `git status --porcelain -- .crap-baseline.json .iai-baseline.json` → empty
- [x] `mise run check` → exit 0, no tracked file modified

**Extra probes (3):**

- `gradlew dependencies --configuration testRuntimeClasspath` — every resolved JUnit artifact
    (jupiter/api/params/engine, platform commons/engine/launcher) is `6.1.2`; no 1.x remnant
- **Anti-collapse oracle derived from the fixtures, not from a second run of the same tree:**
    surefire XML dynamic-case counts equal `data.json` per-function vector counts exactly
    (20+5+3+5+3+2+4+3+5 = 50) + 19 static `@Test` = 69; boundary 7+5 dynamic + 1 static = 13
- `generatePomFileForMavenPublication` → the published Kotlin POM contains **zero** junit entries,
    confirming the "nothing consumer-visible changes" claim (Maven side is `<scope>test</scope>`)

Also re-ran Maven as `mvn clean test`: the criterion run reported "Nothing to compile", i.e. it
reused test classes compiled earlier — a cold `clean test` recompiles both source sets against JUnit
6 (`javac [debug target 17]`) and still yields 82/0/0/0. CI (`temurin 17`, `mvn test -f …`,
`./gradlew test`) matches the local invocations; JUnit 6's Java 17 baseline is met.

**Issues found:**

- (nit, out of scope by next.md) `.claude/context/specs/java-bindings.md:42` still reads
    "IsccLibTest.java # JUnit 5 conformance tests". It is a human-owned sub-spec and a descriptive
    file-tree comment, not a normative requirement — no issue filed; Titusz can fix the word.
- No gate weakening anywhere in `@{upstream}..HEAD` (no diff under `.pre-commit-config.yaml`,
    `.github/`, `mise.toml`, `pyproject.toml`, `deny.toml`, `Cargo.toml`); no suppressions, skips or
    threshold moves; no Rust/Python/API surface touched, so baselines and SemVer are untouched.

**Codex review:** No actionable findings — "Both Maven and Gradle resolve aligned JUnit 6.1.2
dependencies and pass their complete test suites on the CI-targeted JDK 17. No actionable regression
was identified."

**Next:** `jni` 0.22 (`crates/iscc-jni/src/lib.rs`, `JNIEnv` → `Env`/`EnvUnowned` per upstream
`docs/0.22-MIGRATION.md`; ~41 `JNIEnv|GlobalRef|AutoLocal` sites). It is now cleanly bisectable —
the Java-side framework is settled and green at 82 cases, so any JVM-side red is attributable to the
Rust bridge. `magnus` 0.8 (`crates/iscc-rb/src/lib.rs`, ~5 `exception::runtime_error` sites) is the
smaller alternative if a lower-risk step is wanted first. One major per step, no bundling.

**Notes:**

- Maven case totals for the ledger: **82 before, 82 after** (69 IsccLibTest + 13
    UnicodeBoundaryTest), zero skips; Kotlin unchanged at 9 + 13. The totals are now backed by a
    fixture-derived expectation, not just by equality with a prior run.
- Surefire 3.5.6 auto-resolved the aligned 6.1.2 launcher; no explicit pom launcher dependency was
    needed, and no plugin pin moved.
- The Gradle wrapper lives at `packages/kotlin/gradlew` — a bare `./gradlew …` from the repo root
    does not exist. next.md's verification text had it wrong; the advance agent's substitution was
    correct and is the same invocation CI uses via `working-directory`.
- Only two Rust dependency majors remain in the authorized refresh (`jni` 0.22, `magnus` 0.8) plus
    the release-only `release.yml` action bumps; issues.md was rewritten to that smaller problem.
