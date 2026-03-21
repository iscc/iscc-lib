## 2026-03-21 — Add Kotlin CI job to ci.yml

**Done:** Added a `kotlin` job to `.github/workflows/ci.yml` between `swift` and `bench`. The job
runs on `ubuntu-latest`, sets up JDK 17 (Temurin), builds the UniFFI native library, and runs Gradle
tests against the Kotlin conformance test suite.

**Files changed:**

- `.github/workflows/ci.yml`: Added `kotlin:` job (lines 253-268) with 6 steps: checkout, rust
    toolchain, rust cache, setup-java (temurin 17), `cargo build -p iscc-uniffi`, `./gradlew test`

**Verification:**

- `grep -c 'kotlin:' ci.yml` → 1 (pass)
- `grep 'cargo build -p iscc-uniffi' ci.yml` → 2 matches: swift + kotlin (pass)
- `grep 'gradlew test' ci.yml` → 1 match (pass)
- `grep -c 'java-version.*17' ci.yml` → 2 matches: java + kotlin (pass)
- YAML validation → exits 0 (pass)
- `mise run check` → all 15 hooks pass

**Next:** Version sync — add `packages/kotlin/gradle.properties` to `scripts/version_sync.py` so
Kotlin package version stays in sync with the workspace. Then documentation (`docs/howto/kotlin.md`,
README Kotlin sections) and release workflow.

**Notes:** Straightforward addition. The `build.gradle.kts` already has correct `LD_LIBRARY_PATH`,
`jna.library.path`, and `java.library.path` configuration, so no modifications to the Kotlin project
were needed. CI job count is now 16 (version-check + 15 language/tool jobs).
