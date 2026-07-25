# Handoff

## 2026-07-25 — Review of: Refresh the 97 GitHub Actions `uses:` refs in release.yml

**Verdict:** PASS

**Summary:** Nine stale action refs in `.github/workflows/release.yml` were bumped to their current
floating majors across 73 of the 97 `uses:` lines. The diff is exactly what was asked for: all 146
changed lines (73 +/73 −) are `uses:` lines — no `if:`, `needs:`, `with:`, `env:`, `permissions:` or
`run:` block moved, verified by filtering the diff for non-`uses:` changes (empty). Every next.md
criterion reproduced independently, and I went past them: each new major's `action.yml` was fetched
at its tag ref to prove the workflow's `with:` keys and step outputs still exist, and every
intervening major's release notes were read for *default* changes.

**Verification:**

- [x] `uses:` histogram matches the expected 18-ref multiset — empty diff, exit 0
- [x] Total `uses:` count still exactly `97` — no step added, removed or duplicated
- [x] `grep -c 'setup-uv'` → `0` — none added (per Not In Scope)
- [x] Iteration-139 guard invariant intact — `release.yml guards OK` (29 jobs, `prepare-release`
    bare, 28 exact-shape, all per-flag token counts identical)
- [x] Artifact wiring resolves — `unmatched: []` (11 upload names, 20 downloads)
- [x] `actionlint@v1.7.7 .github/workflows/release.yml` → exit 0, no output
- [x] `prek run check-yaml --files …` → `Passed`
- [x] `prek run yamlfix --files …` → `Passed`, no `files were modified by this hook`
- [x] `mise run check` → 15/15 hooks Passed, exit 0
- [x] No source/gate side effects — `git status --porcelain` lists only runner-owned
    `iterations.jsonl`; no Rust source touched, so no CRAP/iai baseline, `cargo deny` or semver
    refresh owed
- [x] Gate-circumvention sweep over `origin/develop..HEAD` (4 commits) — clean. The three
    `noqa`/`exclude` grep hits are prose inside agent-memory files, not code or config
- [x] **Extra (review-added):** all nine `@vN` tags resolve (`git/ref/tags/<vN>` → 200)
- [x] **Extra:** every `with:` key the workflow passes is still a declared `inputs` key in the new
    major's `action.yml`; `cache@v6` still declares the `cache-hit` output the workflow reads; all
    nine refs are `runs.using: node24`
- [x] **Extra:** intervening-major release notes read for behavioural defaults — no live risk (see
    Notes)

**Issues found:**

- (none blocking) Two follow-ups filed to `issues.md`, both explicitly deferred by this step's
    `Not In Scope`:
    - `[review]` `normal` — **Land the `release.yml` static checks as an executable gate.** The
        guard-shape, artifact-wiring and (new) action-input checks have now been hand-retyped from
        `next.md` into heredocs in two consecutive iterations. A `workflow_dispatch`-only file with no
        gate is one forgotten heredoc away from silently reintroducing the iter-139 publish bug.
    - `[review]` `normal` — **Pin `rubygems/configure-rubygems-credentials` off `@main`.** Needs
        Titusz's call: upstream's README shows `@main` everywhere but also recommends SHA pinning, and
        the repo convention is "tags, never SHAs". Facts gathered in the issue (no floating `v2`
        exists; `main` is ahead of `v2.1.0`; the step passes no `with:` keys).
- The issue-ledger text claiming `release.yml` "needs the exact `setup-uv@v9.0.0`" was **wrong** —
    the file invokes no `uv`/`uvx` at all. next.md caught this while scoping and advance correctly
    left the file without a `setup-uv` step; corrected in `issues.md` this review.

**Codex review:** Available and **clean — no findings.** Verdict text: "The updated action tags
exist, retain all inputs used by the workflow, and are compatible with the GitHub-hosted runners
configured here. The workflow structure, artifact wiring, and release logic remain unchanged." This
converges with my own independent `action.yml` probing.

**Next:** The tooling/CI thread is genuinely exhausted — slice 9 closes the last locally-verifiable
ecosystem slice of the v0.6.0 dependency refresh. Two defensible candidates, in preference order:

1. **The Rust-core-only Unicode boundary fixture** (format + loader + three single-code-point cases:
    a 15.1→16 emoji retained, a Unicode-16 code point with a canonical decomposition, a post-16.0
    code point the freeze rule must strip). No binding wiring, no `specs/rust-core.md` edit — the
    parked HUMAN REVIEW sequence-ordering question only affects *sequence* vectors and step (b)'s
    11-binding propagation, so single-code-point fixtures are safe to land now.
2. **The `release.yml` static-check script** (issue above) — smaller, pure-tooling, and it converts
    two iterations of hand-retyped heredocs into a re-running gate. Good filler if (1) looks too
    large; scope it as one `scripts/` file + one narrowly-`files:`-scoped prek hook + one CI step.

Do **not** pick the `rubygems/configure-rubygems-credentials` pin until Titusz answers the
tag-vs-SHA question.

**Notes:**

- **Statically verified only, and that is now a recorded, accepted risk** (`decisions.md`
    2026-07-25). `release.yml` is `workflow_dispatch`-only; first real-world confirmation is the
    next release run. Six of the nine majors already run green in `ci.yml` with byte-identical
    `with:` blocks.
- **Behavioural deltas I checked and cleared** (an input surviving is not its default surviving):
    - `setup-node@v5+` auto-enables package-manager caching when `package.json` carries a
        `packageManager` field, and then *fails* the run if no lockfile exists. Safe here: the only
        tracked `package.json` (`crates/iscc-napi/`) has no `packageManager` field and the repo tracks
        no lockfile. **Re-check this the moment either is added.**
    - `checkout@v6+` persists the auth token to `$RUNNER_TEMP` instead of `.git/config`. Upstream
        states plain `git push`/`fetch` still work — which matters because `prepare-release` commits
        to `main` and force-pushes two tags. `persist-credentials` still defaults to `true` at v7.
        Only authenticated git *inside a Docker container action* needs runner ≥ 2.329.0; none here.
    - `setup-python@v7` dropped `pip-install` (unused), `setup-node@v7` dropped the dummy
        `NODE_AUTH_TOKEN` export (both publish jobs set it themselves in `env:`),
        `upload-artifact@v7`'s new `archive:` input defaults to `true` (unchanged behaviour),
        `download-artifact@v8`'s `skip-decompress` defaults to `false`.
- `download-artifact@v8` defaults `digest-mismatch: error` (was warn). Deliberately left strict — a
    corrupted artifact should fail a publish pipeline. If a release fails on this, investigate the
    artifact, do **not** relax the input.
- `learnings.md` pruned back toward budget (205 lines): the closed prek `.pyi`/formatter detail
    moved to `learnings-archive.md`, and the guard-shape, freeze-rule-adjacency and
    `proc-macro-error2` entries compressed to point at `decisions.md` / `issues.md` where their full
    text already lives.
- Advance trimmed its own `MEMORY.md` from 142 to 139 lines, clearing last iteration's Codex P3. The
    review agent's own index was over its limit too (168 lines) and is now 138: the release.yml
    static gates, the action-major verification recipe, the cleared-defaults table and the pinning
    conventions moved into a new `.claude/agent-memory/review/gha-workflow-reviews.md`.
