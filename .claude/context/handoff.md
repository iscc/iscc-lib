# Handoff

## 2026-07-25 — Review of: ruff 0.16 slice C — isort first-party config, sorted `__all__`, enforce `I`/`RUF022`/`RUF100`

**Verdict:** PASS

**Summary:** A clean, mechanical gate *strengthening*. Three ruff settings (`[tool.ruff] src`,
`[tool.ruff.lint.isort] combine-as-imports`, and `I`/`RUF022`/`RUF100` added to `extend-select`)
plus 7 auto-applied import/`__all__` fixes take the ruff-0.16 baseline from 12 findings to exactly
3\. Every next.md criterion reproduces green in this session, no behaviour changed, and the diff
stayed inside the 3-file non-test budget.

**Verification:**

- [x] `uv run ruff check` exits 0 covering `I`/`RUF022`/`RUF100` — `All checks passed!`
- [x] `uv run ruff format --check` exits 0 — `25 files already formatted`
- [x] `uvx ruff@0.16.0 check . --output-format concise` reports **exactly 3** errors (`RUF007`,
    `PLW1510`, `EXE001`); `grep -Ec 'I001|RUF022'` → `0`
- [x] `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both exit 0
- [x] `grep -c 'noqa: S603' tools/cid.py` → `6`; `grep -c 'noqa: S607' tools/cid.py` → `4` — no
    load-bearing directive lost
- [x] `grep -q 'ruff<0.16' pyproject.toml` exits 0 — pin retained, `uv.lock` untouched
- [x] tomllib assertion exits 0 (`src`, `combine-as-imports`, full `extend-select`, no `select` key)
- [x] `__all__` assertion exits 0 — 49 unique, all resolvable. Additionally diffed `__all__` as a
    *set* across `HEAD~1..HEAD` via `ast.parse`: removed `set()`, added `set()` — a pure reorder
- [x] `uv run pytest -q` → **314 passed** in 35s
- [x] `mise run check` — all 15 hooks Passed, `git status --porcelain` afterwards shows only the
    runner-owned `iterations.jsonl` (mdformat rewrote nothing this cycle)
- [x] `grep -q 'RUF022' docs/development.md` exits 0

Extra gates run beyond next.md (all green, none required by the package): `uv run ty check`,
`cargo clippy --workspace --all-targets -- -D warnings` (no Rust changed in the unpushed range), and
a **hook-mode probe** — the pre-commit `ruff-check` hook invokes `uv run ruff check --fix` with
*filenames* and without `--force-exclude`, so hook-mode could in principle disagree with
`ruff check .`; re-running it by hand on all 6 touched paths left `git diff` empty, proving `src`
and isort resolve identically per-file.

**Issues found:**

- (none blocking) The `# held:` comment's new claim was verified against the live 0.16 run rather
    than taken on faith — all three named findings and their files are correct. This comment has now
    gone stale twice across slices; sub-slice D retires it entirely.
- **Process note, not a defect:** next.md step 5 told the advance agent to append a progress
    paragraph to `.claude/context/issues.md`, which the advance protocol forbids. Advance correctly
    refused and quoted the exact ledger text in its handoff Notes; I appended it to the issue in
    this commit. Recorded in learnings.md so define-next stops issuing that instruction — the ledger
    step belongs in the handoff, addressed to review.

**Codex review:** No actionable findings. Verdict: *"The Ruff configuration and mechanical
import/export ordering changes are consistent and introduce no functional regressions. Both Ruff
versions behaved as expected, all exports remained valid, and the affected tests passed."* Matches
my own analysis.

**Next:** Continue slice 8 with the **three ruff-0.16 one-liners**, then sub-slice D. All three are
in `scripts/`/`tools/` (non-test, non-doc) so they fit one 3-file step:

1. `RUF007` in `scripts/gen_unicode16_unassigned.py:83` → `itertools.pairwise`. **Must** re-run
    `uv run --script scripts/gen_unicode16_unassigned.py` afterwards and assert
    `git status --porcelain <generated .rs>` is empty — this file generates the vendored Unicode
    freeze table.
2. `PLW1510` in `scripts/test_install.py:53` → explicit `check=False` (confirm the call site really
    tolerates a non-zero exit before choosing `False` over `True`).
3. `EXE001` on `tools/cid.py:1` — shebang without exec bit. Mind the Windows bind-mount caveat in
    memory (`core.fileMode=false` hides dropped exec bits) before reaching for `chmod +x`; deleting
    the shebang is the other option, but `tools/cid.py` is invoked via `mise`/`uv run`, so check
    whether anything executes it directly first.

Then sub-slice D: `uv lock --upgrade-package ruff`, drop the `ruff<0.16` pin and its `# held:`
comment once `uvx ruff@0.16.0 check .` exits 0, and re-run the full pre-push set.

If a Unicode step is preferred instead, note that both remaining Unicode criteria are still parked
behind the open `[review]` issue "Freeze-rule ordering diverges from iscc-core on sequences" (HUMAN
REVIEW REQUESTED) — that ruling should land before boundary vectors are wired into 11 bindings.

**Notes:**

- The two isort settings are load-bearing and now documented inline in `pyproject.toml`: without
    `src`, `iscc_lib` merges into the third-party block; without `combine-as-imports`, the
    `_lowlevel` re-export block shatters into ~60 statements. Do not "tidy" either away.
- Behaviour change worth remembering: `ruff check --fix` now **auto-sorts imports at commit time**.
    A future advance agent adding an import to `__init__.py` will see the pre-commit hook move it.
- `RUF100` being selected means every `# noqa` in the repo is now load-bearing-or-flagged. Before
    deleting any directive, still prove it dead with `--select <rule> --ignore-noqa`.
- No Rust, Go, Ruby, JVM or .NET surface was touched, so the binding gates were correctly skipped.
- `.claude/context/iterations.jsonl` remains modified in the working tree (runner-owned, unstaged).
