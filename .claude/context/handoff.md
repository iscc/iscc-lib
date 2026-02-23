## 2026-02-23 — Review of: Add comparative pytest-benchmark for iscc_lib Python bindings

**Verdict:** PASS

**Summary:** The advance agent created `bench_iscc_lib.py` with 9 benchmarks mirroring
`bench_iscc_core.py` exactly — identical inputs, matching group names, correct API type adaptations.
All 18 benchmarks pass together with grouped comparison showing Rust speedup factors (1.1x–5.5x).
Code is clean, simple, and well-documented. No regressions.

**Issues found:**

- Removed unused `collect_ignore = []` from conftest.py (minor fix applied)

**Next:** Update `state.md` to reflect completed benchmark comparison. The remaining target items
are: documentation site (lib.iscc.codes) and CI/CD publishing pipelines (OIDC trusted publishing for
crates.io, PyPI, npm). These are independent workstreams — either can be tackled next. The
documentation site is the larger gap; CI/CD publishing is more mechanical but enables releases.

**Notes:** The benchmark comparison is now functional but not published anywhere — running
`pytest benchmarks/python/ --benchmark-only` produces grouped results locally. Consider whether
benchmark results should be captured in CI or published to GitHub Pages as part of the documentation
site work. The `text_code` benchmark shows only ~1.1x speedup because Python text normalization
(Unicode operations) dominates compute time — this is expected and not a performance concern.
