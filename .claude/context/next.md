# Next Work Package

## Step: Dependency refresh slice 5 — JVM binding manifests (pom.xml + build.gradle.kts)

## Goal

Refresh the third-party pins in the two JVM binding manifests (`crates/iscc-jni/java/pom.xml` and
`packages/kotlin/build.gradle.kts`) to current stable versions, keeping the shared
junit-jupiter/gson pins in lockstep across both files. This continues the `normal` `[human]` issue
"Dependency review and refresh across the project" (slices 1-4 done), and both files are fully
exercised on every develop push by the `Java (JNI build, mvn test)` and
`Kotlin (gradle build, test)` CI jobs — plus locally, since this devcontainer has JDK 17 + Maven
3.8.7 and a warm Gradle wrapper cache.

## Scope

- **Create**: none
- **Modify** (2 non-test, non-doc files — within the 3-file cap):
    - `crates/iscc-jni/java/pom.xml`
    - `packages/kotlin/build.gradle.kts`
- **Modify (docs, not counted)**:
    - `README.md` (line ~151, Kotlin install snippet pins `net.java.dev.jna:jna:5.16.0`)
    - `packages/kotlin/README.md` (line ~14, same JNA snippet)
    - `docs/howto/kotlin.md` (line ~22, same JNA snippet)
    - `crates/iscc-jni/CLAUDE.md` (line ~87, "Test-only dependencies: `junit-jupiter` 5.11.4, `gson`
        2.11.0")
- **Reference**:
    - `.claude/context/issues.md` → "Dependency review and refresh across the project"
    - `.claude/context/specs/ci-cd.md` → "Dependency Freshness" (mandates a documented reason next to
        every deliberately held-back pin)
    - `.claude/context/learnings.md` → CI/CD section (dependency-refresh slicing, hold-back comment
        convention)
    - `packages/kotlin/CLAUDE.md`, `crates/iscc-jni/CLAUDE.md` (build/test commands, JNA + gson
        gotchas)
    - `.github/workflows/ci.yml` → `java:` and `kotlin:` jobs (exact commands CI runs)

## Not In Scope

- **JUnit 6.x migration.** JUnit 6 (latest 6.1.2) renumbers the platform artifacts to 6.x, removes
    deprecated Platform APIs, and requires Kotlin ≥ 2.2 — Gradle 8.12.1's `useJUnitPlatform()` may
    then need an explicit `testRuntimeOnly("org.junit.platform:junit-platform-launcher")`. That is a
    build-structure change, not a version bump. Stay on the 5.x line this step and leave a
    documented hold-back note.
- **`central-publishing-maven-plugin` bump (0.7.0 → 0.11.0).** Its `deploy` goal only runs during a
    real Maven Central publish, so nothing local or in CI can verify it; a regression would surface
    mid-release. Hold it at 0.7.0 with a documented XML comment.
- **Beta/milestone versions** — `maven-compiler-plugin` 4.0.0-beta-4, `maven-source-plugin`
    4.0.0-beta-1, `kotlin-gradle-plugin` 2.4.20-Beta2, `junit-jupiter` 5.13.0-M3 are all
    pre-releases. Use stable releases only (note: Maven Central's `<latest>` metadata field lists
    pre-releases — filter them out).
- **Gradle wrapper version** (`packages/kotlin/gradle/wrapper/gradle-wrapper.properties`, 8.12.1) —
    a build-tool major, its own step.
- **`java-version: '17'`, `maven.compiler.source/target`, `jvmToolchain(...)`** — support-policy
    decisions, not dependency pins. Do not add a toolchain block.
- **Editing the UniFFI-generated
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`** — if a Kotlin plugin bump
    makes generated code fail to compile, step the Kotlin version _down_ (see ladder below); never
    hand-patch generated output.
- **`.claude/context/specs/kotlin-bindings.md`** — its `jna:5.16.0@aar` / `0.4.0` snippets are
    illustrative target prose owned by the human; leave them alone.
- Other refresh slices: `release.yml` action refs, ruff 0.16 adoption, `magnus` 0.8 / `jni` 0.22
    migrations, rb/go/napi/dotnet manifests. One slice per iteration.
- Adding Dependabot/Renovate.

## Implementation Notes

All target versions below were verified against `repo1.maven.org` `maven-metadata.xml` on 2026-07-25
(filter to `<version>` entries matching `^[0-9]+(\.[0-9]+)*$` — the `<latest>` field includes
betas).

### `crates/iscc-jni/java/pom.xml`

| Pin                               | From   | To         |
| --------------------------------- | ------ | ---------- |
| `org.junit.jupiter:junit-jupiter` | 5.11.4 | **5.14.4** |
| `com.google.code.gson:gson`       | 2.11.0 | **2.14.0** |
| `maven-compiler-plugin`           | 3.13.0 | **3.15.0** |
| `maven-surefire-plugin`           | 3.5.2  | **3.5.6**  |
| `maven-source-plugin` (release)   | 3.3.1  | **3.4.0**  |
| `maven-javadoc-plugin` (release)  | 3.11.2 | **3.12.0** |
| `maven-gpg-plugin` (release)      | 3.2.7  | **3.2.8**  |
| `central-publishing-maven-plugin` | 0.7.0  | **hold**   |

- Every bumped plugin declares `<prerequisites><maven>3.6.3</maven></prerequisites>` — satisfied by
    the local Maven 3.8.7 and by CI's Maven. No Maven upgrade needed.
- Do **not** touch `<version>0.5.0</version>` (managed by `scripts/version_sync.py`) or the
    `maven.compiler.*` properties.
- Add an XML comment directly above the `central-publishing-maven-plugin` `<version>` element
    explaining the hold. It must contain the literal token `held:` and say, in substance: 0.11.0 is
    available, but the plugin's `deploy` goal only runs during a real Maven Central publish, so no
    local or CI check can verify a bump — do it with a human-supervised release.

### `packages/kotlin/build.gradle.kts`

| Pin                               | From   | To                                 |
| --------------------------------- | ------ | ---------------------------------- |
| `kotlin("jvm") version`           | 2.1.10 | **2.4.10**                         |
| `net.java.dev.jna:jna`            | 5.16.0 | **5.19.1**                         |
| `org.junit.jupiter:junit-jupiter` | 5.11.4 | **5.14.4** (lockstep with pom.xml) |
| `com.google.code.gson:gson`       | 2.11.0 | **2.14.0** (lockstep with pom.xml) |

- KGP 2.4.0-2.4.10 officially supports Gradle **7.6.3–9.5.0** (kotlinlang.org compatibility table,
    checked today), so the checked-in Gradle 8.12.1 wrapper is fine — no wrapper change required.
- **Kotlin fallback ladder:** if `./gradlew test` fails to compile with 2.4.10, step down to
    **2.3.21**, then **2.2.21**, then **2.1.21**, taking the highest version that builds and tests
    green. If you land below 2.4.10, add a `// held:` comment on the `kotlin("jvm")` line naming the
    concrete error. Do not disable warnings, add `-Xsuppress-*` flags, or edit generated code to
    force a higher version through.
- Add a short `// held:` comment next to the junit-jupiter line noting JUnit 6.x is deferred (needs
    a `junit-platform-launcher` runtime dependency evaluation under Gradle 8.12.1).

### Docs sync

Replace `net.java.dev.jna:jna:5.16.0` with the new JNA version in `README.md`,
`packages/kotlin/README.md`, and `docs/howto/kotlin.md`; update the junit/gson version numbers in
the "Test-only dependencies" line of `crates/iscc-jni/CLAUDE.md`. These are plain version-string
edits — no prose restructuring.

### Local verification workflow

```bash
cargo build -p iscc-jni
mvn test -f crates/iscc-jni/java/pom.xml
mvn -Prelease package -DskipTests -f crates/iscc-jni/java/pom.xml   # exercises source+javadoc plugins
cargo build -p iscc-uniffi
./gradlew test    # run from packages/kotlin (as CI does)
```

`~/.m2` is empty in this container, so the first `mvn` run downloads its dependencies (network is
available). `maven-gpg-plugin` binds to the `verify` phase, so `-Prelease package` resolves it
without attempting to sign. No Rust source changes here, so `.crap-baseline.json` does **not** need
a refresh.

## Verification

- `cargo build -p iscc-jni && mvn test -f crates/iscc-jni/java/pom.xml` exits 0 (all JUnit
    conformance + unit tests pass, zero failures/errors)
- `mvn -Prelease package -DskipTests -f crates/iscc-jni/java/pom.xml` exits 0 (release-profile
    plugins resolve; sources + javadoc jars build)
- `cargo build -p iscc-uniffi` then `./gradlew test` run from `packages/kotlin` exits 0
- `grep -cE '<version>(5\.14\.4|2\.14\.0|3\.15\.0|3\.5\.6|3\.4\.0|3\.12\.0|3\.2\.8)</version>' crates/iscc-jni/java/pom.xml`
    → **7**
- `grep -c '<version>0.7.0</version>' crates/iscc-jni/java/pom.xml` → **1** and
    `grep -c 'held:' crates/iscc-jni/java/pom.xml` → **≥ 1** (the documented central-publishing
    hold-back)
- `grep -E 'jna:5\.19\.1|junit-jupiter:5\.14\.4|gson:2\.14\.0' packages/kotlin/build.gradle.kts`
    prints 3 lines; `grep -c 'held:' packages/kotlin/build.gradle.kts` → **≥ 1**
- `grep -rn 'jna:5\.16\.0' README.md packages/kotlin/README.md docs/howto/kotlin.md` prints nothing
    (exit 1); `grep -rc 'jna:5\.19\.1' README.md packages/kotlin/README.md docs/howto/kotlin.md` →
    **1** each
- `grep -n '5\.11\.4\|2\.11\.0' crates/iscc-jni/CLAUDE.md` prints nothing (exit 1)
- `git status --porcelain packages/kotlin/src packages/kotlin/gradle` prints nothing (generated
    bindings and the Gradle wrapper are untouched)
- `mise run version:check` exits 0 (pom/gradle version strings unchanged)
- `mise run check` exits 0 (all pre-commit hooks pass, nothing left rewritten)

## Done When

`mvn test`, `mvn -Prelease package -DskipTests`, and `./gradlew test` all pass on the refreshed JVM
manifests, the pin/hold-back/doc-sync greps above return the stated results, and `mise run check` is
clean.
