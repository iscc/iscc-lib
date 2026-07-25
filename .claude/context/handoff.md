# Handoff

## 2026-07-25 — Review of: ruff 0.16 sub-slice D — clear the last three findings (RUF007, PLW1510, EXE001)

**Verdict:** PASS

**Summary:** Three mechanical one-liner fixes, exactly as scoped: `itertools.pairwise` in the
Unicode freeze-table generator, an explicit `check=False` on the `subprocess.run` in
`scripts/test_install.py`, and the exec bit on `tools/cid.py`. `uvx ruff@0.16.0 check .` exits 0 for
the first time, the vendored Unicode table regenerates byte-identical, and every criterion in
next.md reproduces green in this session. Nothing in `Not In Scope` was touched — `pyproject.toml`
and `uv.lock` are untouched and the `ruff<0.16` pin is intact.

**Verification:**

- [x] `uvx ruff@0.16.0 check . --output-format concise` → `All checks passed!`, exit 0 (was 3)
- [x] `uv run ruff check` exit 0 (`All checks passed!`); `uv run ruff format --check` exit 0
    (`25 files already formatted`)
- [x] `uv run --script scripts/gen_unicode16_unassigned.py` → `wrote … (731 ranges)`, exit 0;
    `git status --porcelain crates/` empty afterwards — freeze table byte-identical
- [x] `grep -q 'from itertools import pairwise'` hits; `grep -c 'zip('` → `0`
- [x] `grep -q 'check=False'` hits; `grep -c 'noqa: S603' scripts/test_install.py` → `1`
- [x] `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both exit 0
- [x] `test -x tools/cid.py` exit 0; `git ls-files -s tools/cid.py` → `100755 8be62cc…` (blob hash
    unchanged, so the commit really is mode-only)
- [x] `uv run tools/cid.py status` exit 0
- [x] `grep -q 'ruff<0.16' pyproject.toml` exit 0; `git status --porcelain pyproject.toml uv.lock`
    empty
- [x] `uv run pytest -q` → **314 passed** in 28.5s
- [x] `mise run check` — all 15 hooks Passed; `git status --porcelain` afterwards shows only the
    runner-owned `iterations.jsonl`

Extra probes beyond next.md (all green): `uv run ruff check --select S603 --ignore-noqa` still
reports the line, so the retained `# noqa: S603` is genuinely load-bearing (not left as cargo cult);
`grep -n cid.py mise.toml` confirms all 10 tasks invoke it as `uv run tools/cid.py`, so the exec bit
is additive only; `git ls-files -s -- '*.py'` filtered by shebang shows `tools/cid.py` is now the
only shebang'd tracked Python file and it is `100755` — repo-consistent. Gate-circumvention sweep
over the whole unpushed range (`@{upstream}..HEAD`, 4 commits) found no added suppression, skip,
threshold or exclusion.

**Issues found:**

- (none) The diff is 3 non-test/non-doc files — exactly at the budget — and the semantic claims hold
    up: `pairwise(xs)` ≡ `zip(xs, xs[1:])` for all input lengths including 0 and 1, and
    `check=False` is the `subprocess.run` default, so both edits are provably behaviour-preserving.
    The `check_invariants` loop's pre-existing quirk (the last range is never inversion-checked) is
    unchanged by the rewrite — not introduced here, and the `EXPECTED_*` totals catch a bad table
    anyway.

**Codex review:** No actionable findings. Verdict: *"The changes are behavior-preserving and
compatible with the project's Python requirements. Ruff 0.16 passed, the generated Unicode table
remained byte-identical, and all 314 tests passed."* Matches my own analysis.

**Next:** **Sub-slice E — the last piece of the ruff 0.16 adoption**, and it is a one-file step:

1. `uv lock --upgrade-package ruff` (uv.lock only), and delete the `ruff<0.16` pin line plus its
    now-fully-stale `# held:` comment from `pyproject.toml` line 47 — replace with a plain `ruff`
    entry in the dev dependency group.
2. Re-run the full pre-push set with 0.16 as the *project* formatter and default linter. The real
    risk is **`ruff format` drift** — the pinned 0.15.22 reports "25 files already formatted", and
    the 0.16 formatter may reflow some of them. If it does, the reformat belongs in this step (it
    is the cost of the upgrade), but say so explicitly in the handoff rather than letting it look
    like unrelated churn.
3. Never blanket `--fix`. `uvx ruff@0.16.0 check .` is clean at HEAD, so nothing needs fixing — the
    only reason to run `--fix` at all would be a formatter change, which `ruff format` handles.

After E, slice 8 closes and the dependency-refresh issue has only human/major-gated remainders left
(`release.yml` GHA refs — best bundled with the existing `if:`-guard issue — plus the `jni` 0.22,
`magnus` 0.8, xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1 and JUnit 6.x majors, each its
own step). The two Unicode criteria remain parked behind the open `[review]` issue "Freeze-rule
ordering diverges from iscc-core on sequences" (HUMAN REVIEW REQUESTED); that ruling should land
before boundary vectors are wired into 11 bindings.

**Notes:**

- **Ledger appended:** the sub-slice D paragraph from the advance handoff Notes is now in issues.md
    under slice 8, and the stale "sub-slice D drops the pin" wording elsewhere in that issue was
    reconciled to the current A–E naming (D = the three one-liners, E = the pin drop).
- The exec-bit change is index metadata only. On this checkout `core.fileMode=false`, so
    `git status` will never show it — verify with `git ls-files -s` or `git diff --summary`.
    Committing one requires **both** `chmod +x` and `git update-index --chmod=+x`; recorded in
    learnings.md because the failure mode is silent.
- The `# held:` comment on line 47 of `pyproject.toml` now names three findings that no longer
    exist. Intentional (next.md scoped it to E), but it is actively misleading until E lands — worth
    doing soon rather than letting another slice queue in front of it.
- No Rust, Go, Ruby, JVM, .NET or WASM surface was touched and no hot path was involved, so the
    CRAP, iai-callgrind, semver and binding gates were correctly skipped. No coverage change either
    — neither edited script is under pytest coverage.
- learnings.md was at its 200-line budget; two ruff satellite entries were merged and the main ruff
    entry tightened to make room for the exec-bit gotcha (still 200 lines, no archival needed).
- `.claude/context/iterations.jsonl` remains modified in the working tree — runner-owned, unstaged.
