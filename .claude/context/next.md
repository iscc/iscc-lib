# Next Work Package

## Step: Add comparative pytest-benchmark for iscc_lib Python bindings

## Goal

Create `benchmarks/python/bench_iscc_lib.py` — a pytest-benchmark suite for the Rust-backed
`iscc_lib` Python package that mirrors the existing `bench_iscc_core.py`. This enables direct
speedup factor comparison between the Rust bindings and the Python reference, completing the
target's benchmark verification criterion: "pytest-benchmark compares Python bindings vs iscc-core".

## Scope

- **Create**: `benchmarks/python/bench_iscc_lib.py`
- **Modify**: none
- **Reference**:
    - `benchmarks/python/bench_iscc_core.py` — mirror this structure exactly
    - `crates/iscc-py/src/lib.rs` — Python binding API signatures
    - `crates/iscc-py/python/iscc_lib/__init__.py` — import path
    - `notes/09-performance-benchmarks.md` — benchmark conventions

## Implementation Notes

Create `bench_iscc_lib.py` that imports all 9 `gen_*_v0` functions from `iscc_lib` and benchmarks
them with **identical inputs** to `bench_iscc_core.py`. This is critical: same inputs → same
benchmark groups → pytest-benchmark can compute speedup factors when both files run together.

Key API differences from `iscc-core` (the `iscc_lib` bindings take different types):

- **`gen_data_code_v0`** and **`gen_instance_code_v0`**: `iscc_lib` takes `bytes` directly (not
    `io.BytesIO`). No lambda wrapper needed — just pass `DATA_64K` bytes directly
- **`gen_image_code_v0`**: `iscc_lib` takes `bytes` (not `list[int]`). Convert `PIXELS_1024` to
    `bytes(PIXELS_1024)`
- **`gen_audio_code_v0`**: `iscc_lib` takes `list[int]` — same as iscc-core
- **`gen_video_code_v0`**: `iscc_lib` takes `list[list[int]]` — same structure as iscc-core's
    `list[tuple[int, ...]]`; pass as `list[list[int]]` (convert tuples to lists)
- **`gen_mixed_code_v0`**: `iscc_lib` takes `list[str]` — same as iscc-core
- **`gen_iscc_code_v0`**: `iscc_lib` takes `(codes: list[str], wide: bool)` — same as iscc-core

Use the same `@pytest.mark.benchmark(group="gen_<name>_v0")` group names so that `pytest-benchmark`
groups the iscc-core and iscc-lib results together for automatic comparison.

Use the same pre-computed inputs. Copy them (since the file is small and constants are trivial) —
this matches KISS and avoids shared fixture complexity.

Verify that running `pytest benchmarks/python/ --benchmark-only` executes both files and shows
grouped results.

## Verification

- `benchmarks/python/bench_iscc_lib.py` exists with 9 benchmark functions
- `pytest benchmarks/python/bench_iscc_lib.py --benchmark-only` runs without errors
- All 9 benchmarks produce valid ISCC results (no exceptions)
- Benchmark group names match `bench_iscc_core.py` exactly (enabling comparison)
- `pytest benchmarks/python/ --benchmark-only` shows both iscc-core and iscc-lib results grouped
    together with speedup comparison

## Done When

The advance agent is done when `pytest benchmarks/python/ --benchmark-only` runs both
`bench_iscc_core.py` and `bench_iscc_lib.py` together, all 18 benchmarks pass, and results are
grouped by function name showing iscc-lib vs iscc-core comparison.
