# Next Work Package

## Step: ruff 0.16 sub-slice D — clear the last three findings (RUF007, PLW1510, EXE001)

## Goal

Fix the three remaining ruff-0.16 findings so `uvx ruff@0.16.0 check .` exits 0 for the first time,
which is the precondition for sub-slice E (dropping the `ruff<0.16` hold-back pin). This closes the
last locally-verifiable part of the "Dependency review and refresh across the project" issue's slice
8\.

## Scope

- **Modify**:
    - `scripts/gen_unicode16_unassigned.py` — `RUF007`: replace `zip(ranges, ranges[1:])` with
        `itertools.pairwise(ranges)` (line 83, in `check_invariants`)
    - `scripts/test_install.py` — `PLW1510`: add an explicit `check=False` to the `subprocess.run`
        call in `run()` (line 53)
    - `tools/cid.py` — `EXE001`: **file mode only**, no content change (see Implementation Notes)
- **Reference**:
    - `.claude/context/issues.md` → "Dependency review and refresh across the project" (slice 8 ledger
        paragraph)
    - `.claude/context/handoff.md` → the three numbered items under "Next"
    - `pyproject.toml` lines ~44–50 (`[tool.ruff]`, `extend-select`, the `ruff<0.16` `# held:` pin)
    - `crates/iscc-lib/src/utils/unicode16.rs` — the generator's output (must stay byte-identical)

## Not In Scope

- **Do not touch `pyproject.toml` or `uv.lock`.** Dropping the `ruff<0.16` pin, retiring its
    now-fully-stale `# held:` comment, and `uv lock --upgrade-package ruff` are sub-slice E — a
    separate step whose decision content (does the repo survive 0.16 as the *project* formatter and
    default rule set?) is different from these three mechanical fixes.
- **Do not run `ruff check --fix .` (or `ruff@0.16 --fix`) without an explicit `--select`.** A
    blanket fix deletes load-bearing `# noqa: S603/S607` directives and reddens the security gate.
    This trap has been documented across two prior slices.
- Do not delete the shebang from `tools/cid.py` — adding the exec bit is the chosen resolution (the
    repo already tracks `100755` for every other shebang'd script: `.devcontainer/*.sh`,
    `scripts/build_xcframework.sh`, `packages/kotlin/gradlew`).
- Do not add any new rule to `extend-select` this step, and do not "tidy" the isort `src` /
    `combine-as-imports` settings — both are load-bearing (documented inline).
- Do not edit `crates/iscc-lib/src/utils/unicode16.rs` by hand, and do not change the freeze table,
    its invariants, or `EXPECTED_*` constants.
- Do not append progress notes to `.claude/context/issues.md` — the advance protocol forbids it. Put
    the ledger update in your handoff Notes addressed to the review agent instead.

## Implementation Notes

Findings verified live at HEAD (`uvx ruff@0.16.0 check . --output-format concise` → exactly 3):

```text
scripts/gen_unicode16_unassigned.py:83:35: RUF007 Prefer `itertools.pairwise()` over `zip()`
scripts/test_install.py:53:12: PLW1510 `subprocess.run` without explicit `check` argument
tools/cid.py:1:1: EXE001 Shebang is present but file is not executable
```

**1. `RUF007` — `scripts/gen_unicode16_unassigned.py`**

Add `from itertools import pairwise` to the stdlib import block (it sorts *before*
`from pathlib import Path`; the `I` rule is now enforced and the pre-commit hook auto-sorts), and
change line 83 to:

```text
for (lo, hi), (next_lo, _) in pairwise(ranges):
```

`pairwise(xs)` and `zip(xs, xs[1:])` are exactly equivalent, including the empty/one-element cases.
`requires-python = ">=3.10"` in the PEP 723 header covers `itertools.pairwise` (added in 3.10).
Apply this by hand or with the *pinned* ruff and an explicit selector
(`uv run ruff check --select RUF007 --unsafe-fixes --fix scripts/gen_unicode16_unassigned.py`) —
never a blanket `--fix`.

