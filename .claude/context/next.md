# Next Work Package

## Step: ruff 0.16 slice C — isort first-party config, sorted `__all__`, enforce `I`/`RUF022`/`RUF100`

## Goal

Clear the 9-finding isort cluster that blocks the `ruff<0.16` pin (8 × `I001` + 1 × `RUF022`) by
teaching ruff that `iscc_lib` is first-party, and promote `I`, `RUF022` and `RUF100` into the
project lint selection so import order is enforced *now* under the pinned 0.15.22 rather than
silently arriving with the 0.16 upgrade. This is sub-slice C of the `normal` `[human]` issue
"Dependency review and refresh across the project" (slice 8, ruff 0.16 adoption).

## Scope

- **Modify** (3 non-test source files, at budget):
    - `pyproject.toml` — add `[tool.ruff] src`, `[tool.ruff.lint.isort] combine-as-imports`, extend
        the lint selection, refresh the `# held:` comment on the `ruff<0.16` pin
    - `crates/iscc-py/python/iscc_lib/__init__.py` — mechanical fix (one blank line + `__all__` order)
    - `benchmarks/python/bench_iscc_lib.py` — mechanical fix (one blank line)
- **Modify** (tests/docs, excluded from the file budget): `tests/test_algo.py`,
    `tests/test_benchmarks.py`, `tests/test_smoke.py`, `tests/test_text_utils.py`,
    `docs/development.md`
- **Reference**: `.pre-commit-config.yaml` (the two pre-push `--select S` / `--select C901` hooks —
    do not touch), `.claude/context/issues.md` (issue text + slice-8 progress log),
    `.claude/context/handoff.md` (review "Next" section)

## Not In Scope

- **Do not drop the `ruff<0.16` pin and do not run `uv lock`.** Three findings survive this slice
    (`RUF007`, `PLW1510`, `EXE001`); the pin retires in sub-slice D once `uvx ruff@0.16.0 check .`
    exits 0.
- **Do not fix the three one-liners** (`scripts/gen_unicode16_unassigned.py`,
    `scripts/test_install.py`, `tools/cid.py`) — they are 3 more non-test files and would blow the
    budget. They are their own step.
- **Never run `ruff@0.16 check --fix .` (or any blanket `--fix` with an unpinned ruff).** It deletes
    the 14 load-bearing `# noqa: S603/S607` directives in `tools/`/`scripts/` and reddens the
    pre-push security gate. Fix with the pinned `uv run ruff` and an explicit `--select I,RUF022`.
- Do not simplify, merge or delete the pre-push `S`/`C901` hooks — the redundancy is a recorded
    decision (`decisions.md`, 2026-07-25).
- Do not add further rule families (`PLC0415` in particular adds 5 `tests/` findings), and do not
    introduce `[tool.ruff.lint] select` — it would replace ruff's `E4`/`E7`/`E9`/`F` defaults.
- No Unicode work. Both remaining Unicode criteria (the differential sweep and the boundary vectors)
    are parked behind the open `[review]` issue "Freeze-rule ordering diverges from iscc-core on
    sequences" (HUMAN REVIEW REQUESTED).

## Implementation Notes

All settings below were probed live against the working tree with `--config` overrides — no guessing
needed.

**1. `pyproject.toml` config.** Insert a `[tool.ruff]` table *before* the existing
`[tool.ruff.lint]` table, and an isort table after it:

```toml
[tool.ruff]
# `iscc_lib` ships from `crates/iscc-py/python`, not the repo root. Listing it here makes isort
# resolve it as first-party instead of lumping it in with third-party imports.
src = [".", "crates/iscc-py/python"]

[tool.ruff.lint]
extend-select = ["S", "C901", "I", "RUF022", "RUF100"]

[tool.ruff.lint.isort]
# Keep the single `from iscc_lib._lowlevel import (X as X, ...)` re-export block intact —
# without this, isort explodes it into ~60 one-member import statements.
combine-as-imports = true
```

Both isort settings are load-bearing and were verified:

- Without `src`, `iscc_lib` is treated as third-party and merges with `iscc_core`/`pytest`.
- Without `combine-as-imports`, the `__init__.py` fix produces a **110-line** diff that shatters the
    re-export block; with it, the fix is a single deleted blank line.

