# Handoff

## 2026-07-25 — Review of: Close the ruff/Markdown gate-parity gap in the prek hooks

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent widened the `ruff-format` prek hook to `types_or: [python, markdown]`
with an evergreen comment, and added a one-sentence qualification to the hook lists in
`docs/development.md` and `CLAUDE.md`. One non-doc file, two doc files — exactly the scoped diff,
every Not-In-Scope entry respected, all 12 verification criteria reproduced green in this session.
Reviewing the surface arithmetic independently turned up a residual hole the handoff's "strict
superset" claim hides: prek classifies `.pyi` as the `pyi` type, not `python`, so **both** ruff
hooks skip the published `_lowlevel.pyi` that CI does check. Pre-existing, not a regression, filed
as a `normal` `[review]` issue with a probed two-token fix.

**Verification:**

- [x] `grep -A7 'id: ruff-format' … types_or: \[python, markdown\]` — exit 0
- [x] `grep -A6 'id: ruff-check' … types: \[python\]` — exit 0, lint hook deliberately unchanged
- [x] `uv run prek run check-yaml --files .pre-commit-config.yaml` — Passed
- [x] `uv run prek run yamlfix --files .pre-commit-config.yaml` — Passed, file unchanged
- [x] Markdown-fence probe — staged `probe_gate.md` with a `py` fence containing `x=1` →
    `Failed / files were modified by this hook / 1 file reformatted`, `grep 'x = 1'` → FIXED, tree
    clean afterwards (`git rm --cached -f` needed, `git rm --cached` alone errors once the hook has
    rewritten the worktree copy)
- [x] `uv run ruff format --check` — exit 0, `153 files already formatted` (exact CI command)
- [x] `uv run ruff format --check $(git ls-files '*.md' '*.py' '*.pyi')` — exit 0,
    `154 files already formatted`. **Caveat:** this measures an `ls-files` list, not the hook's real
    surface (see Issues found) — the criterion as written cannot detect the `.pyi` hole
- [x] `uv run ruff check` — `All checks passed!`, exit 0
- [x] `mise run check` — 15/15 hooks Passed, run **twice**, exit 0 both times; a porcelain status
    after each shows only the runner-owned `.claude/context/iterations.jsonl`. No mdformat ↔ ruff
    ping-pong over the 129 tracked `.md` files
- [x] `uv run zensical build` — exit 0, `No issues found` (1.62s)
- [x] `grep -n 'ruff format' docs/development.md CLAUDE.md` — both mention Markdown code blocks
    (development.md:137/140, CLAUDE.md:179/180)
- [x] `git status --porcelain -- crates/ packages/ .github/ mise.toml pyproject.toml uv.lock` and
    the equivalent `git diff HEAD~1..HEAD --name-only` — both empty
- [x] Gate integrity — `git diff @{upstream}..HEAD` over all 4 unpushed commits contains no
    suppression, skip, threshold reduction, hook removal or scope exclusion. The change strictly
    *widens* a gate

**Issues found:**

- **`.pyi` files are invisible to both local ruff hooks** (pre-existing, filed `normal` `[review]`).
    prek's `python` type tag does not match `.pyi` — probed with a staged `probe_gate.pyi`
    containing `import os` + `def f(x:int)->int: ...`: `prek run ruff-check --files` and
    `prek run ruff-format --files` both report `(no files to check) Skipped`, while bare
    `ruff check` on the same file reports `Found 2 errors` and `ruff format` reformats it. So
    `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — consumer-facing (the wheel ships `py.typed`)
    and hand-edited as recently as iter 131 — is gated only by CI. Same trap class this iteration
    just closed for Markdown. Nothing is red today. Fix is two tokens (`pyi` added to both hooks),
    probed green against an alternate prek config.
- **The handoff's "strict superset (154 vs 153)" arithmetic is wrong**, and it had propagated into
    `.claude/agent-memory/advance/deps-refresh.md`. Corrected there in this commit: 154 tracked
    candidates = 129 `.md` + 24 `.py` + 1 `.pyi`; CI's recursive discovery sees 153 (skips the
    tracked-but-gitignored `.claude/plans/*.md`); the hook at `--all-files` also sees 153, but a
    *different* 153 — plans file in, `.pyi` out. Local is neither subset nor superset. Not a
    correctness problem in the shipped config, but a claim that would have calcified.
- Advance-memory compaction (157 → 139 lines) dropped the note that
    `scripts/gen_unicode16_unassigned.py` is `[tool.ty.src]`-excluded. Still captured in
    `learnings.md`; no action needed.

**Codex review:** Clean. "The widened hook configuration is valid, processes Markdown as intended,
and matches the CI formatting check without affecting the Python-only lint hook. The accompanying
documentation accurately reflects the behavior." No actionable findings — it did not probe the
`.pyi` type-tag edge either.

**Next:** Bundle the two remaining `release.yml` issues into one step — the GHA `uses:` refresh (97
refs; `upload-artifact@v4` ↔ `download-artifact@v4` must move together, `setup-uv` needs the exact
`@v9.0.0`) and the broken single-registry re-trigger fix (`!cancelled() && !failure()` guards on
every `test-*` / `publish-*` job, matching `publish-crates-io`). Both touch the same file, neither
is exercised by CID pushes, so both are verified statically (YAML parse + matrix-entry presence +
artifact-name consistency + guard presence). The `.pyi` hook gap filed this iteration is a good
cheap warm-up or tail-end bundle if a smaller package is wanted. Still parked: the Unicode boundary
vectors (blocked on the open HUMAN REVIEW freeze-rule-ordering ruling) and the dependency-refresh
majors (jni 0.22, magnus 0.8, xunit 3.x, Gradle wrapper, JUnit 6.x), each its own migration step.

**Notes:**

- **Verify hook surfaces by probing, not by `git ls-files` arithmetic.** next.md's criterion
    `ruff format --check $(git ls-files '*.md' '*.py' '*.pyi')` looks like it measures the hook but
    measures a hand-built list; the hook's actual surface only shows up under
    `uv run prek run <hook> --files <probe>`. Both new learnings entries encode this.
- prek reports `files were modified by this hook` only for **tracked** files — an untracked probe
    gets silently fixed and reported `Passed`. Stage probes with `git add`, and unstage with
    `git rm --cached -f` (plain `--cached` refuses once the hook has rewritten the worktree copy).
- `ruff format` does not error on a syntactically-invalid Python fence in Markdown — it leaves it
    alone and exits 0 (probed with `def f(:`). Widening the hook therefore cannot break commits over
    illustrative pseudo-code in docs.
- Issue ledger updated as the advance handoff requested: the closed gate-parity issue was deleted
    (history in git) and its resolution folded into the dependency-refresh issue's slice-8 paragraph
    — advance correctly refused to edit `issues.md` itself.
- No Rust, Python, Go, JVM, .NET, Ruby or WASM source touched — no CRAP baseline, iai perf,
    cargo-deny or semver refresh owed, no benchmarked hot path, no API surface change.
- `learnings.md` pruned back to the 200-line budget while absorbing the three new entries.
