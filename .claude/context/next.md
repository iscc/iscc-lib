# Next Work Package

## Step: Add pytest-benchmark for iscc-core Python reference baseline

## Goal

Establish the Python reference performance baseline by benchmarking all 9 `gen_*_v0` functions from
`iscc-core` using pytest-benchmark. This completes Phase 0 of the benchmark plan and provides the
denominator needed to compute speedup factors (Rust vs Python).

## Scope

- **Modify**: `pyproject.toml` — add `pytest-benchmark` and `iscc-core` to dev dependencies
- **Create**: `benchmarks/python/bench_iscc_core.py` — pytest-benchmark tests for all 9 gen
    functions using matching inputs to the Rust criterion benchmarks
- **Reference**: `crates/iscc-lib/benches/benchmarks.rs` (must use same inputs),
    `notes/09-performance-benchmarks.md` (benchmark plan), `pyproject.toml` (current deps)

## Implementation Notes

1. **Add dependencies** to `pyproject.toml` `[dependency-groups] dev`:

    - `pytest-benchmark` — the pytest plugin for statistical benchmarks
    - `iscc-core` — the Python reference implementation being benchmarked

2. **Create `benchmarks/python/bench_iscc_core.py`** with pytest-benchmark tests for all 9
    functions. Each test function receives the `benchmark` fixture and calls
    `benchmark(gen_func, *args)`.

3. **Use identical inputs** to the Rust criterion benchmarks in `benchmarks.rs` for fair comparison.
    The exact inputs:

    - `gen_meta_code_v0`: name="Die Unendliche Geschichte", description="Von Michael Ende", bits=64
    - `gen_text_code_v0`: ~1000-char text from repeating "The quick brown fox jumps over the lazy
        dog. " (same synthetic_text logic as Rust)
    - `gen_image_code_v0`: `list(range(1024))` mapped to `i % 256` — 1024-element pixel list
    - `gen_audio_code_v0`: `list(range(300))` — 300-element i32 vector
    - `gen_video_code_v0`: 10 frames of 380 sequential ints each (matching Rust: frame f has values
        `f*380..f*380+379`)
    - `gen_mixed_code_v0`: `["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"]`, bits=64
    - `gen_data_code_v0`: 64KB deterministic bytes `bytes(i % 256 for i in range(65536))`, bits=64
    - `gen_instance_code_v0`: same 64KB deterministic bytes, bits=64
    - `gen_iscc_code_v0`: 4 unit strings
        `["AAAYPXW445FTYNJ3", "EAARMJLTQCUWAND2", "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG", "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ"]`,
        wide=False

4. **Import from iscc_core**: `from iscc_core.code_meta import gen_meta_code_v0`, etc. Each gen
    function lives in its own module in iscc-core (`code_meta`, `code_content_text`,
    `code_content_image`, `code_content_audio`, `code_content_video`, `code_content_mixed`,
    `code_data`, `code_instance`, `code_iscc`). Alternatively, import from `iscc_core` top-level if
    it re-exports them.

5. **Naming convention**: Name benchmark functions `test_bench_gen_meta_code_v0`, etc. Group with
    `@pytest.mark.benchmark(group="gen_meta_code_v0")` for organized output.

6. **Run command**: `pytest benchmarks/python/ --benchmark-only` runs only benchmarks (skips regular
    tests). Alternatively `pytest benchmarks/python/` also works since all functions use the
    benchmark fixture.

7. **Configure pytest** for the benchmark directory — add `benchmarks/python` to be discoverable.
    The root `pyproject.toml` `testpaths` currently only includes `["tests"]`. Either:

    - Run benchmarks explicitly: `pytest benchmarks/python/` (don't change testpaths)
    - Or add a separate pytest config section for benchmarks Prefer the explicit invocation approach
        — don't add benchmarks to the default test suite.

8. **Do NOT modify** `[tool.pytest.ini_options] testpaths` — benchmarks should not run as part of
    `pytest` (the normal test command). They are run separately via `pytest benchmarks/python/`.

## Verification

- `uv sync --group dev` installs successfully (including iscc-core and pytest-benchmark)
- `uv run pytest benchmarks/python/bench_iscc_core.py --benchmark-only` completes and shows timing
    results for all 9 gen functions
- Each benchmark runs at least one iteration without errors
- Benchmark output includes statistical summary (min, max, mean, stddev)
- `uv run pytest tests/` still passes (49 tests — benchmarks don't interfere with regular tests)
- `cargo test -p iscc-lib` still passes (143 tests)

## Done When

`pytest benchmarks/python/bench_iscc_core.py --benchmark-only` completes without errors, reporting
timing results for all 9 `gen_*_v0` iscc-core functions, and existing tests remain unaffected.
