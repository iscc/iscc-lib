## 2026-03-21 — Add Kotlin conformance tests for all 9 gen\_\*\_v0 functions

**Done:** Created JUnit 5 conformance tests for the Kotlin JVM package that verify all 9 `gen_*_v0`
functions against the vendored `data.json` test vectors (50 vectors total). Added JUnit 5 and Gson
test dependencies to build.gradle.kts, and fixed native library loading by adding `jna.library.path`
and `LD_LIBRARY_PATH` environment to the test task.

**Files changed:**

- `packages/kotlin/build.gradle.kts`: Added `mavenLocal()` repository, JUnit 5 + Gson test deps,
    `jna.library.path` + `LD_LIBRARY_PATH` for native lib loading
- `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`: New — 9 test methods
    covering all conformance vectors (20+5+3+5+3+2+4+3+5 = 50 vectors)
- `packages/kotlin/src/test/resources/data.json`: New — copied from
    `crates/iscc-lib/tests/data.json`

**Verification:** `./gradlew clean test` produces 9 tests, 0 failures, 0 errors. `mise run check`
passes all 15 hooks. `cargo build -p iscc-uniffi` exits 0. All verification criteria from next.md
met.

**Next:** Add Kotlin CI job to `.github/workflows/ci.yml`, then version sync, docs, and howto guide
for Kotlin.

**Notes:**

- Maven Central was unreachable from the devcontainer, so Gson 2.8.9 (the version cached in
    `~/.gradle/caches`) was used instead of 2.11.0 specified in next.md. It was installed into Maven
    local repository (`~/.m2/repository`) manually. The `mavenLocal()` repository was added to
    build.gradle.kts to resolve it. In CI (with network), `mavenCentral()` will resolve the
    dependency normally. The version should be bumped to 2.11.0 once Maven Central connectivity is
    available.
- JNA's `Native.register()` (used by UniFFI-generated code) does NOT reliably use
    `java.library.path` alone. It needs `jna.library.path` JVM property AND/OR `LD_LIBRARY_PATH` env
    var. Both were added to the test task configuration. The original `java.library.path` was also
    retained for completeness.
- Test structure follows the Swift `ConformanceTests.swift` pattern exactly: one test method per gen
    function, companion object helpers for JSON parsing, hex decoding, and meta preparation.
