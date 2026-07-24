# Handoff

## 2026-07-24 — Dependency refresh slice 4 — GitHub Actions versions in ci.yml + docs.yml

**Done:** Bumped all 9 stale GitHub Actions refs in `.github/workflows/ci.yml` (29 `uses:` lines
changed) and all 5 in `.github/workflows/docs.yml` to their current majors, exactly per the next.md
table. No action was held back — every evaluated breaking change was confirmed non-applicable to our
usage, so no `# held:` comments were needed. No steps, jobs, inputs, or runtime versions were added,
removed, or changed.

**Files changed:**

- `.github/workflows/ci.yml`: `actions/checkout@v4→v7` (18), `actions/setup-python@v5→v7` (2),
    `astral-sh/setup-uv@v4→v9` (1), `actions/setup-node@v4→v7` (1), `actions/setup-java@v4→v5` (2),
    `actions/setup-go@v5→v7` (1), `actions/setup-dotnet@v4→v6` (1), `actions/upload-artifact@v4→v7`
    (2), `github/codeql-action/upload-sarif@v3→v4` (1)
- `.github/workflows/docs.yml`: `actions/checkout@v4→v7`, `actions/setup-python@v5→v7`,
    `astral-sh/setup-uv@v4→v9`, `actions/upload-pages-artifact@v3→v5`, `actions/deploy-pages@v4→v5`
    (paired bump per next.md)

**Verification:**

- [x] YAML parses: `uv run python -c "import yaml; ..."` → exit 0 for both files (system `python3`
    lacks PyYAML; the venv one works)
- [x] Stale-ref grep from next.md → no output, exit 1 (no old refs remain in the two files)
- [x] `grep -c 'actions/checkout@v7'` → ci.yml **18**, docs.yml **1**
- [x] `grep -c 'uses:'` → ci.yml **67**, docs.yml **5** (step counts unchanged)
- [x] `release.yml` untouched and internally consistent: checkout@v4 **23**, upload-artifact@v4
    **11**, download-artifact@v4 **20**
- [x] `.pre-commit-config.yaml` untouched: `rev: v6.0.0` **1**, `rev: 1.0.0` **1**
- [x] `mise run format` exit 0; `mise run check` exit 0 — all 15 pre-commit hooks pass, no file
    rewritten (`git status --porcelain` shows only the two workflow files + the runner's
    `iterations.jsonl`)
- [ ] CI on pushed develop commit — see follow-up section below (result appended after polling the
    pushed sha; could not be verified before this commit exists)

**Next:** Dependency refresh slice 5 = `release.yml` (97 `uses:` refs; `upload-artifact@v4` ↔
`download-artifact@v4` must move together and can only be truly validated by a release run —
consider whether that slice should wait for the next human-triggered release). After that, the
remaining per-ecosystem manifests (jni `pom.xml` + kotlin `build.gradle.kts`; dotnet `.csproj`; go
`go.mod`; rb `Gemfile`/gemspec) and the three flagged migrations (ruff 0.16, magnus 0.8, jni 0.22).

**Notes:**

- Pure mechanical bump — no `# held:` line was needed since none of the surveyed breaking changes
    (node24 runtime, setup-uv default-cache behavior, setup-node packageManager-gated caching,
    upload-artifact backend, upload-pages-artifact hidden-file exclusion) applies to how these two
    workflows invoke the actions.
- Per next.md, `package-manager-cache: false` was NOT added to the `setup-node` step; if the
    `nodejs` job fails with "Dependencies lock file is not found", that is the documented remedy.
- `docs.yml` only triggers on push to `main`, so its bumps are statically verified only (YAML parse
    \+ grep); first real exercise will be the next develop→main merge.
- The `upload-artifact@v7` bump in ci.yml is safe from pairing issues: ci.yml contains zero
    `download-artifact` steps (both uploads are terminal artifacts: `iai-baseline`, `lcov`).
