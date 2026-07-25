# Handoff

## 2026-07-25 — Dependency refresh slice 5 — JVM binding manifests (pom.xml + build.gradle.kts)

**Done:** Refreshed all third-party pins in the two JVM manifests to the exact versions specified in
next.md — 7 bumps in `pom.xml` (junit-jupiter 5.14.4, gson 2.14.0, compiler 3.15.0, surefire 3.5.6,
source 3.4.0, javadoc 3.12.0, gpg 3.2.8) with `central-publishing-maven-plugin` held at 0.7.0 under
a documented `held:` XML comment, and 4 bumps in `build.gradle.kts` (Kotlin JVM plugin 2.4.10 — no
fallback ladder needed, JNA 5.19.1, junit/gson in lockstep with pom.xml) plus a `// held:` note
deferring JUnit 6.x. Doc version strings synced in 5 files.

**Files changed:**

- `crates/iscc-jni/java/pom.xml`: 7 pins bumped; `held:` comment above the 0.7.0
    central-publishing-maven-plugin pin
- `packages/kotlin/build.gradle.kts`: kotlin("jvm") 2.1.10→2.4.10, jna 5.16.0→5.19.1, junit-jupiter
    5.11.4→5.14.4, gson 2.11.0→2.14.0; **one deviation (see Notes):** added
    `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.14.4")` — required to make the
    scoped junit bump pass under Gradle 8.12.1
- `README.md`, `packages/kotlin/README.md`, `docs/howto/kotlin.md`: jna:5.16.0 → jna:5.19.1 (1 each)
- `crates/iscc-jni/CLAUDE.md`: test-dep line now reads junit-jupiter 5.14.4, gson 2.14.0
- `packages/kotlin/CLAUDE.md`: jna:5.16.0 → 5.19.1 (line 12 — an occurrence next.md's doc list
    missed; docs are uncounted, same class of edit)

**Verification:**

- All 9 target versions pre-checked as existing on `repo1.maven.org` (HTTP 200 on each `.pom`)
- `cargo build -p iscc-jni` + `mvn test -f crates/iscc-jni/java/pom.xml` → exit 0, **Tests run: 69,
    Failures: 0, Errors: 0, Skipped: 0** (surefire auto-resolved junit-platform-launcher 1.14.4)
- `mvn -Prelease package -DskipTests` → exit 0; sources + javadoc jars built by the bumped
    source-plugin 3.4.0 / javadoc-plugin 3.12.0; gpg 3.2.8 resolved without signing
- `cargo build -p iscc-uniffi` + `./gradlew test` (from packages/kotlin) → exit 0 after the launcher
    fix; results XML confirms **tests="9" failures="0" errors="0"** (ConformanceTest) — first run
    failed, see Notes
- pom greps: bumped-pin count **7**, `<version>0.7.0</version>` count **1**, `held:` count **1**
- gradle greps: the 3 pin lines print exactly; `held:` count **1**
- Docs greps: `jna:5\.16\.0` in the 3 listed docs → exit 1 (nothing); `jna:5\.19\.1` → 1 each;
    `5\.11\.4\|2\.11\.0` in `crates/iscc-jni/CLAUDE.md` → exit 1 (nothing)
- `git status --porcelain packages/kotlin/src packages/kotlin/gradle` → empty (generated bindings +
    wrapper untouched)
- `mise run version:check` → exit 0; `mise run check` → exit 0, all 15 hooks pass, no files
    rewritten

**Next:** Remaining refresh slices per issues.md: rb/go/napi/dotnet manifests (each CI-exercised on
develop pushes, so verifiable the same way as this slice), then the deliberately deferred items —
release.yml GHA refs (bundle with the re-trigger fix near a release), ruff 0.16 adoption, magnus 0.8
/ jni 0.22 majors. The Gradle wrapper (8.12.1) and JUnit 6.x remain their own future steps; note
JUnit 6's Kotlin ≥ 2.2 prerequisite is now satisfied by this slice's 2.4.10.

**Notes:**

- **Deviation, in-scope file:** the first `./gradlew test` run on junit 5.14.4 failed —
    `JUnitException: OutputDirectoryCreator not available; probably due to unaligned versions of the   junit-platform-engine and junit-platform-launcher jars`.
    Root cause: jupiter 5.14.4 brings platform-engine 1.14.4, but Gradle 8.12.1 injects its own
    bundled launcher, which predates platform 1.12 (where `OutputDirectoryCreator` appeared). Fix is
    the JUnit-user-guide-recommended explicit
    `testRuntimeOnly("org.junit.platform:junit-platform-launcher:1.14.4")` (pinned, with a lockstep
    comment — 1.x.y follows 5.x.y). This is the exact mechanism next.md's Not-In-Scope predicted for
    JUnit 6, needed already at 5.14.x. It is a new dependency line in the in-scope file, not a
    build-structure change; Maven needs nothing because surefire resolves the aligned launcher
    itself (visible in the mvn log). Consequently I reworded the `// held:` JUnit 6 comment to cite
    the remaining blockers (artifact renumbering, removed Platform APIs) instead of the launcher
    question, which is now settled.
- Kotlin 2.4.10 compiled the UniFFI-generated bindings and tests cleanly on the first try — the
    fallback ladder (2.3.21 → 2.2.21 → 2.1.21) was never exercised; no `held:` needed on the kotlin
    line.
- Gradle prints a deprecation warning ("Deprecated Gradle features … incompatible with Gradle 9.0")
    — pre-existing, also present before this change; relevant to the future wrapper-major step, not
    this one.
- `~/.m2` was cold; mvn downloaded everything from Central without issue. Total local verification
    ~3 min.
- Not staged: `.claude/context/iterations.jsonl` (runner-owned).
