## 2026-03-22 — Review of: Validate assembled JAR in Kotlin release smoke test

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent correctly added JAR content validation to the `test-kotlin-release`
job in `release.yml`, adding `assemble-kotlin` as a dependency and checking all 9 JNA native library
resource paths. The Codex review caught a real bug (JAR selection picking `-javadoc.jar` instead of
the runtime JAR) which the reviewer fixed directly.

**Verification:**

- [x] YAML validation exits 0
- [x] `grep -A2 'test-kotlin-release' | grep 'assemble-kotlin'` — 2 matches (test needs + publish
    needs)
- [x] `grep 'kotlin-jar'` — 4 matches (upload name/path + download name/path)
- [x] `grep -c 'android-arm/'` — 1 match (validation only; build matrix uses `android-arm` without
    trailing `/`, so next.md's "at least 2" criterion was slightly inaccurate, but functional intent
    met)
- [x] `grep 'jar tf'` — 1 match (validation step)
- [x] `mise run format` — no changes
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (fixed) JAR selection bug: `ls *.jar | head -1` picks `-javadoc.jar` alphabetically instead of the
    runtime JAR. Fixed by adding `grep -v` to exclude `-sources.jar` and `-javadoc.jar` classifier
    JARs before `head -1`

**Codex review:** Codex correctly identified that `withSourcesJar()` and `withJavadocJar()` in
`build.gradle.kts` cause 3 JARs to be uploaded, and `head -1` selects the wrong one. This was a
legitimate P1 finding — fixed in the review commit.

**Next:** Three normal-priority release workflow issues remain: XCFramework cache key, Swift
ref:main race, root Package.swift CI. The XCFramework cache key issue is probably most impactful
(stale cache could publish wrong binaries). Alternatively, the benchmarks documentation gap
(publishing speedup factors) could be addressed — it's more visible to users.

**Notes:** The Kotlin JAR smoke test issue is resolved and deleted from issues.md. After this fix, 3
normal-priority issues + 1 low-priority issue remain. All are release workflow hardening or cosmetic
— no blocking issues.
