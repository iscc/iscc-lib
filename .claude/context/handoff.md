## 2026-03-21 — Review of: Fix Kotlin CI — wrong Gson Maven groupId

**Verdict:** PASS

**Summary:** The advance agent correctly fixed the Kotlin CI failure by changing the Gson Maven
dependency coordinate in `packages/kotlin/build.gradle.kts` from `com.google.gson:gson:2.8.9` (wrong
groupId) to `com.google.code.gson:gson:2.11.0` (correct groupId, matching Java binding's pom.xml).
Clean 1-line fix, perfectly scoped.

**Verification:**

- [x] `cd packages/kotlin && ./gradlew compileTestKotlin` succeeds — BUILD SUCCESSFUL, dependency
    resolves from Maven Central
- [x] `cd packages/kotlin && ./gradlew test` passes all 9 conformance tests — BUILD SUCCESSFUL
- [x] `grep 'com.google.code.gson:gson:2.11.0' packages/kotlin/build.gradle.kts` — finds the
    corrected dependency (line 16)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** No regressions or blocking issues found. Codex confirmed the Gson coordinate fix
is correct and the Kotlin test suite still passes.

**Next:** Continue Kotlin integration. Remaining sub-tasks from the Kotlin issue:

1. **Documentation** — `docs/howto/kotlin.md` howto guide, `packages/kotlin/README.md`,
    `packages/kotlin/CLAUDE.md`
2. **README integration** — Kotlin install/quickstart sections in root README
3. **Release workflow** — `maven-kotlin` input in `release.yml`

Documentation is the largest chunk — could be done in one step (howto + README + CLAUDE.md) or
split.

**Notes:**

- Kotlin CI should now pass (16/16 green) once this push lands — verify in next update-state
- The two Swift packaging issues remain open (normal priority)
- Gson groupId confusion: Java *package* name is `com.google.gson` (used in imports) while Maven
    *artifact* groupId is `com.google.code.gson` — a well-known gotcha
