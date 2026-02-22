## 2026-02-22 — Review of: Expose all 9 gen\_\*\_v0 functions in PyO3 bindings with type stub

**Verdict:** PASS

**Summary:** All 8 remaining `gen_*_v0` functions are correctly exposed through PyO3 with proper
type mappings, default arguments, and docstrings. The `_lowlevel.pyi` type stub and `__init__.py`
re-exports complete the Python bindings. All quality gates pass: 143 Rust tests, clippy clean, fmt
clean, ty type checking, pytest (3 smoke tests), and all pre-commit/pre-push hooks green.

**Issues found:**

- (none)

**Next:** The Python bindings are functionally complete (all 9 `gen_*_v0` functions exposed). The
most impactful next step is adding Python-level conformance tests that exercise all 9 functions
against the vendored `data.json` vectors via pytest. This validates the full pipeline (Python → PyO3
→ Rust → result) and catches any type conversion bugs. After that, consider CI/CD setup (GitHub
Actions) or benchmarks.

**Notes:** The `next.md` had an incorrect expected value for `gen_meta_code_v0('Hello')` — it
expected `ISCC:AAAWKLHFPV6OPKDG` but the correct output (verified against all 16 conformance
vectors) is `ISCC:AAAWKLHFXM75OAMK`. This is not a code issue; the expected value in next.md was
wrong. State.md should be updated to reflect that Python bindings are now complete (all 9 functions
exposed, ty passes). Only 3 smoke tests exist on the Python side — conformance test coverage from
Python is the main gap.
