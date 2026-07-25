# Handoff

## 2026-07-25 — Review of: Close two silent no-op gates — release.yml re-trigger guards and `.pyi` hook coverage

**Verdict:** PASS

**Summary:** Two long-standing silent no-op gates are closed. All 19 unguarded
`test-*`/`assemble-*`/`pack-*`/`publish-*` jobs in `release.yml` now carry
`${{ !cancelled() && !failure() && (…) }}`, matching `publish-crates-io`, so `-f <registry>=true`
re-triggers reach their publish jobs instead of inheriting `prepare-release`'s skip. Both local ruff
hooks gained the `pyi` type tag, making the local prek surface a genuine strict superset of CI's.
The diff is exactly the scoped 2 non-doc + 3 doc files; every registry condition is preserved
verbatim and no `needs:` list was touched.

**Verification:**

- [x] Guard-shape script (29 jobs, `prepare-release` untouched, token counts) — reproduced
    independently: `release.yml guards OK`. All 28 non-`prepare-release` jobs match the exact shape;
    `prepare-release` is still bare `inputs.version != ''`
- [x] `actionlint@v1.7.7 .github/workflows/release.yml` — exit 0, no output
- [x] `prek run check-yaml --files release.yml` — `Passed`
- [x] `prek run yamlfix --files release.yml` — `Passed`, no `files were modified`
- [x] `grep -A6 'id: ruff-check' … 'types_or: [python, pyi]'` — exit 0
- [x] `grep -A8 'id: ruff-format' … 'types_or: [python, pyi, markdown]'` — exit 0
- [x] `prek run ruff-check`/`ruff-format --files …/_lowlevel.pyi` — both `Passed`, neither printed
    `no files to check`. **Probed harder than asked:** a *staged, deliberately dirty*
    `probe_gate.pyi` (`import os` + `def f(x:int)->int: ...`) now makes both hooks report
    `files were modified by this hook` (`Found 1 error (1 fixed)`, `1 file reformatted`) where iter
    138 got `(no files to check) Skipped`. The gate genuinely bites; probe removed afterwards
- [x] No gate weakened — `grep -c 'id:'` → 22 (no hook added/removed/narrowed);
    `grep -cE '^\s*exclude:'` → 0. **Criterion was mis-specified:** `grep -c 'exclude'` → 2, not 0,
    both pre-existing `--force-exclude` CLI flags on the pre-push S/C901 hooks (confirmed present at
    `HEAD~1`). Advance's correction accepted
- [x] `uv run ruff check` → `All checks passed!`; `uv run ruff format --check` → exit 0. **Criterion
    was mis-specified:** 155 files, not 153 — the two extra are the tracked `.md` agent-memory files
    added by this iteration's own update-state and define-next commits (verified
    `git diff HEAD~3..HEAD --diff-filter=A`). Exit code is the gate; the count was a stale
    prediction
- [x] `mise run check` — 15/15 hooks `Passed`, exit 0; `git status --porcelain` afterwards shows
    only runner-owned `iterations.jsonl`
- [x] `grep -q 'Known bug' SKILL.md` → exit 1; `grep -q 'gh run rerun' SKILL.md` → exit 0; the
    "Never pass `-f version=`" warning is intact (line 469)
- [x] `grep -q 'pyi' docs/development.md && grep -q 'pyi' CLAUDE.md` → exit 0
- [x] `uv run zensical build` → exit 0, `No issues found`
- [x] Gate integrity over the full unpushed range (`origin/develop..HEAD`) — no suppression, skip,
    threshold reduction, hook weakening or scope exclusion. Both config changes strictly *widen* a
    gate
- [x] Scope discipline — 2 non-doc files (`release.yml`, `.pre-commit-config.yaml`), exactly the
    stated budget; nothing from `## Not In Scope` touched (no `uses:` ref bumped, `prepare-release`
    untouched, no `always()`, pre-push S/C901 hooks unchanged, `issues.md` left to review)
- [x] No Rust/binding source touched — no CRAP baseline, iai perf, `cargo deny` or semver refresh
    owed; no API surface change

**Issues found:**

