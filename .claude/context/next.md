# Next Work Package

## Step: Add Kotlin CI job to ci.yml

## Goal

Add a `kotlin` job to `.github/workflows/ci.yml` that builds the UniFFI native library and runs
Kotlin conformance tests via Gradle, bringing CI coverage to 16/16 jobs. This gates the Kotlin
binding quality in CI — tests exist locally but are not yet enforced on every push.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` (add `kotlin` job)
- **Reference**: `.github/workflows/ci.yml` (existing `swift` and `java` jobs as patterns),
    `packages/kotlin/build.gradle.kts` (native lib path config)

## Not In Scope

- Version sync (adding `gradle.properties` to `version_sync.py`) — separate step
- Documentation (`docs/howto/kotlin.md`, README Kotlin sections) — separate step
- Release workflow (`maven-kotlin` in `release.yml`) — separate step
- Removing `mavenLocal()` from `build.gradle.kts` — harmless, CI resolves from `mavenCentral()`
- Bumping Gson from 2.8.9 to 2.11.0 — not needed for CI functionality
- Modifying `build.gradle.kts` — it already has correct `LD_LIBRARY_PATH` and `java.library.path`

## Implementation Notes

Model the Kotlin job after the Swift CI job (both use UniFFI) but run on `ubuntu-latest` (Kotlin/JVM
doesn't need macOS). Use JDK 17 setup from the existing Java/JNI job.

Add the job between `swift` and `bench` to maintain logical grouping:

```yaml
kotlin:
  name: Kotlin (gradle build, test)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - uses: actions/setup-java@v4
      with:
        distribution: temurin
        java-version: '17'
    - name: Build UniFFI native library
      run: cargo build -p iscc-uniffi
    - name: Run Gradle tests
      run: ./gradlew test
      working-directory: packages/kotlin
```

Key details:

- Build `iscc-uniffi` (NOT `iscc-ffi` or `iscc-jni`) — Kotlin uses UniFFI bindings
- `build.gradle.kts` sets `nativeLibDir = "${rootProject.rootDir}/../../target/debug"` — this
    resolves correctly because `working-directory: packages/kotlin` makes `rootProject.rootDir` =
    `packages/kotlin`, so `../../target/debug` = repo-root `target/debug/`
- `LD_LIBRARY_PATH` and `jna.library.path` are already configured in `build.gradle.kts` task config
- No Gradle wrapper caching action needed — Gradle wrapper downloads are fast and the project is
    small (~1 test class)
- Use `./gradlew test` (not `./gradlew build`) — tests include compilation

## Verification

- `grep -c 'kotlin:' .github/workflows/ci.yml` returns at least 1
- `grep 'cargo build -p iscc-uniffi' .github/workflows/ci.yml` finds the build step
- `grep 'gradlew test' .github/workflows/ci.yml` finds the test step
- `grep 'java-version.*17' .github/workflows/ci.yml` matches at least 2 (existing java + new kotlin)
- YAML is valid: `python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0

## Done When

All verification criteria pass — the `kotlin` job is defined in `ci.yml` with UniFFI native library
build, JDK 17 setup, and Gradle test execution.
