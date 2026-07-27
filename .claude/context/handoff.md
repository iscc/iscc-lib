# Handoff

## 2026-07-27 — Pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0`

**Done:** Replaced the repository's only `@main` action reference — the OIDC credential step in
`release.yml`'s `publish-rubygems` job (formerly line 895) — with the exact tag `@v2.1.0`, preceded
by a two-line `# exact tag:` comment mirroring the `astral-sh/setup-uv` precedent in `ci.yml` lines
61-63. The step passes no `with:` keys, so this is purely a trust-anchor change.

**Files changed:**

- `.github/workflows/release.yml`: one `uses:` line changed `@main` → `@v2.1.0`, plus a two-line
    comment above it explaining the action publishes no floating major tag (only
    `v1.0.0`/`v2.0.0`/`v2.1.0` exist, so `@v2` does not resolve). Diff: 3 insertions, 1 deletion.

**Verification:** (all six next.md criteria run this session)

- [x] `grep -c 'uses: rubygems/configure-rubygems-credentials@v2\.1\.0'` → `1`;
    `grep -c 'uses: .*@main'` → `0`
- [x] `uv run scripts/check_release_workflow.py` → exit 0, "OK: … passed release-workflow static
    checks."
- [x] `uv run --no-project --with pyyaml python scripts/check_release_workflow.py   --check-action-inputs`
    → exit 0, **no** `warning: skipped` line, summary
    `action-inputs: resolved 18 of 18 action refs (0 skipped)` — the changed ref still resolves
- [x] `uv run pytest -q tests/test_check_release_workflow.py` → 27 passed
- [x] `uv run prek run --files .github/workflows/release.yml` → every applicable hook Passed
    (yamlfix included); `git diff --stat` afterward still shows exactly 3 insertions / 1 deletion,
    so no hook modified the file or the comment
- [x] `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 .github/workflows/release.yml` →
    exit 0, no output
- [x] `mise run check` → exit 0, all 17 hooks Passed (run twice: once via `mise run format`
    beforehand per implementation notes, once as the final gate)

**Next:** As next.md itself queued: make the `specs/ci-cd.md` job table exhaustive against the 21
real `ci.yml` jobs (authorized spec edit), then the authorized dependency major bumps one per step
(xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle wrapper 8.12.1, JUnit 6.x, then the riskier `jni`
0.22 / `magnus` 0.8 migrations).

**Notes:**

- This push is the first to expose commits `acf178a`, `e38c17e`, `c7cc0bc` (rewritten `tools/cid.py`
    / `tests/test_cid.py`) to CI, per next.md's rationale for keeping this diff tiny. Nothing here
    touches those files.
- Ledger note for review: this closes the RULED `normal` issue "Pin
    `rubygems/configure-rubygems-credentials` off the `@main` branch" — the fix is exactly option
    (a) from `decisions.md` 2026-07-26 (exact tag, no SHA pin). issues.md update is review's.
- No SHA pin, no new policy gate in `scripts/check_release_workflow.py`, no other ref touched — all
    per Not In Scope. `release.yml` is `workflow_dispatch`-only, so the static gates plus actionlint
    are the full verification surface; no CI run exercises the changed step.
- No agent-memory change: nothing non-derivable was learned (the exact-tag-only publishing fact was
    already recorded in learnings and deps-refresh.md).
