# Handoff

## 2026-07-25 — ruff 0.16 slice B — enforce `S` + `C901` project-wide via `extend-select`, clear all RUF100

**Done:** Added `[tool.ruff.lint] extend-select = ["S", "C901"]` to `pyproject.toml` so the
project's default ruff selection (and therefore `uv run ruff check` in pre-commit, `mise run lint`,
and CI) enforces the security and complexity gates that were previously pre-push-only. Deleted the
two directives ruff 0.16 correctly reports as unused: the `# noqa: S603` in `tools/metrics.py`
`git_sha()` (0.16 refined S603 to skip static list-literal argv) and the stale `# noqa: PLC0415` in
`tools/cid.py` (rule never in select). Updated `docs/development.md` to keep the gate description
accurate.

**Files changed:**

- `pyproject.toml`: new `[tool.ruff.lint]` table with `extend-select = ["S", "C901"]` (placed above
    `[tool.ruff.lint.mccabe]`), with a comment explaining why `extend-select` and not `select`.
    `ruff<0.16` pin, mccabe config, and `tests/**` per-file-ignores untouched.
- `tools/metrics.py`: removed `# noqa: S603` from the `subprocess.run(` line in `git_sha()`;
    explanatory comment above and `# noqa: S607` on the argv line kept.
- `tools/cid.py`: removed `# noqa: PLC0415` from `import msvcrt` (line 980); surrounding docstring
    untouched.
- `docs/development.md`: pre-commit bullet notes `ruff check --fix` now covers `S`/`C901` via
    `extend-select`; the two pre-push Ruff bullets note they are focused re-runs of the same rules.
    (mdformat re-wrapped the first edit on its auto-fix pass — cosmetic only.)

**Verification:** All next.md criteria reproduced green on the working tree this session:

- `uv run ruff check` → `All checks passed!`, exit 0 (now includes `S` + `C901`)
- `uv run ruff format --check` → `25 files already formatted`, exit 0
- `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both exit 0
- `uvx ruff@0.16.0 check . --output-format concise` → `Found 12 errors.` (down from 27); code
    breakdown exactly as scoped: `I001` ×8, `RUF022` ×1, `RUF007` ×1, `PLW1510` ×1, `EXE001` ×1;
    `grep -c RUF100` → **0**
- tomllib assertion passes: `{'S','C901'} <= extend-select` and no `select` key
- `grep -q 'ruff<0.16' pyproject.toml` exits 0 (hold-back pin retained)
- `grep -c 'noqa: S603' tools/cid.py` → 6; `grep -c 'noqa: S607' tools/cid.py` → 4 (load-bearing
    directives intact)
- `uv run python -m compileall -q tools/cid.py tools/metrics.py` exits 0
- `mise run cid:status` exits 0
- `uv run pytest -q` → 314 passed
- `mise run check` → all hooks Passed. First run: mdformat re-wrapped my `docs/development.md` edit
    (auto-fix working as designed); second run: all hooks Passed with nothing rewritten
- `grep -q 'C901' docs/development.md` exits 0; the pre-commit bullet names both `S` and `C901`

**Next:** Ledger slice C — the isort cluster (`I001` ×8 + `RUF022`), which needs the src-root /
`[tool.ruff.lint.isort]` configuration decision. The other three leftovers (`RUF007`, `PLW1510`,
`EXE001`) are one-liner decisions that could ride along in the same or a follow-up step. Once all 12
are cleared, the `ruff<0.16` pin can drop with `uv lock --upgrade-package ruff`.

**Notes:**

- Stayed inside the Not-In-Scope fence: pin untouched, no `uv lock`, no `--fix` under 0.16, the two
    pre-push hooks in `.pre-commit-config.yaml` kept verbatim, no new rule families, no Unicode
    work, `issues.md` untouched.
- The 14 remaining `# noqa: S603` directives across `tools/` and `scripts/` all sit on dynamic argv
    (`list(argv)`, `cmd` parameters) and are now *recognised* (not unused) because `S` is in the
    default selection — that is what cleared the 15 RUF100 findings without deleting load-bearing
    suppressions.
- The two deletions are suppression *removals* (directives that no longer suppress anything), the
    opposite of gate-weakening; the `S` gate now runs in strictly more places than before.
- `.claude/context/iterations.jsonl` is modified in the working tree (runner-owned); left unstaged
    per protocol.
