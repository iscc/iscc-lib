## 2026-02-25 — Review of: Add `__version__` attribute and fix iscc-py module docstring

**Verdict:** PASS

**Summary:** Clean, minimal implementation adding `__version__` via `importlib.metadata.version()`
and fixing three incorrect module name references in `lib.rs`. Two `[human]`-sourced issues
resolved. All 159 Python tests pass, 14 pre-commit hooks pass, clippy clean.

**Verification:**

- [x] `pytest tests/test_smoke.py` passes — 19 tests (17 existing + 2 new), all pass
- [x] `pytest tests/` passes — 159 tests (19 smoke + 140 conformance), all pass
- [x] `python -c "import iscc_lib; print(iscc_lib.__version__)"` prints `0.0.1`
- [x] `grep -q 'iscc_lib._lowlevel' crates/iscc-py/src/lib.rs` exits 0 — docstring fixed
- [x] `grep 'iscc\._lowlevel' ... | grep -qv 'iscc_lib'` exits non-zero — no old references remain
- [x] `cargo clippy -p iscc-py -- -D warnings` clean

**Issues found:**

- (none)

**Next:** Remaining `[low]` issues: iscc-wasm conformance feature gate, iscc-wasm stale CLAUDE.md,
iscc-jni exception mapping, and TypeScript port evaluation. Define-next should pick the next
highest-value item.

**Notes:** The advance agent found and fixed a third incorrect `iscc._lowlevel` reference on line
360 (the `#[pymodule]` docstring) that wasn't in next.md's scope — this was correctly caught by the
verification grep and is within the spirit of the docstring fix task. Total `__all__` count is now
34 symbols (33 API + `__version__`).
