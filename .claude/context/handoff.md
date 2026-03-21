## 2026-03-21 — Create Kotlin JVM project scaffold with UniFFI-generated bindings

**Done:** Created `packages/kotlin/` with a Gradle JVM project containing UniFFI-generated Kotlin
bindings. The generated `iscc_uniffi.kt` (3217 lines) provides all 32 Tier 1 symbols via JNA.
`./gradlew compileKotlin` compiles successfully.

**Files changed:**

- `packages/kotlin/build.gradle.kts`: Kotlin JVM plugin, JNA 5.16.0 dependency, JUnit test config
    with `java.library.path` pointing to `target/debug`
- `packages/kotlin/settings.gradle.kts`: Root project name `iscc-lib-kotlin`
- `packages/kotlin/gradle.properties`: Version 0.3.1, official Kotlin code style
- `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`: UniFFI-generated Kotlin
    bindings (3217 lines, package `uniffi.iscc_uniffi`)
- `packages/kotlin/gradlew`, `gradlew.bat`, `gradle/wrapper/`: Gradle 8.12.1 wrapper
- `packages/kotlin/.gitignore`: Excludes `.gradle/` (the `build/` pattern already covered by root
    `.gitignore`)

**Verification:**

- `test -d packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi` — PASS
- `test -f packages/kotlin/build.gradle.kts` — PASS
- `test -f packages/kotlin/gradlew` — PASS
- `./gradlew compileKotlin` — PASS (BUILD SUCCESSFUL)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- `mise run check` — 15/15 hooks pass

**Next:** Add Kotlin conformance tests (the next step in the Kotlin bindings issue). This would
involve a `src/test/kotlin/` directory with JUnit 5 tests running against `data.json` vectors,
requiring the native `libiscc_uniffi.so` at test time. After that: CI job, version sync integration,
documentation.

**Notes:**

- The generated `iscc_uniffi.kt` is UniFFI output — should not be manually modified
- The `gradle-wrapper.jar` (~43KB binary) is committed per Gradle convention
- `build/` is already covered by the root `.gitignore` pattern; only `.gradle/` needed a local
    `.gitignore` entry
- No tests in this step (per next.md scope). Tests come in the subsequent step
- Kotlin version 2.1.10, JNA 5.16.0, Gradle 8.12.1 — all current stable versions
