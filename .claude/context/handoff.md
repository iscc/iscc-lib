## 2026-03-21 — Review of: Add Kotlin CI job to ci.yml

**Verdict:** PASS

**Summary:** The Kotlin CI job was added correctly to `ci.yml`, following the established Java CI
job pattern exactly. The diff is minimal and well-scoped — only `ci.yml` was modified. All
verification criteria pass, all pre-commit hooks pass, and clippy is clean across the workspace.

**Verification:**

- [x] `grep -c 'kotlin:' .github/workflows/ci.yml` returns 1
- [x] `grep 'cargo build -p iscc-uniffi' .github/workflows/ci.yml` finds build step (2 matches:
    swift + kotlin)
- [x] `grep 'gradlew test' .github/workflows/ci.yml` finds test step (1 match)
- [x] `grep -c 'java-version.*17' .github/workflows/ci.yml` returns 2 (java + kotlin)
- [x] YAML valid: `python -c "import yaml; yaml.safe_load(open(...))"` exits 0
- [x] `mise run check` — all 15 hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** Skipped — Codex sandbox (bwrap) cannot create namespaces in this devcontainer.

**Next:** Continue Kotlin integration. The remaining sub-tasks from the Kotlin issue are:

1. **Version sync** — add `packages/kotlin/gradle.properties` to `scripts/version_sync.py`
2. **Documentation** — `docs/howto/kotlin.md` howto guide
3. **README integration** — Kotlin install/quickstart sections in root README
4. **Per-package docs** — `packages/kotlin/README.md` + `packages/kotlin/CLAUDE.md`
5. **Release workflow** — `maven-kotlin` input in `release.yml`

Version sync is the smallest and most mechanical — good candidate for next step.

**Notes:**

- CI now has 15 YAML job entries (16 actual jobs when counting python-test matrix expansion)
- Kotlin issue in issues.md remains open — CI sub-task done, but version sync, docs, README, and
    release workflow still pending
- No quality gate circumvention detected in diff
