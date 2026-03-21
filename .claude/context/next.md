# Next Work Package

## Step: Add pytest-benchmark comparing iscc-lib vs iscc-core

## Goal

Create a pytest-benchmark test file that measures performance of all 9 `gen_*_v0` functions in both
`iscc_lib` (Rust bindings) and `iscc_core` (Python reference), producing speedup comparisons. This
fulfills the target's "pytest-benchmark compares Python bindings vs `iscc-core`" verification
criterion.

## Scope

- **Create**: `tests/test_benchmarks.py`
- **Modify**: (none)
- **Reference**: `tests/test_conformance.py` (input preparation patterns, vector loading),
    `crates/iscc-lib/tests/data.json` (conformance vectors), `pyproject.toml` (pytest-benchmark
    already in dev deps)

## Not In Scope

- Publishing speedup factors in documentation (separate future step)
- Adding a benchmark CI job or benchmark tracking in CI
- Modifying existing test files or pyproject.toml
- Criterion (Rust) benchmark changes — those already exist and pass
- Benchmarking `gen_sum_code_v0` (no conformance vectors, no iscc-core equivalent)

## Implementation Notes

**Pattern**: For each of the 9 `gen_*_v0` functions, create two benchmark functions — one calling
`iscc_lib.<fn>` and one calling `iscc_core.<fn>` — with the same input data derived from conformance
vectors. Use pytest-benchmark's `benchmark` fixture.

**Key API differences between iscc-lib and iscc-core**:

- `gen_data_code_v0` / `gen_instance_code_v0`: iscc-core requires a stream object (`io.BytesIO`
    wrapper), iscc-lib accepts raw `bytes` directly
- `gen_meta_code_v0`: both have the same signature (`name, description=, meta=, bits=`)
- `gen_iscc_code_v0`: both accept `(codes, wide=False)` — iscc-core parameter is also named `wide`
- Return types differ: iscc-core returns `dict`, iscc-lib returns typed result objects. Benchmarks
    only measure timing, so return type doesn't matter for the benchmark itself

**Input data**: Use representative inputs from conformance vectors (pick one representative vector
per function). Reuse the `load_vectors`, `_decode_stream`, and `_prepare_meta_arg` patterns from
`tests/test_conformance.py`. For data/instance, wrap bytes in `io.BytesIO` for iscc-core calls.

**Grouping**: Use `@pytest.mark.benchmark(group="<function_name>")` decorator so pytest-benchmark
shows side-by-side comparisons within each function.

**Naming convention**: `test_bench_<function>_iscc_lib` and `test_bench_<function>_iscc_core` for
each function pair.

**Running benchmarks**:

- `uv run pytest tests/test_benchmarks.py --benchmark-only` — runs only benchmarks
- `uv run pytest tests/ --benchmark-disable` — normal test runs skip benchmarks

**Correctness assertion**: Each benchmark pair should include a post-benchmark assertion that both
implementations produce the same ISCC output for the same input. This catches drift between
implementations. Access via `result = benchmark(fn, *args)` and compare `result["iscc"]` (iscc-core
dict) vs `result.iscc` (iscc-lib typed result).

**iscc-core module imports**: Functions are available at top-level:

```python
import iscc_core as ic

ic.gen_meta_code_v0(...)
```

**iscc-core version note**: The installed version is 1.2.2 (not the latest 1.3.0 in data.json). This
shouldn't matter for benchmarking since we use the same inputs and both produce valid ISCC codes —
but the advance agent should verify output matching works for the chosen inputs.

## Verification

- `uv run pytest tests/test_benchmarks.py --benchmark-only -q` exits 0 and produces benchmark output
    for all 9 function pairs (18 benchmarks total)
- `uv run pytest tests/test_benchmarks.py --benchmark-only -q 2>&1 | grep -c "bench_"` shows 18
    benchmark entries
- `uv run pytest tests/ --benchmark-disable -q` passes (all existing tests still pass with
    benchmarks disabled)
- `uv run ruff check tests/test_benchmarks.py` clean
- `uv run ruff format --check tests/test_benchmarks.py` clean

## Done When

All verification criteria pass — 18 benchmark functions (9 functions x 2 implementations) run
successfully via pytest-benchmark, producing timing comparisons between iscc-lib and iscc-core.
