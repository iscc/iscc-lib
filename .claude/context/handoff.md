# Handoff

## 2026-07-24 — Dependency refresh slice 4 — GitHub Actions versions in ci.yml + docs.yml

**Done:** Bumped all 9 stale GitHub Actions refs in `.github/workflows/ci.yml` (29 `uses:` lines
changed) and all 5 in `.github/workflows/docs.yml` to their current majors per the next.md table,
with one deviation forced by upstream: `astral-sh/setup-uv` is pinned to the exact tag `@v9.0.0`
because the floating major tag `@v9` **does not exist** — upstream stopped publishing floating major
tags after `v7` (verified: `git/matching-refs/tags/v` lists only v1–v7; v8.x/v9.0.0 are exact
release tags; setup-uv's own README pins exact refs). The first pushed attempt (`ebac57f`, with
`@v9`) failed CI on exactly this: `Unable to resolve action astral-sh/setup-uv@v9`. A follow-up
commit switched both files to `@v9.0.0` with an explanatory YAML comment. No steps, jobs, inputs, or
runtime versions were added, removed, or changed.

**Files changed:**

- `.github/workflows/ci.yml`: `actions/checkout@v4→v7` (18), `actions/setup-python@v5→v7` (2),
    `astral-sh/setup-uv@v4→v9.0.0` (1, + 2-line comment why exact tag), `actions/setup-node@v4→v7`
    (1), `actions/setup-java@v4→v5` (2), `actions/setup-go@v5→v7` (1), `actions/setup-dotnet@v4→v6`
    (1), `actions/upload-artifact@v4→v7` (2), `github/codeql-action/upload-sarif@v3→v4` (1)
- `.github/workflows/docs.yml`: `actions/checkout@v4→v7`, `actions/setup-python@v5→v7`,
    `astral-sh/setup-uv@v4→v9.0.0` (+ same comment), `actions/upload-pages-artifact@v3→v5`,
    `actions/deploy-pages@v4→v5` (paired bump per next.md)

**Verification:**

- [x] YAML parses: `uv run python -c "import yaml; ..."` → exit 0 for both files (system `python3`
    lacks PyYAML; the venv one works)
- [x] Stale-ref grep from next.md → no output, exit 1 (`@v9.0.0` is not matched by
    `setup-uv@v[1-8]`, so the assertion holds as written)
- [x] `grep -c 'actions/checkout@v7'` → ci.yml **18**, docs.yml **1**
- [x] `grep -c 'uses:'` → ci.yml **67**, docs.yml **5** (step counts unchanged; the setup-uv comment
    adds no `uses:` line)
- [x] `release.yml` untouched and internally consistent: checkout@v4 **23**, upload-artifact@v4
    **11**, download-artifact@v4 **20**
- [x] `.pre-commit-config.yaml` untouched: `rev: v6.0.0` **1**, `rev: 1.0.0` **1**
- [x] `mise run format` + `mise run check` exit 0 — all 15 pre-commit hooks pass, no file left
    rewritten
- [x] **CI evidence from first push (`ebac57f`):** 41 check-runs across two runs (push +
    `pull_request` from the open develop→main PR). The ONLY failures were the 6 Python entries, all
    rooted in `Unable to resolve action astral-sh/setup-uv@v9` at job setup (matrix sibling +
    aggregate gate cascades). Every other bumped action — checkout@v7, setup-python@v7,
    setup-node@v7, setup-java@v5, setup-go@v7, setup-dotnet@v6, upload-artifact@v7, upload-sarif@v4
    — resolved and its job passed, including the enforcing perf/coverage/audit gates.
- [ ] CI on the follow-up fix commit — pushed; result pending at handoff-commit time. Reviewer:
    `gh api repos/iscc/iscc-lib/commits/<sha>/check-runs --jq '[.check_runs[]|select(.conclusion!="success")]|length'`
    (note: counts check-runs from BOTH the push run and the PR run on this sha).

**Next:** Dependency refresh slice 5 = `release.yml` (97 `uses:` refs; `upload-artifact@v4` ↔
`download-artifact@v4` must move together and can only be truly validated by a release run —
consider whether that slice should wait for the next human-triggered release). Note for that slice:
`astral-sh/setup-uv` appears in release.yml too and must use `@v9.0.0` (exact tag), not `@v9`. After
that: per-ecosystem manifests (jni `pom.xml` + kotlin `build.gradle.kts`; dotnet `.csproj`; go
`go.mod`; rb `Gemfile`/gemspec) and the three flagged migrations (ruff 0.16, magnus 0.8, jni 0.22).

**Notes:**

- **Survey gap worth remembering:** `gh api repos/<o>/<r>/releases/latest` proves a release exists
    but NOT that a floating major tag exists. Always confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN`. This is how next.md's
    `@v9` recommendation slipped through.
- The `@v9.0.0` exact pin means setup-uv patch releases are not picked up automatically — a
    deliberate trade-off since upstream offers no floating tag; the YAML comment above each pin
    documents this.
- Per next.md, `package-manager-cache: false` was NOT added to the `setup-node` step — the nodejs
    job passed on the first push with setup-node@v7, confirming the survey's caching analysis.
- `docs.yml` only triggers on push to `main`, so its bumps are statically verified only (YAML parse
    \+ grep); first real exercise is the next develop→main merge.
- ci.yml has zero `download-artifact` steps, so `upload-artifact@v7` has no pairing risk (both
    uploads are terminal artifacts: `iai-baseline`, `lcov`).
- Each commit on develop currently triggers TWO CI runs (push + `pull_request` for the open
    develop→main PR) — the check-runs API returns both, which is why totals show ~41 not ~20.