`RUF100` is green at HEAD today (probed: `uv run ruff check --extend-select RUF100` reports
nothing), so adding it costs zero fixes and prevents stale `# noqa` directives from accumulating
unseen — the exact failure mode sub-slice B had to clean up.

**2. Apply the fixes** with the *pinned* ruff, scoped:

```
uv run ruff check --fix --select I,RUF022 .
```

Expected: `Found 7 errors (7 fixed, 0 remaining).` across 6 files — `__init__.py` (blank line +
`__all__` reorder), `benchmarks/python/bench_iscc_lib.py`, `tests/test_algo.py`,
`tests/test_benchmarks.py`, `tests/test_smoke.py`, `tests/test_text_utils.py`. All are safe fixes;
no `--unsafe-fixes` needed. `ruff format --check` stays clean afterwards (verified on a scratch copy
of all 6 files).

The `RUF022` fix only reorders the 49 strings in `__all__` (`"__version__"` moves after
`"VideoCodeResult"`). Nothing in the repo reads `__all__` positionally — verified by grep.

**3. Refresh the hold-back comment** on the `ruff<0.16` dev-dependency so it names what is actually
left: `RUF007` (`scripts/gen_unicode16_unassigned.py`), `PLW1510` (`scripts/test_install.py`),
`EXE001` (`tools/cid.py`).

**4. Docs.** `docs/development.md` lines ~137–138 currently say the pre-commit `ruff check --fix`
covers `S` and `C901` via `extend-select`. Extend that sentence to name import sorting (`I`),
`RUF022` and `RUF100`. Behavioural consequence worth stating: `ruff check --fix` now **auto-sorts
imports at commit time**.

**5. Ledger.** Append the slice-8 sub-slice-C result to the "Dependency review and refresh" issue's
progress paragraph in `.claude/context/issues.md` (do not delete the issue — the review agent owns
resolution).

## Verification

- `uv run ruff check` exits 0 — now covering `I`, `RUF022`, `RUF100` in addition to `S`, `C901`
- `uv run ruff format --check` exits 0
- `uvx ruff@0.16.0 check . --output-format concise` reports **exactly 3 errors**, and
    `uvx ruff@0.16.0 check . --output-format concise | grep -Ec 'I001|RUF022'` → `0`
- `uv run ruff check --select S --force-exclude` exits 0 and
    `uv run ruff check --select C901 --force-exclude` exits 0 (pre-push gates intact)
- `grep -c 'noqa: S603' tools/cid.py` → `6` and `grep -c 'noqa: S607' tools/cid.py` → `4` (no
    load-bearing directive was deleted by a stray `--fix`)
- `grep -q 'ruff<0.16' pyproject.toml` exits 0 (the pin is retained; slice D drops it)
- tomllib assertion exits 0:
    `uv run python -c "import tomllib,pathlib; c=tomllib.loads(pathlib.Path('pyproject.toml').read_text())['tool']['ruff']; assert c['src']==['.','crates/iscc-py/python']; assert c['lint']['isort']['combine-as-imports'] is True; assert {'S','C901','I','RUF022','RUF100'} <= set(c['lint']['extend-select']); assert 'select' not in c['lint']"`
- `uv run python -c "import iscc_lib; a=iscc_lib.__all__; assert len(a)==49==len(set(a)); assert all(hasattr(iscc_lib,n) for n in a)"`
    exits 0 (the `__all__` reorder neither dropped nor duplicated a symbol)
- `uv run pytest -q` → 314 passed
- `mise run check` — all hooks pass and nothing is rewritten (re-run `git status --porcelain` after
    it to confirm no hook-applied changes remain unstaged)
- `grep -q 'RUF022' docs/development.md` exits 0

## Done When

`uv run ruff check` and `uv run ruff format --check` are clean with `I`/`RUF022`/`RUF100` enforced,
`uvx ruff@0.16.0 check .` is down to exactly the 3 non-isort findings, the full test suite passes,
and the pin's hold-back comment plus `docs/development.md` describe the new state accurately.
