# Next Work Package

## Step: ruff 0.16 slice B — enforce `S` + `C901` project-wide via `extend-select`, clear all RUF100

## Goal

Move the Ruff security (`S`) and complexity (`C901`) rules from two pre-push-only hook invocations
into the project's default lint selection, so CI (`uv run ruff check`) enforces them for the first
time and the 15 load-bearing `# noqa: S603/S607` directives stop being reported as unused by ruff
0.16. This is the "`# noqa` / security-gate cluster" sub-slice of the `normal` `[human]` issue
"Dependency review and refresh across the project" (ledger slice **B**; state.md calls it "slice C"
— same work, naming drift).

Deliberately **not** the Unicode boundary-vector slice: an open `normal` `[review]` issue
("Freeze-rule ordering diverges from iscc-core on sequences", HUMAN REVIEW REQUESTED) asks for the
spec-wording ruling before boundary vectors are wired out, and the Rust core already has inline
`#[test]`s for all three boundary code points — so the marginal value of a Rust-only fixture is low
while the propagation it exists to feed is parked.

## Scope

- **Modify**: `pyproject.toml`, `tools/cid.py`, `tools/metrics.py` (3 non-test, non-doc files)
- **Modify (docs, outside the file budget)**: `docs/development.md`
- **Reference**: `.pre-commit-config.yaml` (the `security` / `complexity` pre-push hooks),
    `.github/workflows/ci.yml` lines 48–71 (the Python job runs bare `uv run ruff check`),
    `mise.toml` `[tasks.lint]`, `.claude/context/issues.md` (the dependency-refresh issue),
    `.claude/context/specs/ci-cd.md` → "Local Development" / "Pre-commit Hooks (prek)"

## Not In Scope

- **Do not drop the `ruff<0.16` pin** from `[dependency-groups]` and do not run `uv lock` /
    `uv lock --upgrade-package ruff`. 12 findings survive this slice; the pin and its `# held:`
    comment stay until the tree is clean under 0.16.
- **Do not fix the remaining 12 ruff-0.16 findings** — `I001` ×8 + `RUF022` (the isort `src`-root
    decision, ledger slice C), `RUF007` in `scripts/gen_unicode16_unassigned.py`, `PLW1510` in
    `scripts/test_install.py`, `EXE001` on `tools/cid.py`. Each needs its own decision and its own
    step.
- **Do not run `uvx ruff@0.16 check --fix .`** anywhere. It deletes the load-bearing
    `# noqa: S603/S607` directives in `tools/` and `scripts/` and reds the pre-push security gate.
- **Do not remove or relax the `security` / `complexity` pre-push hooks** in
    `.pre-commit-config.yaml`. They now overlap with `ruff check`, and that redundancy is
    intentional: they pass `--select` explicitly, so they keep working regardless of project config,
    and they name the failing gate in the push output.
- **Do not add `PLC0415`, `I`, or any other rule family** to the selection — those are separate
    decisions with their own fallout (`PLC0415` alone adds 5 new findings in `tests/`).
- **Do not touch any Unicode / freeze-rule work** (boundary vectors, differential sweep, Go tables)
    — parked on a human ruling.
- **Do not edit `.claude/context/issues.md`** — the review agent annotates slice progress there.

## Implementation Notes

**1. `pyproject.toml` — add the selection.** Insert a `[tool.ruff.lint]` table *above* the existing
`[tool.ruff.lint.mccabe]` table (TOML permits a super-table after a sub-table, but taplo-friendly
ordering avoids churn):

