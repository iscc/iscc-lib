## 2026-02-23 — Review of: Add pytest-benchmark for iscc-core Python reference baseline

**Verdict:** PASS

**Summary:** pytest-benchmark tests for all 9 `gen_*_v0` iscc-core functions were added cleanly.
Inputs match the Rust criterion benchmarks, all 9 benchmarks execute successfully with full
statistical output, and existing tests (143 Rust, 49 Python) remain unaffected. Code is clean, well
documented, and follows the benchmark plan from `notes/09-performance-benchmarks.md`.

**Issues found:**

- (none)

**Next:** Phase 0 benchmarks are complete (both Rust criterion and Python reference baseline). The
next logical step is either: (1) **Node.js bindings** (`@iscc/lib` via napi-rs) — the next major
deliverable per target.md, or (2) **Python bindings benchmarks** (Phase 1 — benchmark `iscc_lib`
PyO3 bindings vs `iscc-core` to compute speedup factors). Node.js bindings is the larger remaining
target item and would advance the project toward multi-language support.

**Notes:** Python baseline numbers (containerized): Meta ~807μs, Text ~2.6ms, Image ~4.2ms, Audio
~1.1ms, Video ~137μs, Mixed ~78μs, Data ~6.5ms (64KB), Instance ~71μs (64KB), ISCC-CODE ~87μs. These
will serve as the denominator for speedup factor computation once the bindings benchmarks are added.
The `benchmarks/python/` directory is correctly excluded from the default `pytest` test suite via
explicit `testpaths = ["tests"]` in pyproject.toml.
