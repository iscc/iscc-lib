# Handoff

## 2026-07-25 — ruff 0.16 adoption — slice A: clear the config-free findings (stub `...` + unused unpacked vars)

**Done:** Cleared the 78 ruff-0.16 findings that need no configuration decision: deleted all 36 lone
`...` placeholder lines from `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (each was
double-reported as `PIE790` + `PYI048` = 72 errors; docstring-only stub bodies remain) and
`_`-prefixed the 6 `RUF059` unused unpacked bindings in `tests/test_new_symbols.py`. Added the
stub-body convention bullet to `crates/iscc-py/CLAUDE.md`. The `ruff<0.16` pin stays in place — 26
findings remain for the follow-up slices.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: removed exactly 36 lines whose stripped content
    was `...` (verified via a Python filter, `removed 36 lines, 440 remain`); every signature,
    docstring, and blank line untouched
- `tests/test_new_symbols.py`: 4 `iscc_decode` unpackings — `_li` (line 57), `_st, _vs, _li` (line
    68), `_length` (lines 340, 350); assertions unchanged
- `crates/iscc-py/CLAUDE.md`: one bullet in Common Pitfalls — stub bodies are docstring-only, no
    trailing `...` (`PIE790`/`PYI048`)

**Verification:** All next.md criteria reproduced in this session:

- `uvx ruff@0.16.0 check crates/iscc-py/python/iscc_lib/_lowlevel.pyi` → "All checks passed!"
- `uvx ruff@0.16.0 check --select PIE790,PYI048,RUF059 .` → "All checks passed!"
- `uvx ruff@0.16.0 check . --statistics` → exactly 26 errors: RUF100 15, I001 8, EXE001 1, PLW1510
    1, RUF022 1 (down from a re-measured 104 baseline; exit 1 is expected — those 26 are
    deliberately left for later slices)
- Stub integrity greps: lone-`...` count 0, `^def` 27, `^class` 3, `^    def` 9
- Pinned ruff 0.15.22: `uv run ruff check .` exit 0, `uv run ruff format --check .` "24 files
    already formatted"
- Pre-push gates intact: `--select S` exit 0, `--select C901` exit 0; noqa counts unchanged
    (tools/cid.py 11, tools/metrics.py 4, scripts/test_install.py 1)
- `uv run ty check` → "All checks passed!"
- `uv run pytest -q` → 314 passed
- Pin intact: `grep -c 'ruff<0.16' pyproject.toml` → 1, `grep -c 'held: 0.16 expands'` → 1
- `mise run check` → all 15 hooks Passed, nothing rewritten (working tree afterwards: only the 3
    scoped files + runner-owned iterations.jsonl)
- `cargo test -p iscc-lib` exit 0 (sanity; no Rust touched)

**Next:** Slice B/C of ruff 0.16: (1) the isort decision — 8 `I001` + 1 `RUF022` findings in
`__init__.py`, `tests/`, and `benchmarks/python/bench_iscc_lib.py` likely need a
`[tool.ruff.lint.isort]` src/known-first-party setting rather than blind `--fix`; (2) the 15
`RUF100` findings — these are the load-bearing `# noqa: S603/S607` directives that the pre-push
`--select S` gate depends on; 0.16 flags them only because `S` is not in default select, so the
resolution is a config decision (e.g., adding S/C901 to the lint select so the noqas stay "used"),
NOT deletion. Plus `EXE001` (shebang on a non-executable file) and `PLW1510` (`subprocess.run`
without `check=`). Only after all 26 are resolved: drop the `ruff<0.16` pin and refresh `uv.lock`.

**Notes:**

- The `...` removal was done with a deterministic Python line filter (strip == `...`), not a blanket
    `ruff --fix`, so `tools/` and `scripts/` were provably untouched — confirmed by
    `git diff   --stat` showing only the 3 scoped files.
- `PIE790`+`PYI048` double-count the same 36 lines, which is why 72 errors collapse to 36 deletions.
- `ruff format` treats the docstring-only stub bodies as already formatted — no blank-line churn, as
    next.md's scoping prototype predicted.
- No API, perf, or conformance surface touched; no benchmark or semver run owed.