```toml
[tool.ruff.lint]
# `S` (security) and `C901` (complexity) are project gates. Selecting them here — instead of
# only in the two pre-push hooks — makes `uv run ruff check` (pre-commit, `mise run lint`, CI)
# enforce them too, and keeps the load-bearing `# noqa: S603/S607` directives recognised.
extend-select = ["S", "C901"]
```

Use `extend-select`, **never** `select`: `select` *replaces* ruff's default (`E4`, `E7`, `E9`, `F`)
and would silently drop pyflakes coverage. `[tool.ruff.lint.mccabe] max-complexity = 15` and the
existing `[tool.ruff.lint.per-file-ignores]` for `tests/**` (`S101`, `S603`, `S607`) stay exactly as
they are — they are what keeps the test suite green under the new selection.

**2. `tools/metrics.py` — delete one now-genuinely-unused directive.** In `git_sha()` (~line 174),
change `out = subprocess.run(  # noqa: S603` to `out = subprocess.run(`. Keep the explanatory
comment line above it and keep the `# noqa: S607` on the argv line below. Rationale: ruff 0.16
refined `S603` so it no longer fires when the command is a static list literal
(`["git", "rev-parse", …]`) — this was verified with `--select S` under **both** 0.15.22 and 0.16.0
with the directive removed, both "All checks passed". Every other `# noqa: S603` in `tools/` and
`scripts/` sits on a *dynamic* argv (`list(argv)`, a `cmd` parameter) and **must be kept**.

**3. `tools/cid.py` — delete the stale `PLC0415` directive.** In the Windows branch (~line 980),
change `import msvcrt  # noqa: PLC0415` to `import msvcrt`. `PLC0415` is not in ruff's default
select and this step does not add it, so the directive is dead weight that ruff 0.16 reports as
`RUF100`. Leave the surrounding docstring/comment explaining the platform-conditional import
untouched.

**4. `docs/development.md` — keep the gate description accurate.** In "Pre-commit (fast, auto-fix on
every commit)" (~line 137) note that `ruff check --fix` now covers the security (`S`) and complexity
(`C901`) rules via `extend-select` in `pyproject.toml`, and in the "Pre-push" list note that the two
Ruff entries are a focused re-run of the same rules. Two short edits — do not restructure the
section.

**Expected end state, measured while scoping** (with the three edits applied):

- `uvx ruff@0.15.22 check .` → `All checks passed!` (this is the pinned version in the venv)
- `uvx ruff@0.16.0 check .` → **exactly 12 errors**, down from 27, with **zero** `RUF100`
- `--select S` and `--select C901` (the pre-push gates) both exit 0
- `ruff format --check` unchanged: `25 files already formatted`

## Verification

- `uv run ruff check` exits 0 (now includes `S` + `C901`)
- `uv run ruff format --check` exits 0
- `uv run ruff check --select S --force-exclude` exits 0 (pre-push security gate unchanged)
- `uv run ruff check --select C901 --force-exclude` exits 0 (pre-push complexity gate unchanged)
- `uvx ruff@0.16.0 check . --output-format concise` prints `Found 12 errors.`
- `uvx ruff@0.16.0 check . --output-format concise | grep -c RUF100` prints `0`
- `uv run python -c "import tomllib,pathlib; c=tomllib.loads(pathlib.Path('pyproject.toml').read_text())['tool']['ruff']['lint']; assert {'S','C901'} <= set(c['extend-select']); assert 'select' not in c"`
    exits 0
- `grep -q 'ruff<0.16' pyproject.toml` exits 0 (the hold-back pin is retained by this step)
- `grep -c 'noqa: S603' tools/cid.py` prints `6` and `grep -c 'noqa: S607' tools/cid.py` prints `4`
    (the load-bearing directives survive)
- `uv run python -m compileall -q tools/cid.py tools/metrics.py` exits 0
- `mise run cid:status` exits 0 (`tools/cid.py` still runs after the edit)
- `uv run pytest -q` passes
- `mise run check` — all hooks pass with nothing rewritten
- `grep -q 'C901' docs/development.md` exits 0 and the pre-commit bullet mentions `S` / `C901`

## Done When

`S` and `C901` are enforced by the project's default ruff selection (and therefore by CI), all
`RUF100` findings under ruff 0.16 are gone with every load-bearing `# noqa` intact, and all
verification commands above pass on the working tree.
