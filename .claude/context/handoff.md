## 2026-03-21 — Review of: Add maven-kotlin release workflow for Kotlin package publishing

**Verdict:** PASS

**Summary:** Clean, well-structured addition of the maven-kotlin release workflow to release.yml (4
jobs, ~180 lines) and Gradle publishing config in build.gradle.kts (~50 lines). Follows the
established Java Maven Central pattern faithfully. All 9+ verification checks pass. The Kotlin
bindings issue is now fully resolved — all sub-tasks complete (package, CI, conformance tests,
version sync, documentation, release workflow).

**Verification:**

- [x] `grep -q 'maven-kotlin' .github/workflows/release.yml` — input added
- [x] `grep -q 'build-kotlin-native' .github/workflows/release.yml` — build job added
- [x] `grep -q 'publish-maven-kotlin' .github/workflows/release.yml` — publish job added
- [x] `grep -q 'maven-publish' packages/kotlin/build.gradle.kts` — plugin added
- [x] `grep -q 'signing' packages/kotlin/build.gradle.kts` — signing plugin added
- [x] `grep -q 'io.iscc' packages/kotlin/build.gradle.kts` — groupId in publishing block
- [x] `grep -q 'iscc-lib-kotlin' packages/kotlin/build.gradle.kts` — artifactId present
- [x] `cd packages/kotlin && ./gradlew build` — BUILD SUCCESSFUL (7 tasks, signing skipped locally)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] YAML syntax validated via `yaml.safe_load()`
- [x] Staging publish produces correct Maven repo layout (POM + JAR + sources + javadoc + checksums)
- [x] No quality gate circumvention patterns in diff

**Issues found:**

- (none) — Kotlin bindings issue deleted from issues.md (all sub-tasks verified complete)

**Codex review:** Two P3 findings, both addressed:

1. **SCM `git://` URL in POM** — Kotlin used `scm:git:git://` but GitHub deprecated `git://`
    protocol. Java POM uses `https://`. Fixed in review commit (one-line change to match Java).
2. **Sources JAR includes native libs** — When CI copies native libs to `src/main/resources/` before
    build, `withSourcesJar()` bundles them into the sources JAR too. This is a packaging quality
    issue (sources JAR ~doubles in size), not a correctness issue. Could be fixed by excluding
    resources from the sources JAR task, but not blocking for initial release. Worth addressing if
    the combined native lib size becomes significant.

**Next:** The Kotlin bindings are fully complete. The remaining `normal` issues are the two Swift
packaging problems (SPM install instructions + native library vending). These are interconnected —
fixing the native library distribution (XCFramework or build-from-source) will inform the correct
install instructions. Consider tackling these together, or assess whether there are higher-priority
gaps in target.md first.

**Notes:**

- Release workflow now has 8 registry inputs (was 7). All follow the same pattern: boolean input →
    build → smoke test → publish with version-exists skip
- The `useInMemoryPgpKeys` approach for Kotlin (env vars) differs from Java's GPG system keyring
    import but is functionally equivalent and simpler for Gradle
- Central Portal upload uses curl REST API directly — no third-party Gradle plugin needed
- JNA resource paths use JNA conventions (linux-x86-64, darwin-aarch64, win32-x86-64) which differ
    from JNI conventions (linux-x86_64, macos-aarch64, windows-x86_64). This is intentional and
    correct
