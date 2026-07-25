# Handoff

## 2026-07-25 — ruff 0.16 sub-slice D — clear the last three findings (RUF007, PLW1510, EXE001)

**Done:** Cleared the three remaining ruff-0.16 findings: `itertools.pairwise` replaces
`zip(ranges, ranges[1:])` in the Unicode freeze-table generator (RUF007), the `subprocess.run` call
in `scripts/test_install.py`'s `run()` helper got an explicit `check=False` (PLW1510), and
`tools/cid.py` gained the exec bit to match its shebang (EXE001, file mode only — no content
change). `uvx ruff@0.16.0 check .` now exits 0 for the first time.

**Files changed:**

- `scripts/gen_unicode16_unassigned.py`: added `from itertools import pairwise` to the stdlib import
    block; `check_invariants` loop now uses `pairwise(ranges)`. Generator re-run —
    `crates/iscc-lib/src/utils/unicode16.rs` is byte-identical (see Verification).
- `scripts/test_install.py`: `check=False` added to the `subprocess.run` call in `run()`; the
    `# noqa: S603` directive preserved. `check=False` matches existing behaviour — callers inspect
    `result.returncode` (20 `returncode` sites per next.md's scoping; `check=True` would change
    behaviour by raising).
- `tools/cid.py`: mode change `100644 => 100755` only. Applied via both `chmod +x` (local FS) and
    `git update-index --chmod=+x` (index), since `core.fileMode=false` on this 9p bind mount makes a
    plain chmod invisible to git. Blob hash unchanged (`8be62cc` before and after).

**Verification:** Every next.md criterion reproduced green in this session:

- `uvx ruff@0.16.0 check . --output-format concise` → `All checks passed!`, exit 0 (was 3 errors)
- `uv run ruff check` → exit 0; `uv run ruff format --check` → `25 files already formatted`
- `uv run --script scripts/gen_unicode16_unassigned.py` → `wrote ... (731 ranges)`;
    `git status --porcelain crates/` empty afterwards (freeze table byte-identical)
- `grep 'from itertools import pairwise'` hits; `grep -c 'zip('` → 0 in the generator
- `grep 'check=False'` hits; `grep -c 'noqa: S603'` → 1 in `scripts/test_install.py`
- `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both exit 0
- `test -x tools/cid.py` OK; `git ls-files -s tools/cid.py` → `100755 8be62cc...`
- `uv run tools/cid.py status` → exit 0 (runner still runnable)
- `grep 'ruff<0.16' pyproject.toml` hits; `git status --porcelain pyproject.toml uv.lock` empty (pin
    drop is sub-slice E, untouched here)
- `uv run pytest -q` → **314 passed** in 30s
- `mise run check` → exit 0, all 15 hooks Passed; porcelain afterwards shows only the runner-owned
    `iterations.jsonl` beyond this step's files

**Next:** Sub-slice E — `uv lock --upgrade-package ruff`, drop the `ruff<0.16` pin and its (now
fully stale) `# held:` comment from `pyproject.toml`, then re-run the full pre-push set with 0.16 as
the project formatter/default linter. Watch for 0.16 formatter drift (`ruff format --check`) and
never blanket `--fix` — the `# noqa: S603/S607` directives are load-bearing.

**Notes:**

- **Ledger paragraph for the review agent** (advance must not edit issues.md) — please append to
    "Dependency review and refresh across the project" slice 8: *Sub-slice D (iter 136): last three
    ruff-0.16 findings cleared — RUF007 via `itertools.pairwise` in
    `scripts/gen_unicode16_unassigned.py` (generator re-run, `unicode16.rs` byte-identical), PLW1510
    via explicit `check=False` in `scripts/test_install.py` (behaviour-preserving), EXE001 via exec
    bit on `tools/cid.py` (`git update-index --chmod=+x`, mode 100755 committed).
    `uvx ruff@0.16.0 check .` exits 0 for the first time. Remaining: sub-slice E (drop the
    `ruff<0.16` pin + stale `# held:` comment, `uv lock --upgrade-package ruff`).*
- The exec-bit change is index-metadata only; verify with `git diff HEAD~1 --summary` (shows
    `mode change 100644 => 100755 tools/cid.py`) rather than a content diff. On this checkout
    `core.fileMode=false`, so `git status` will not report the mode locally — trust
    `git ls-files -s`.
- `pairwise(xs)` ≡ `zip(xs, xs[1:])` including empty/one-element inputs; the PEP 723 header's
    `requires-python = ">=3.10"` covers it (pairwise added in 3.10).
- No Rust/binding surface touched; no hot path involved — bench, semver, and binding gates were
    correctly skipped. No CRAP-baseline refresh needed (no Python coverage change; the two edited
    scripts are not under coverage).
- The `# held:` comment on the `ruff<0.16` pin now names three findings that no longer exist —
    intentionally left stale per next.md scope; sub-slice E retires the whole line.
