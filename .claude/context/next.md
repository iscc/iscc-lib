# Next Work Package

## Step: ruff 0.16 adoption — slice A: clear the config-free findings (stub `...` + unused unpacked vars)

## Goal

Start the final dependency-refresh item ("Dependency review and refresh across the project",
`normal` `[human]`) by clearing the 78 ruff-0.16 findings that need **no configuration decision and
no gate interaction**: 72 stub-style errors in `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`
(PIE790 + PYI048) and 6 `RUF059` unused-unpacked-variable findings in `tests/test_new_symbols.py`.
This is a deliberate slice of the ruff 0.16 adoption — the remaining 26 findings each carry a real
decision (isort source-root config, and the `# noqa: S603/S607` directives that the pre-push
security hook depends on) and get their own steps. The `ruff<0.16` pin stays in place until the tree
is fully clean.

## Scope

- **Modify**:
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — the only non-test source file in this step
        (file budget: 1 of 3)
    - `tests/test_new_symbols.py` (test file, budget-free)
    - `crates/iscc-py/CLAUDE.md` (doc, budget-free) — add one bullet recording the stub-body
        convention so the `...` lines are not reintroduced with the next Tier 1 symbol
- **Reference**: `.claude/context/issues.md` (the dependency-refresh issue, "Progress" section),
    `pyproject.toml` (the `ruff<0.16` hold-back comment on line 27), `.pre-commit-config.yaml` (the
    pre-push `ruff check --select S` and `--select C901` hooks)

## Not In Scope

- **Do NOT drop or loosen the `ruff<0.16` pin** in `pyproject.toml` and do NOT touch `uv.lock`. With
    26 findings still open the pin must stay; retiring it is the last slice.
- **Do NOT run a blanket `uvx ruff@0.16.0 check --fix .`.** It would delete the `# noqa: S603` /
    `# noqa: S607` directives in `tools/cid.py`, `tools/metrics.py` and `scripts/test_install.py`
    (ruff 0.16 reports them as `RUF100` only because `S` is not in the default select). Those
    directives are load-bearing for the pre-push `ruff check --select S` hook — removing them turns
    the security gate red. Leave `tools/` and `scripts/` completely untouched this step.
- **Do NOT touch `crates/iscc-py/python/iscc_lib/__init__.py`** (its `I001` + `RUF022` findings need
    the isort config decision) and do NOT fix the `I001` findings in `tests/` or
    `benchmarks/python/bench_iscc_lib.py`.
- **Do NOT add any lint suppression or config widening** — no `# noqa`, no new `per-file-ignores`,
    no `[tool.ruff.lint]` `ignore` entries, no `select` changes. Nothing in `pyproject.toml` changes
    at all this step.
- No Rust source, no PyO3 `lib.rs` change, no API change — this is a type-stub and test cleanup
    only.

## Implementation Notes

**1. `_lowlevel.pyi` (72 errors → 0).** Every stub function/method body is a docstring followed by a
lone `...` placeholder line. In a `.pyi` file that is two statements, so ruff 0.16 (whose default
rule set now includes `PIE` and `PYI`) reports `PIE790 Unnecessary '...' literal` **and**
`PYI048 Function body must contain exactly one statement` for the same line. The fix is to delete
every line whose stripped content is exactly `...` — 36 lines total (27 at 4-space indent for the
module-level functions, 9 at 8-space indent for the methods of the 3 hasher classes). Nothing else
in the file changes: keep all 27 `def`, all 3 `class`, all 9 methods, every docstring and every
signature byte-identical.

This was prototyped during scoping: removing exactly those 36 lines makes
`uvx ruff@0.16.0 check --isolated` on the file exit 0, keeps `ruff format` a no-op (the file stays
"already formatted" — no stray double blank lines appear), and `ty check` still passes. A
docstring-only body is a valid stub body.

