# Next Work Package

## Step: Migrate both JVM test suites from JUnit 5.14.4 to JUnit 6.1.2

## Goal

Close the last manifest-level item of the authorized dependency-majors refresh: move JUnit to 6.1.2
in both JVM build systems, where the platform artifacts renumber 1.x -> 6.x. Cites the `normal`
issue "Dependency review and refresh across the project".

## Alternatives Considered

- **Chosen:** JUnit 6.x across both JVM manifests — one dependency major, 2 non-doc files, fully
    verifiable here (JDK 17, Gradle 9.6.1 wrapper, Maven 3.8.7), and with the wrapper now on a
    supported base any launcher/discovery failure is attributable to JUnit alone. Splitting Gradle
    and Maven into two steps was rejected: identical bump, both locally gated, serialising doubles
    the cost for no added safety.
- **Rejected:** the `jni` 0.22 migration — a wholesale `JNIEnv` -> `Env`/`EnvUnowned` rewrite of
    `crates/iscc-jni/src/lib.rs`. Changing that crate's Java test framework and its Rust bridge in
    overlapping steps makes a JVM-side failure un-bisectable; take it once this lands.

## Scope

- **Modify**: `packages/kotlin/build.gradle.kts`, `crates/iscc-jni/java/pom.xml`,
    `packages/kotlin/CLAUDE.md` (line 62 "JUnit 5"), `crates/iscc-jni/CLAUDE.md` (lines 30/80/86/87)
- **Reference**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`,
    `.github/workflows/ci.yml` (`java` and `kotlin` jobs)

## Not In Scope

- Bumping any other JVM pin: `maven-surefire-plugin` 3.5.6 is already latest and JUnit 6 only drops
    support for Surefire < 3.0.0; KGP, JNA, gson and the held `central-publishing-maven-plugin`
    0.7.0 all stay.
- `.claude/context/specs/java-bindings.md` (human-owned spec) and `issues.md` (review owns it).
- Adopting new JUnit 6 features (CancellationToken, `suspend` test methods, `kotlin-test`,
    `@CsvSource`) or restructuring any test class — this is a version bump, not a rewrite.
- `jni` 0.22 / `magnus` 0.8 migrations, Gradle configuration cache, `distributionSha256Sum`.

## Implementation Notes

- Latest stable, verified from `repo1.maven.org` metadata while scoping:
    `org.junit.jupiter:junit-jupiter` **6.1.2** and `org.junit.platform:junit-platform-launcher`
    **6.1.2** — JUnit 6 gives Platform, Jupiter and Vintage a single version number, so the launcher
    is no longer a 1.x number.
- Update the launcher comment in `build.gradle.kts` accordingly (it currently says "keep the 1.x.y
    version in lockstep with 5.x.y") and delete the `// held: JUnit 6.x deferred ...` comment above
    the jupiter dependency.
- **No floor moves.** JUnit 6 baselines are Java 17 and Kotlin 2.2 (6.0.0 release notes); local JDK
    is 17.0.19, CI is temurin 17, KGP is 2.4.10. Both artifacts are test-scoped — Gradle publishes
    from `components["java"]` (no `testImplementation`/`testRuntimeOnly`) and the pom entries carry
    `<scope>test</scope>` — so nothing consumer-visible changes.
- Maven: Surefire auto-adds a launcher aligned to the platform version it finds on the test
    classpath. If provider selection or discovery fails, the fallback is an explicit
    `org.junit.platform:junit-platform-launcher:6.1.2` test-scoped dependency in the same pom — not
    a plugin bump. `~/.m2` is empty, so the first `mvn` run downloads (network works).
- **Record the pre-bump Maven total first**: run `mvn test -f crates/iscc-jni/java/pom.xml` on the
    unmodified tree, note the `Tests run: N` summary, then edit. `IsccLibTest` builds its
    conformance cases with `@TestFactory`/`DynamicTest`, and a silent collapse to one case per
    factory method is the failure mode a bare "green" hides. Put N (before and after) in the handoff
    Notes.
- Removed-in-6 modules (`junit-platform-runner`, `junit-platform-jfr`) and the ConsoleLauncher
    changes are unused here; the suites only use `@Test`, `@TestFactory`, `DynamicTest`,
    `@BeforeAll` and `org.junit.jupiter.api.Assertions.*`, all still present. Expect no test-source
    change beyond the "Uses JUnit 5 ..." javadoc line in `IsccLibTest.java` (test file, free of the
    file budget).
- Gradle on this bind mount: if `clean` fails with "Unable to delete directory ... New files were
    found", a concurrent build wrote `build/reports/problems` — re-run sequentially before believing
    it. Run `mise run format` before `git add` (the two `CLAUDE.md` edits go through mdformat).
- No Rust source, public API or benchmarked path is touched: the CRAP and iai baselines must stay
    byte-untouched.

## Verification

- `grep -rn "5\.14\.4\|1\.14\.4\|held: JUnit" packages/kotlin/build.gradle.kts crates/iscc-jni/java/pom.xml`
    → no match, and both files pin `6.1.2`.
- `cargo build -p iscc-uniffi` then `./gradlew -p packages/kotlin clean test` exit 0, with
    `TEST-...ConformanceTest.xml` reporting `tests="9"` and `TEST-...UnicodeBoundaryTest.xml`
    `tests="13"`, each `skipped="0" failures="0" errors="0"`.
- `cargo build -p iscc-jni` then `mvn test -f crates/iscc-jni/java/pom.xml` exit 0, with a
    `Tests run: N, Failures: 0, Errors: 0, Skipped: 0` summary whose N equals the total recorded
    from the unmodified tree before the edit.
- `grep -rn "JUnit 5\|5\.14\.4\|1\.14\.4" crates/iscc-jni/CLAUDE.md packages/kotlin/CLAUDE.md` → no
    match.
- `git status --porcelain -- .crap-baseline.json .iai-baseline.json` is empty.
- `mise run check` exits 0 and modifies no tracked file.

## Done When

Both JVM manifests pin JUnit 6.1.2, both suites pass at their unchanged case totals, the docs no
longer say JUnit 5, and `mise run check` is clean.
