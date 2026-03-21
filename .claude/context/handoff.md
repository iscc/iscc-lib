## 2026-03-21 — Add pytest-benchmark comparing iscc-lib vs iscc-core

**Done:** Created `tests/test_benchmarks.py` with 18 benchmark functions (9 functions x 2
implementations) comparing iscc-lib (Rust bindings) against iscc-core (Python reference) using
pytest-benchmark. Each benchmark pair uses the same conformance vector input and asserts that both
implementations produce matching ISCC codes.

**Files changed:**

- `tests/test_benchmarks.py`: New file with 18 benchmarks covering all 9 `gen_*_v0` functions. Uses
    representative conformance vectors, grouped by function name for side-by-side comparison.

**Verification:**

- `uv run pytest tests/test_benchmarks.py --benchmark-only -q` exits 0, all 18 benchmarks pass
- `grep -c "bench_"` on benchmark output shows 18 entries
- `uv run pytest tests/ --benchmark-disable -q` passes (229 tests including existing tests)
- `uv run ruff check tests/test_benchmarks.py` clean
- `uv run ruff format --check tests/test_benchmarks.py` clean
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `cargo test -p iscc-lib` clean
- `mise run check` fails due to pre-existing trailing whitespace in
    `.claude/context/specs/swift-bindings.md` and mdformat oscillation — not related to this change

**Next:** All target verification criteria for benchmarks are met. Consider publishing speedup
factors in documentation as a future step, or adding a benchmark CI job for tracking regressions.

**Notes:**

- For `gen_data_code_v0` and `gen_instance_code_v0`, iscc-core requires `io.BytesIO` streams which
    get consumed after the first read. The benchmark wrapper creates a fresh `BytesIO` each
    iteration to ensure correct results across all benchmark rounds.
- For `gen_image_code_v0`, iscc-core expects `list[int]` while iscc-lib expects `bytes` — the
    benchmark passes `list(_image_pixels)` to iscc-core and `_image_pixels` (bytes) to iscc-lib.
- Representative speedups observed: meta ~20x, text ~33x, image ~15x, audio ~50x, video ~13x, mixed
    ~30x, data ~11x, instance ~62x, iscc ~20x.
- Pre-existing `mise run check` issue: `.claude/context/specs/swift-bindings.md` has trailing
    whitespace that hooks keep fixing on each run. This is out of scope (context file).
