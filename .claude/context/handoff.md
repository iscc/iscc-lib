## 2026-02-22 — Add pytest-benchmark for iscc-core Python reference baseline

**Done:** Added `pytest-benchmark` and `iscc-core` as dev dependencies, and created
`benchmarks/python/bench_iscc_core.py` with benchmark tests for all 9 `gen_*_v0` functions using
identical inputs to the Rust criterion benchmarks. This completes Phase 0 of the benchmark plan.

**Files changed:**

- `pyproject.toml`: Added `pytest-benchmark` and `iscc-core` to `[dependency-groups] dev`
- `benchmarks/python/bench_iscc_core.py`: Created pytest-benchmark tests for all 9 gen functions
    with matching inputs to the Rust criterion benchmarks

**Verification:**

- `uv sync --group dev` installs successfully (iscc-core 1.2.2, pytest-benchmark 5.2.3)
- `uv run pytest benchmarks/python/bench_iscc_core.py --benchmark-only` — all 9 benchmarks pass with
    full statistical output (min, max, mean, stddev, median, IQR, outliers, OPS)
- `uv run pytest tests/` — all 49 Python tests pass (benchmarks don't interfere)
- `cargo test -p iscc-lib` — all 143 Rust tests pass
- Ruff lint + format checks pass on the benchmark file

**Next:** Phase 0 is now complete (both Rust criterion and Python reference benchmarks). Next steps
could be: (1) Add Python bindings benchmarks (Phase 1) to measure iscc-lib PyO3 bindings vs
iscc-core, (2) Create the benchmark report generation script, or (3) Start Node.js bindings
(`@iscc/lib` via napi-rs) as the next major target deliverable.

**Notes:** Streaming functions (`gen_data_code_v0`, `gen_instance_code_v0`) require `io.BytesIO`
wrappers in Python (iscc-core expects file-like `Stream` objects). These benchmarks use lambda
wrappers to create fresh `BytesIO` instances each iteration, which includes the trivial BytesIO
construction overhead in the measurement — acceptable for baseline comparison. Python reference
baseline numbers (containerized): Meta ~791μs, Text ~2.8ms, Image ~4.2ms, Audio ~1.1ms, Video
~128μs, Mixed ~69μs, Data ~6.5ms (64KB), Instance ~71μs (64KB), ISCC-CODE ~87μs.
