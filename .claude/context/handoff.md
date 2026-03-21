## 2026-03-21 — Review of: Add Kotlin conformance tests for all 9 gen\_\*\_v0 functions

**Verdict:** PASS

**Summary:** Kotlin conformance tests were implemented correctly, covering all 9 `gen_*_v0`
functions with 50 test vectors from `data.json`. The code is clean, idiomatic Kotlin, and follows
the established Swift conformance test pattern. All quality gates pass.

**Verification:**

- [x] `cargo build -p iscc-uniffi` exits 0 — built in 1.34s
- [x] `cd packages/kotlin && ./gradlew test` exits 0 with all 9 test methods passing (50 vectors) —
    XML report confirms `tests="9"` skipped="0" failures="0" errors="0"
- [x] `./gradlew test` output shows BUILD SUCCESSFUL — test report confirms 9 tests successful
- [x] `test -f packages/kotlin/src/test/resources/data.json` exits 0 — file exists, SHA256 matches
    `crates/iscc-lib/tests/data.json` exactly
- [x] `test -f packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt` exits 0

**Issues found:**

- (none — clean implementation)

**Minor observations (not blocking):**

- `mavenLocal()` repository added to build.gradle.kts as devcontainer workaround (Maven Central
    unreachable offline). Harmless — CI resolves from `mavenCentral()` which is also present
- Gson 2.8.9 instead of 2.11.0 specified in next.md — due to devcontainer cache. Functionally
    equivalent for this use case. Can be bumped when CI job is added
- JNA native library loading fix (`jna.library.path` + `LD_LIBRARY_PATH`) was a good proactive
    addition — `java.library.path` alone was insufficient for JNA's `Native.register()`

**Codex review:** Skipped — Codex sandbox (bwrap) cannot create namespaces in this devcontainer
environment, preventing git access. All commands fail with "No permissions to create a new
namespace."

**Next:** Add Kotlin CI job to `.github/workflows/ci.yml` — the natural next step now that
conformance tests pass locally. Then: version sync (add `gradle.properties` to `version_sync.py`),
documentation (`docs/howto/kotlin.md`, README Kotlin sections), and release workflow.

**Notes:**

- Kotlin issue in issues.md remains open — conformance tests done, but CI job, version sync, docs,
    README, and release workflow still needed
- data.json is now the 5th vendored copy (Rust, Go, .NET, Swift, Kotlin) — established pattern
- `HexFormat.of().parseHex()` used for hex decoding requires Java 17+ — consistent with project's
    JDK 17 requirement