**2. `tests/test_new_symbols.py` (6 `RUF059` → 0).** Four `iscc_decode(...)` tuple unpackings bind
names that are never asserted on. Prefix each unused binding with `_`. The exact edit (verified with
`uvx ruff@0.16.0 check --diff --select RUF059 --unsafe-fixes tests/test_new_symbols.py`):

- line ~57 → `mt, st, vs, _li, decoded_digest = iscc_decode(f"ISCC:{encoded}")`
- line ~68 → `mt, _st, _vs, _li, decoded_digest = iscc_decode(encoded)`
- line ~340 → `mt, st, vs, _length, decoded_digest = iscc_decode(encoded)`
- line ~350 → `mt, st, vs, _length, digest = iscc_decode("GAA2XTPPAERUKZ4J")`

You may apply these with
`uvx ruff@0.16.0 check --select RUF059 --fix --unsafe-fixes tests/test_new_symbols.py` (the fix is
classified "unsafe" only because it renames bindings) or by hand — the result must match the four
lines above. Do not delete the assertions or restructure the tests.

**3. `crates/iscc-py/CLAUDE.md`.** Extend the "When adding a Tier 1 function" area with one bullet
stating the convention, e.g. "`_lowlevel.pyi` stub bodies are a docstring only — no trailing `...`
placeholder (ruff `PIE790`/`PYI048`)". Keep it to one bullet; no restructuring of the file.

**Working with two ruff versions.** The repo's pinned ruff is 0.15.22 (`uv run ruff`); 0.16.0 is
reachable without touching the lock via `uvx ruff@0.16.0`. Both must stay green: 0.15 is what
`mise run lint` and the pre-commit hooks execute today, 0.16 is the target. `uvx` needs network on
first use; it was exercised during scoping so the cache is warm.

## Verification

- `uvx ruff@0.16.0 check crates/iscc-py/python/iscc_lib/_lowlevel.pyi` exits 0 ("All checks
    passed!")
- `uvx ruff@0.16.0 check --select PIE790,PYI048,RUF059 .` exits 0 (whole tree clean for these three
    rules)
- `uvx ruff@0.16.0 check . --statistics` reports **exactly 26** findings — `RUF100` 15, `I001` 8,
    `EXE001` 1, `PLW1510` 1, `RUF022` 1 — down from 104, with no `PIE790`, `PYI048` or `RUF059` left
- `grep -c '^[[:space:]]*\.\.\.[[:space:]]*$' crates/iscc-py/python/iscc_lib/_lowlevel.pyi` prints
    `0`
- On `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: `grep -c '^def '` prints `27`,
    `grep -c '^class '` prints `3`, and `grep -c '^    def '` prints `9` (no signature lost)
- `uv run ruff check .` (pinned 0.15.22) exits 0 and `uv run ruff format --check .` exits 0
- `uv run ruff check --select S --force-exclude` exits 0 (pre-push security gate intact)
- `uv run ruff check --select C901 --force-exclude` exits 0 (pre-push complexity gate intact)
- `grep -c noqa tools/cid.py` prints `11`, `grep -c noqa tools/metrics.py` prints `4`,
    `grep -c noqa scripts/test_install.py` prints `1` (the security `noqa` directives are untouched)
- `uv run ty check` exits 0
- `uv run pytest -q` passes with 314 tests collected, 0 failures
- `grep -c 'ruff<0.16' pyproject.toml` prints `1` and `grep -c 'held: 0.16 expands' pyproject.toml`
    prints `1` (the hold-back and its comment are still in the working tree, not retired here)
- `mise run check` exits 0 with nothing rewritten
- `cargo test -p iscc-lib` still passes (no Rust touched — sanity only)

## Done When

`_lowlevel.pyi` and `tests/test_new_symbols.py` are clean under ruff 0.16, the whole-tree 0.16
finding count is down to the 26 that still need a config or gate decision, and every existing gate
(`mise run check`, the pinned-ruff lint, the `S`/`C901` pre-push scans, `ty`, pytest) is still green
with the `ruff<0.16` pin still in place.
