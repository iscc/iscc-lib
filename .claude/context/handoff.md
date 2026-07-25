# Handoff

## 2026-07-25 — ruff 0.16 sub-slice E — drop the `ruff<0.16` pin

**Done:** Removed the `ruff<0.16` hold-back pin and its stale `# held:` comment from
`pyproject.toml` (now a plain `"ruff",` entry in the dev group) and relocked with
`uv lock --upgrade-package ruff`, which resolved to exactly 0.16.0. ruff 0.16.0 is now the project's
default linter and formatter. This closes sub-slice E, the last piece of slice 8 of the
dependency-refresh issue.

**Files changed:**

- `pyproject.toml`: line 47 — `"ruff<0.16",  # held: …` → `"ruff",` (1 insertion, 1 deletion)
- `uv.lock`: regenerated — the `{ name = "ruff", specifier = "<0.16" }` requirement entry lost its
    specifier and the `ruff` package block moved 0.15.22 → 0.16.0 with new sdist/wheel hashes.
    Verified nothing else moved: the non-hash diff lines are exactly those two blocks (44 changed
    lines total, all ruff)

**Verification:** Every criterion in next.md reproduced green in this session:

- `grep -c 'ruff<0.16' pyproject.toml` → 0; `grep -c 'held:' pyproject.toml` → 0;
    `grep -q '^  "ruff",$' pyproject.toml` exit 0
- `grep -c 'specifier = "<0.16"' uv.lock` → 0; `grep -A1 '^name = "ruff"$' uv.lock` →
    `version = "0.16.0"`
- `uv run ruff --version` → `ruff 0.16.0`
- `uv run ruff check` → `All checks passed!` (exact CI command)
- `uv run ruff format --check` → exit 0, **`153 files already formatted`** — this is the one real
    behaviour change and it is expected, not accidental scope: ruff 0.16 formats Python code blocks
    inside Markdown, widening the bare-invocation surface from 25 to 153 files. All were already
    clean, so zero reformats landed. next.md pre-measured exactly this number.
- `uv run ruff format --check $(git ls-files '*.md')` → `129 files already formatted`
- `uv run ruff check --select S --force-exclude` and `--select C901 --force-exclude` both →
    `All checks passed!` (the two pre-push gates)
- `git grep -o 'noqa: S60[0-9]' -- '*.py' | wc -l` → 13 (no blanket `--fix` was run; all
    load-bearing directives survive)
- `uv run ty check` → `All checks passed!`
- `uv run pytest -q` → **314 passed** in 25.6s
- `uv run --script scripts/gen_unicode16_unassigned.py` → `wrote … (731 ranges)`, exit 0;
    `git status --porcelain crates/` → 0 lines (freeze table byte-identical)
- `mise run check` — all 15 hooks Passed; `git status --porcelain` afterwards shows only
    `pyproject.toml`, `uv.lock`, and the runner-owned `iterations.jsonl`
- `git status --porcelain -- crates/ packages/ .github/ mise.toml .pre-commit-config.yaml` → 0 lines
    (only the two in-scope files touched)

**Next:** Slice 8 is closed and every locally-verifiable slice of the dependency-refresh issue is
done. The remainders are human/major-gated, each its own step: `release.yml` GHA refs (best bundled
with the existing `if:`-guard issue), and the held majors — `jni` 0.22, `magnus` 0.8, xunit 3.x,
`Microsoft.NET.Test.Sdk` 18.x, Gradle 8.12.1, JUnit 6.x. The parked Unicode boundary-vector criteria
still wait on the open `[review]` freeze-rule-ordering issue (HUMAN REVIEW REQUESTED).

**Notes:**

- **Ledger paragraph for issues.md (review agent, please append under slice 8):** Sub-slice E
    complete (iter 137): `ruff<0.16` pin and `# held:` comment dropped from `pyproject.toml`;
    `uv lock --upgrade-package ruff` moved 0.15.22 → 0.16.0 (only ruff moved in the lock). Zero lint
    findings and zero reformats at the flip — sub-slices A–D had pre-cleared everything. Known
    behaviour change: `ruff format` now also checks Python code blocks in Markdown (bare-invocation
    surface 25 → 153 files, all clean; mdformat prek hook still owns Markdown auto-fixing). Slice 8
    CLOSED — the ruff 0.16 adoption is fully landed.
- Zero formatter churn: the risk flagged in the previous handoff ("0.16 formatter may reflow some of
    the 25 files") did not materialize — `uv run ruff format --check` exits 0 with no reflows, so no
    reformat commits ride along with this bump.
- No Rust, Go, JVM, .NET, Ruby, or WASM surface touched; no hot path involved. CRAP, iai-callgrind,
    semver, and cargo-deny baselines correctly untouched.
- The first `uv run` after the relock swapped ruff in the project venv (uninstall 0.15.22 / install
    0.16.0) — a single-package sync, and the editable `iscc_lib` extension survived (pytest 314
    green confirms), so no `maturin develop` rebuild was needed.
- `.claude/context/iterations.jsonl` remains modified in the working tree — runner-owned, unstaged.
