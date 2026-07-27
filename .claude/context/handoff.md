# Handoff

## 2026-07-27 — Review of: Pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0`

**Verdict:** PASS

**Summary:** One `uses:` line in `release.yml`'s `publish-rubygems` job moved from `@main` to
`@v2.1.0`, with a two-line `# exact tag:` comment mirroring the `astral-sh/setup-uv` precedent in
`ci.yml`. Diff is 3 insertions / 1 deletion in a single non-test file — exactly what next.md scoped,
nothing excluded was touched (no SHA pin, no new policy gate, no other ref). All six next.md
criteria plus `mise run check` verified green this session; the committed comment's factual claim
was re-derived from the GitHub API and holds.

**Verification:**

- [x] `grep -c '…configure-rubygems-credentials@v2\.1\.0'` → `1`; `grep -c 'uses: .*@main'` → `0`
- [x] `uv run scripts/check_release_workflow.py` → exit 0, printed the `OK: …` pass line
- [x] `--check-action-inputs` (CI-exact invocation) → exit 0, **no** `warning: skipped` line,
    `action-inputs: resolved 18 of 18 action refs (0 skipped)` — the changed ref still resolves
- [x] `uv run pytest -q tests/test_check_release_workflow.py` → 27 passed
- [x] `uv run prek run --files .github/workflows/release.yml` → all applicable hooks Passed, no file
    modified (`git diff --stat` still 3 insertions / 1 deletion afterwards)
- [x] `actionlint@v1.7.7 .github/workflows/release.yml` → exit 0, no output
- [x] `mise run check` → all 17 hooks Passed
- [x] Extra (push pre-flight): `uv run prek run --hook-stage pre-push --all-files` → exit 0, 432
    pytest passed — the batch's never-CI'd `tools/cid.py` / `tests/test_cid.py` rewrite clears
    clippy, cargo test, `ty`, ruff `S`/`C901` and coverage locally
- [x] Gate integrity across the **whole unpushed range** (`@{upstream}..HEAD`, 29 files): the only
    added suppressions are the established `# noqa: S603/S607` on `git` subprocess argv in
    `tools/cid.py` — the pattern `decisions.md` 2026-07-25 explicitly preserves, not circumvention

**Issues found:**

- (no defects) One mis-spec, recorded in learnings rather than filed: next.md's Implementation Notes
    called `@main`'s head "a single transitive `yaml 2.8.3 → 2.9.0` dependabot bump". It is not —
    `gh api …/compare/v2.1.0...main` reports **31 commits ahead**, including `undici 6.24.1→6.27.0`
    (the OIDC token-exchange HTTP client), `@vercel/ncc 0.38.4→0.44.0` and a rebuilt `dist/index.js`
    bundle. That strengthens rather than weakens the change — the pin drops 31 unreviewed commits
    instead of one — and no false claim reached the repo: the committed comment says only that no
    floating major tag exists, which `git/matching-refs/tags` confirms (`v1.0.0`, `v2.0.0`,
    `v2.1.0`, nothing else). `v2.1.0` already carries the Node 24 switch, so nothing functional is
    lost.
- Ledger: the RULED `normal` issue "Pin `rubygems/configure-rubygems-credentials` off the `@main`
    branch" is resolved and deleted from issues.md. `decisions.md` 2026-07-26 already records the
    rationale — no new decision entry.

**Codex review:** No actionable regressions. "The workflow now references an existing exact action
tag, preserves the action's interface and behavior, and passes the relevant static checks and
tests."

**Next:** Make the CI job table in `.claude/context/specs/ci-cd.md` exhaustive — the `[human]` issue
authorizes the spec edit, it is the step next.md itself queued, and it is now the only `normal`
non-dependency item. After that, the authorized dependency majors one per step (xunit 3.x →
`Microsoft.NET.Test.Sdk` 18.x → Gradle wrapper 8.12.1 → JUnit 6.x → the riskier `jni` 0.22 /
`magnus` 0.8 migrations).

**Notes:**

- This push is the first to expose `acf178a`, `e38c17e`, `c7cc0bc` (785 lines of rewritten runner
    and its tests) to CI. Watch that run: pre-push passed locally, but `ci.yml` is the first
    independent exercise of the rewrite. `release.yml` itself stays unexercised — it is
    `workflow_dispatch`-only, so the static gates plus actionlint remain its full verification
    surface and the pin's real proof is the first v0.6.0 gem publish.
- Gate-strengthening candidate (needs Titusz's sign-off, do not scope unprompted): the repo now has
    **zero** `@main` action refs, so a "no floating branch ref" assertion in
    `scripts/check_release_workflow.py` would lock that in at near-zero cost. It is a new policy
    that must distinguish `@main` from deliberate pointers like `@stable` / `@release/v1`, which is
    why next.md excluded it.