This file generates the vendored Unicode freeze table, so **re-run the generator and prove the
output is unchanged**: `uv run --script scripts/gen_unicode16_unassigned.py` (takes \<1 s, cache is
warm, network works) then assert `git status --porcelain crates/iscc-lib/src/utils/unicode16.rs` is
empty.

**2. `PLW1510` — `scripts/test_install.py`**

The `run()` helper's callers inspect `result.returncode` at 20 call sites (`grep -n returncode`), so
the call site genuinely tolerates a non-zero exit → `check=False` is correct; `check=True` would
change behaviour by raising. Keep the `# noqa: S603` on the `subprocess.run(` line — `RUF100` is
enforced, and the directive is still load-bearing:

```python
return subprocess.run(  # noqa: S603
    cmd,
    capture_output=True,
    text=True,
    cwd=cwd,
    timeout=timeout,
    env=run_env,
    check=False,
)
```

Let `uv run ruff format` decide the final line wrapping.

**3. `EXE001` — `tools/cid.py`** (file mode only; do not edit the file's content)

`core.fileMode` is `false` in this checkout (9p Windows bind mount), so a plain `chmod +x` is
invisible to git. Both commands are required:

```bash
chmod +x tools/cid.py                      # so ruff/prek see it locally
git update-index --chmod=+x tools/cid.py   # so mode 100755 lands in the commit
```

Run `git update-index --chmod=+x` **after** any `git add`, immediately before `git commit`, and
confirm with `git diff --cached --summary` → `mode change 100644 => 100755 tools/cid.py`. This
round-trip was probed during scoping and works cleanly. Nothing invokes `tools/cid.py` directly —
all 10 `mise.toml` tasks use `uv run tools/cid.py …` — so the exec bit is additive only. There is no
`check-executables-have-shebangs` hook in `.pre-commit-config.yaml`, so nothing else reacts.

Ruff's project pin stays at 0.15.22; none of RUF007/PLW1510/EXE001 is in its enforced set, so
`uv run ruff check` is green both before and after — the 0.16 command is the meaningful gate here.

## Verification

- `uvx ruff@0.16.0 check . --output-format concise` exits 0 and prints `All checks passed!` (was: 3
    errors)
- `uv run ruff check` exits 0 and `uv run ruff format --check` exits 0 (pinned 0.15.22)
- `uv run --script scripts/gen_unicode16_unassigned.py` exits 0 and prints `731 ranges`; afterwards
    `git status --porcelain crates/iscc-lib/src/utils/unicode16.rs` prints nothing and
    `git status --porcelain crates/` prints nothing
- `grep -q 'from itertools import pairwise' scripts/gen_unicode16_unassigned.py` exits 0 and
    `grep -c 'zip(' scripts/gen_unicode16_unassigned.py` → `0`
- `grep -q 'check=False' scripts/test_install.py` exits 0 and
    `grep -c 'noqa: S603' scripts/test_install.py` → `1` (directive preserved)
- `uv run ruff check --select S --force-exclude` and
    `uv run ruff check --select C901 --force-exclude` both exit 0
- `test -x tools/cid.py` exits 0 and `git ls-files -s tools/cid.py` starts with `100755`
- `uv run tools/cid.py status` exits 0 (runner still runnable after the mode change)
- `grep -q 'ruff<0.16' pyproject.toml` exits 0 and `git status --porcelain pyproject.toml uv.lock`
    prints nothing (the pin drop is sub-slice E, not this step)
- `uv run pytest -q` passes (314 tests at HEAD)
- `mise run check` — all hooks pass, and `git status --porcelain` afterwards shows no hook-applied
    rewrites beyond the runner-owned `.claude/context/iterations.jsonl`

## Done When

`uvx ruff@0.16.0 check .` reports zero findings, the Unicode freeze table regenerates
byte-identical, the `ruff<0.16` pin is still in place, and every verification command above passes.
