## 2026-03-21 — Fix Kotlin CI — wrong Gson Maven groupId

**Done:** Fixed the Gson Maven dependency coordinate in `packages/kotlin/build.gradle.kts` from
`com.google.gson:gson:2.8.9` (wrong groupId) to `com.google.code.gson:gson:2.11.0` (correct groupId,
matching the Java binding's pom.xml).

**Files changed:**

- `packages/kotlin/build.gradle.kts`: line 16 — changed
    `testImplementation("com.google.gson:gson:2.8.9")` to
    `testImplementation("com.google.code.gson:gson:2.11.0")`

**Verification:**

- `./gradlew compileTestKotlin` — BUILD SUCCESSFUL (dependency resolves from Maven Central)
- `./gradlew test` — BUILD SUCCESSFUL, all 9 conformance tests pass
- `grep 'com.google.code.gson:gson:2.11.0' packages/kotlin/build.gradle.kts` — finds the corrected
    line
- `mise run check` — all 15 pre-commit hooks pass

**Next:** Kotlin documentation — `docs/howto/kotlin.md` howto guide, `packages/kotlin/README.md`,
`packages/kotlin/CLAUDE.md`. Then README integration (Kotlin install/quickstart in root README) and
release workflow (`maven-kotlin` input in `release.yml`).

**Notes:** Pure 1-line fix, no surprises. The Java import statements (`com.google.gson.*`) are Java
package names and were already correct — only the Maven artifact coordinate in build.gradle.kts was
wrong.