- (none blocking) Two next.md criteria were mis-specified (exact file count, substring `grep -c`).
    Advance handled both correctly: reported the mismatch, proved the intent, did not chase the
    number. Crystallized as a learnings entry rather than an issue
- Independently verified the safety argument for relaxing the implicit `success()`: **every** job's
    `needs` chain is gated by the same registry flag or a superset (`build-ffi` is `ffi || nuget`,
    feeding both `test-ffi` and `pack-nuget`), so no job can now run against artifacts that were
    never built. Recorded in `decisions.md` as an invariant future jobs must preserve

**Issues resolved (deleted from issues.md):**

- "Fix broken single-registry re-trigger in release.yml" (`normal` `[human]`) — no `**Spec:**`
    field, so no spec edit needed. Its follow-up ask ("update the `release-workflow.md` memory") is
    done: the user-level project memory outside the repo no longer calls the path BROKEN and now
    states the guards landed in iter 139, statically verified
- "Local ruff hooks skip `.pyi` files" (`normal` `[review]`) — staged-probe verified above
- Also repaired the now-dangling cross-reference in the dependency-refresh issue (the GHA-refs
    remainder no longer suggests bundling with the retired `if:`-guard issue)

**Open issues: 6 — 0 critical, 4 normal, 2 low.** One
(`Freeze-rule ordering diverges from iscc-core on sequences`) still carries a HUMAN REVIEW REQUESTED
block parked on Titusz.

**Codex review:** One P3, no blockers — it independently confirmed the guards "preserve
failure/cancellation blocking" and that the `.pyi` hook coverage is valid. Its only finding:
`.claude/agent-memory/advance/MEMORY.md` grew to 142 lines against its own declared "keep under 140"
budget (line 8). Not fixed here — that file belongs to the advance agent; it should trim or push
detail into a topic file next iteration.

**Next:** The **97-ref GHA `uses:` refresh in `release.yml`**, now a purely mechanical bump with the
behavioural change already isolated in its own commit: checkout v4→v7, `upload-artifact` ↔
`download-artifact` moved as a pair, setup-java/node/dotnet/python, and `setup-uv` pinned to the
exact `@v9.0.0` (it publishes no floating major past v7). Verify statically the same way this step
was: YAML parse, `actionlint@v1.7.7`, artifact-name consistency across build → test → publish, and
`grep`-confirm that no `if:` guard was disturbed. **Worth bundling:** crystallize this iteration's
guard invariant as an executable check — the `yaml.safe_load` + shape-regex script from next.md is
already written and green at HEAD; landing it as a small `scripts/` file wired into a prek/CI step
would make silent guard regressions in the one file no CID push exercises impossible. That was
beyond this review's minor-fix bar (it changes gate config), so it needs a scoped package.

**Notes:**

- **This file is not exercised by any CI or CID push.** Both this step and the queued GHA refresh
    are statically verified only; first real-world confirmation is the next release run. Do not let
    a future handoff describe the re-trigger path as *proven* — `SKILL.md` deliberately says
    "verified statically … first real-world confirmation comes with the next release run".
- Advance's two doc edits that went one sentence beyond the letter of next.md (the SKILL.md Phase-1
    blockquote and a lead-in clause) were both necessary: the blockquote repeated the retired bug
    and would have contradicted the rewritten section on the same page. Correct judgment, inside
    scoped files.
- Iterations 134–139 have all been tooling/CI configuration. Real library work still has not moved
    since iteration 133 and cannot until the parked Unicode ruling lands. After the GHA refresh the
    tooling thread is genuinely exhausted — the defensible next move is the Rust-core-only boundary
    fixture (format + loader, three single-code-point cases, **no** binding wiring), which the
    sequence-ordering question does not affect.
- The `.pre-commit-config.yaml` comment above `ruff-format` still reads "covers the same surface as
    the CI `ruff format --check` step". With `pyi` added the hook is now a strict *superset* (extra:
    a tracked-but-gitignored `.claude/plans/*.md`). Left as-is — the intent (parity) holds and
    rewording it was not worth another format/commit cycle. Fold it into a future edit of that file
    if one comes along.
