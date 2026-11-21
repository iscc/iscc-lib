## 2026-11-21 — Review of: Cut the pytest gate time: benchmarks disabled by default, Tier 1 doc tests copy only what they read

**Verdict:** PASS

**Summary:** The pytest gate dropped from about 45 s to about 20 s. `pyproject.toml` disables
pytest-benchmark timing loops by default, the documented benchmark command gained
`--benchmark-enable`, and the Tier 1 doc tests read the repo sources once per session. The diff
touches exactly the three files next.md named and changes no assertion.

**Verification:**

- [x] `uv run pytest` exits 0 — 763 passed in 20.62 s (25 s wall) on my run. Top of
    `--durations=10`: tier1 `test_real_repo_passes` 1.09 s, tier1 `test_fire_pub_fn_added_to_lib_rs`
    0.65 s, `test_check_ci_job_table.py::test_real_repo_passes` 0.42 s, everything else ≤ 0.40 s
- [x] `uv run pytest tests/test_benchmarks.py` exits 0 without the stats header — 18 passed in 0.68
    s, `Name (time in` count 0
- [x] The `docs/benchmarks.md` command contains `--benchmark-enable`, exits 0 under `uv run` and
    prints the stats table — 18 passed in 13.92 s, 9 tables. Without `--benchmark-enable` it exits 4
    with the "Can't have both" usage error, so the doc edit was required
- [x] `uv run pytest tests/test_check_tier1_docs.py` exits 0 — 36 passed. Read in full: the 13
    unrecognised-item cases and the floor test use `_copy_lib` (lib.rs only), and `_tree_bytes` is
    `functools.cache`d. `run_checks` returns on both paths before the doc loop
    (`scripts/check_tier1_docs.py`), and a regression there would raise `FileNotFoundError`
- [x] `uv run zensical build` exits 0 — "No issues found"
- [x] `mise run check` exits 0

Beyond next.md: `uv run ty check` and ruff `S`/`C901` on the changed test file are clean. Every
pytest invocation in the repo (`ci.yml` `uv run pytest`, the pre-push hook, `mise.toml`) runs with
the dev group, which holds `pytest-benchmark`, so the ini-wide `addopts` cannot hit an "unrecognized
arguments" error there. No gate was weakened: benchmarked calls still run once each.

**Issues found:**

- (none filed) Two nits, not worth an entry: a bare `pytest benchmarks/python/` now runs each
    benchmark once and prints no timings, where the `conftest.py` docstring and `notes/09` mention
    that form. `_copy_tree` and `_copy_lib` keep "copy" in their names although they write cached
    bytes

**Issue resolved:** "Pytest gate spends ~25 s of ~44 s on benchmark loops and doc-tree copies"
(`normal` `[audit]`) is deleted from `issues.md`.

**Codex review:** No actionable findings. It ran the full suite (763 passed) and the enabled
benchmark command (18 passed, timing tables printed).

**Next:** The C FFI `(NULL, 0)` unification (`normal` `[audit]`, scope estimate 2-4 non-test files)
as its own step. It changes FFI behaviour and flips len-0 null tests on the C, .NET and C++
surfaces, so run all three suites, regenerate `iscc.h` and `NativeMethods.g.cs` in the same step if
a doc comment on an export moves, and add `data-edge.json` vectors only if the change is
JSON-expressible. After it: the integer-range issue, then the published-docs issue.

**Notes:**

- Three `normal` `[audit]` issues remain and block v0.8.0: C FFI `(NULL, 0)`, out-of-range integers,
    published-package docs
- `issues.md` is at 268 of 300 lines, `learnings.md` at its 200-line budget: a new learning needs an
    equal cut in the same commit
- To see benchmark numbers from pytest, pass `--benchmark-enable`. This holds for
    `tests/test_benchmarks.py` as well as for `benchmarks/python/`
