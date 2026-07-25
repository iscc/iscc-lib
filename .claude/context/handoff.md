# Handoff

## 2026-07-25 — ruff 0.16 slice C — isort first-party config, sorted `__all__`, enforce `I`/`RUF022`/`RUF100`

**Done:** Added `[tool.ruff] src = [".", "crates/iscc-py/python"]` and
`[tool.ruff.lint.isort] combine-as-imports = true` to `pyproject.toml`, extended the lint selection
to `extend-select = ["S", "C901", "I", "RUF022", "RUF100"]`, then applied the mechanical fixes with
the *pinned* ruff via `uv run ruff check --fix --select I,RUF022 .` — exactly the predicted
`Found 7 errors (7 fixed, 0 remaining)` across 6 files. Refreshed the `# held:` comment on the
`ruff<0.16` pin to name the 3 surviving findings, and updated `docs/development.md` to document the
expanded rule set and commit-time import auto-sorting.

**Files changed:**

- `pyproject.toml`: added `[tool.ruff]` (src) and `[tool.ruff.lint.isort]` (combine-as-imports)
    tables with load-bearing comments; `extend-select` now `["S", "C901", "I", "RUF022", "RUF100"]`;
    `# held:` comment rewritten to name `RUF007`/`PLW1510`/`EXE001` with their files
- `crates/iscc-py/python/iscc_lib/__init__.py`: one blank line removed after the `_lowlevel` import
    block (kept intact by `combine-as-imports`); `__all__` reordered (`"__version__"` moved after
    `"VideoCodeResult"`, `"IsccResult"` to sorted position) — 49 unique symbols, none dropped
- `benchmarks/python/bench_iscc_lib.py`: blank line inserted between `import pytest` and the
    first-party `iscc_lib` import block
- `tests/test_algo.py`, `tests/test_benchmarks.py`, `tests/test_smoke.py`,
    `tests/test_text_utils.py`: mechanical import-order fixes only (blank-line placement,
    `iscc_core` grouped with third-party, `IsccResult` alphabetized)
- `docs/development.md`: pre-commit `ruff check --fix` bullet now names `S`, `C901`, `I`, `RUF022`,
    `RUF100` and states imports are auto-sorted at commit time

**Verification:** All next.md criteria reproduce green in this session:

- `uv run ruff check` → `All checks passed!` (exit 0, now covering `I`/`RUF022`/`RUF100`)
- `uv run ruff format --check` → `25 files already formatted` (exit 0)
- `uvx ruff@0.16.0 check . --output-format concise` → **exactly 3 errors** (`RUF007`, `PLW1510`,
    `EXE001`), zero `I001`/`RUF022`
- `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both exit 0
- `grep -c 'noqa: S603' tools/cid.py` → 6; `grep -c 'noqa: S607' tools/cid.py` → 4
- `grep -q 'ruff<0.16' pyproject.toml` → pin retained
- tomllib assertion exits 0 (`src`, `combine-as-imports`, full `extend-select` set, no `select` key)
- `__all__` assertion exits 0 (49 == len(set), all names resolvable)
- `uv run pytest -q` → **314 passed**
- `mise run check` → all 15 hooks Passed; `git status --porcelain` afterwards shows only my intended
    changes plus runner-owned `iterations.jsonl` (nothing rewritten by hooks)
- `grep -q 'RUF022' docs/development.md` → exit 0

**Next:** The three ruff-0.16 one-liners (their own step per next.md): `RUF007` in
`scripts/gen_unicode16_unassigned.py` (→ `itertools.pairwise`; re-run the generator and assert the
generated Rust is unchanged), `PLW1510` in `scripts/test_install.py` (explicit `check=False`),
`EXE001` on `tools/cid.py` (mind the Windows-bind-mount exec-bit caveat before choosing `chmod +x`).
Then sub-slice D: `uv lock --upgrade-package ruff`, drop the pin once `uvx ruff@0.16.0 check .`
exits 0.

**Notes:**

- **issues.md ledger not written by advance (rule conflict, deliberate):** next.md step 5 asked me
    to append the sub-slice-C result to the "Dependency review and refresh" issue's progress
    paragraph, but my protocol forbids modifying `.claude/context/issues.md`, and the slice-B review
    explicitly counted "issues.md untouched by advance" as scope discipline. Ledger text for the
    review agent to append: *Slice 8 sub-slice C (2026-07-25): isort cluster cleared —
    `[tool.ruff]   src` + `combine-as-imports` added, `I`/`RUF022`/`RUF100` promoted into
    `extend-select`, 7 mechanical fixes applied with pinned 0.15.22. `uvx ruff@0.16.0 check .` now
    reports exactly 3 findings (RUF007, PLW1510, EXE001). Pin retained; sub-slice D drops it.*
- Every Not-In-Scope fence held: no `uv lock`, no ruff@0.16 `--fix`, the three one-liners untouched,
    pre-push `S`/`C901` hooks verbatim, no `select` key introduced, no new rule families beyond the
    three named, no Unicode work.
- `RUF100` was green at HEAD before this change (slice-B review verified) and stays green — adding
    it cost zero fixes.
- `combine-as-imports` did exactly what next.md predicted: the `_lowlevel` re-export block survived
    intact; without it the diff would have been ~110 lines.
- `.claude/context/iterations.jsonl` is modified in the working tree (runner-owned) and was left
    unstaged, as in prior iterations.
