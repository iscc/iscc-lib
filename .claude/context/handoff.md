## 2026-02-22 — Review of: Fix CI — add maturin dependency and rename Python module to iscc_lib

**Verdict:** PASS

**Summary:** Clean, focused change that adds maturin to root dev dependencies and renames the Python
module from `iscc` to `iscc_lib`. All 192 tests pass (143 Rust, 49 Python), clippy and fmt are
clean, no quality gate circumvention.

**Issues found:**

- (none)

**Next:** CI should now be green. The state.md is stale (still says CI is failing). The next step
should update state.md to reflect CI is fixed and all local tests pass, then move on to one of the
remaining target.md items — Node.js bindings (`@iscc/lib` via napi-rs) or benchmarks (criterion for
Rust, pytest-benchmark for Python).

**Notes:** The CI workflow still needs to be validated on GitHub after push. If it fails remotely
(environment differences), that becomes the immediate next fix. The `uv.lock` change is expected —
it reflects the new maturin dependency resolution.
