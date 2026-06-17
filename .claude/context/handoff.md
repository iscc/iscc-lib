> **HUMAN REVIEW REQUESTED**: TWO CID loops are running concurrently on the same `develop` checkout
> and racing each other. `ps aux` shows two `mise run cid:run 100 1800` process trees — one started
> 07:17 (pts/5, currently running THIS iteration-97 review) and one started 09:10 (pts/6, already on
> iteration-98 `define-next`, having run iter-98 update-state that rewrote `state.md` in the working
> tree at 09:15). They clobber each other's context files and will race on `git push`. **A human
> must kill the duplicate loop** (keep one, e.g. terminate the 09:10 process group `78441`/pgid
> `78441`) before the repo state is corrupted further. Because of this race I did NOT push (a
> concurrent push would collide); the iter-97 advance work is sound and ready to push once a single
> loop remains.

## 2026-06-17 — Review of: CRAP gate Phase 3 — regression gate with committed baseline

**Verdict:** PASS_WITH_NOTES (work is correct; push withheld due to the concurrent-loop hazard
above)

**Summary:** The advance agent turned the report-only Phase 2 CRAP gate into an enforcing regression
gate exactly as next.md scoped: a committed `.crap-baseline.json` (97 iscc-lib functions, 10 source
files), an enforcing `cargo crap --fail-regression --baseline` final step in the `Coverage + CRAP`
CI job, a `mise run crap:baseline` refresh task, and the ci-cd.md Phase 3 checkbox flipped. The diff
is tight and touches only the files next.md named. All verification criteria pass locally; the work
did NOT cause the `mise run check` failure (that was a race artifact from the second loop writing
`state.md` mid-run — no `.rb` file is actually dirty).

**Verification:**

- [x] Gate passes against committed baseline —
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` exits 0
    (`0 regressed`, `97 unchanged`) on freshly regenerated coverage
- [x] Regression is caught — inflated baseline (`/tmp/b.json`) → exit 1 (`17 regressed`)
- [x] Baseline tracked, not gitignored — `git ls-files` confirms tracked; `git check-ignore` empty;
    only `lcov.info`/`crap.sarif` ignored
- [x] Baseline shape — top-level keys `$schema`, `version` (`0.2.2`), `entries`; 97 entries; every
    `file` under `crates/iscc-lib/src/` (10 files)
- [x] CI gate step present — `grep -E 'fail-regression|crap-baseline' ci.yml` shows the
    `CRAP regression gate` step at line 336, not continue-on-error, runs last (after SARIF upload)
- [x] mise task — `mise tasks` lists `crap:baseline`; `mise run crap:baseline` regenerates
    `.crap-baseline.json` byte-identical (idempotent, no spurious diff)
- [x] ci-cd.md — line 414 checkbox `[x]`; Phase 3 prose names `mise run crap:baseline` as the
    reviewed-commit refresh mechanism
- [~] `mise run check` — 14/15 hooks green (mdformat, fmt, yaml, json, toml, large-files all pass);
    `standardrb-fix` reported "files modified" but this is a RACE ARTIFACT (the concurrent loop
    wrote `state.md` during the run; no `.rb` is dirty). The advance-touched files are individually
    format-clean (ci-cd.md mdformat-clean, mise.toml valid TOML, ci.yml valid YAML)

**Issues found:**

- **Codex P2 (valid, follow-up):** `--fail-regression` only blocks worsening of *existing* baseline
    entries; a brand-new high-CRAP function reports `★ new` and exits 0, bypassing the gate. This
    was a deliberate Phase 3 scope choice (regression-only, no `--fail-above`), not a defect — filed
    as a new `[review]` normal issue (pair `--fail-above 30` with `--fail-regression`; current max
    ~22.3).
- **Environment (critical):** concurrent duplicate CID loops — see HUMAN REVIEW REQUESTED banner.

**Codex review:** One [P2] finding (above) — the enforcing gate does not fail on new risky
functions, only on regressions of existing entries. Confirmed accurate against `cargo-crap 0.2.2`
semantics. Advisory follow-up, not blocking for Phase 3, which deliberately excluded `--fail-above`.

**Next:** (1) **Human: resolve the duplicate loop, then push the iter-97 batch** (4 commits ahead of
`origin/develop` `cbc0d14`) and confirm the new enforcing CRAP gate runs green in CI — there is a
known cross-environment determinism risk (baseline captured in devcontainer, CI regenerates on
`@stable`); if the first run flaps, regenerate `.crap-baseline.json` from CI's `lcov` artifact, do
NOT widen `--epsilon`. The "Add Rust coverage + CRAP quality gate" issue stays OPEN until then. (2)
Consider the new `[review]` issue: add `--fail-above 30` so new high-CRAP code also fails. (3)
Remaining v1.0.0 gates: `iai-callgrind` perf-regression gate (mirror this reviewed-baseline pattern)
and the PyO3 0.23→0.29 bump.

**Notes:**

- The iter-97 advance code (ci.yml, mise.toml, .crap-baseline.json, ci-cd.md) is genuinely sound and
    ready to land. The ONLY blocker is the concurrent-loop hazard, which is an environment fault,
    not a code problem.
- I staged ONLY my review artifacts (learnings, handoff, issues, agent memory). I did NOT stage the
    working-tree `state.md` change (it belongs to the other loop's iter-98 update-state) or
    `iterations.jsonl` (runner-owned).
- Do not interpret a future `git push` failure as a code-quality problem until the duplicate loop is
    gone — pushes will fail by non-fast-forward as long as two loops commit to the same branch.
