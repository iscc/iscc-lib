## 2026-02-23 — Add comparative pytest-benchmark for iscc_lib Python bindings

**Done:** Created `benchmarks/python/bench_iscc_lib.py` with 9 benchmark functions mirroring
`bench_iscc_core.py`, using identical inputs and benchmark group names. Added
`benchmarks/python/conftest.py` to configure pytest to discover `bench_*.py` files when running
`pytest benchmarks/python/`.

**Files changed:**

- `benchmarks/python/bench_iscc_lib.py`: 9 benchmark functions for all `gen_*_v0` functions from
    `iscc_lib`, with API-appropriate input types (bytes instead of io.BytesIO for data/instance,
    bytes instead of list[int] for image, list\[list[int]\] instead of list[tuple] for video)
- `benchmarks/python/conftest.py`: `pytest_configure` hook that adds `bench_*.py` to `python_files`
    collection pattern, enabling `pytest benchmarks/python/` directory discovery

**Verification:** All 18 benchmarks pass (9 iscc-core + 9 iscc-lib). Results are grouped by function
name showing direct speedup comparison. Notable speedups observed: gen_data_code_v0 ~5.5x,
gen_instance_code_v0 ~2.6-3.2x, gen_video_code_v0 ~3.4-3.8x, gen_mixed_code_v0 ~2.8x,
gen_meta_code_v0 ~2.6x, gen_audio_code_v0 ~2.5x, gen_image_code_v0 ~3.1x, gen_iscc_code_v0 ~1.7x,
gen_text_code_v0 ~1.1x. All existing tests (49 Python conformance + Rust tests) continue to pass.
Clippy and fmt checks pass.

**Next:** Update `state.md` to reflect the completed Python bindings benchmark. The remaining target
items are: documentation site (lib.iscc.codes) and CI/CD publishing pipelines. The text_code
benchmark shows minimal speedup (~1.1x) which suggests the Python overhead is small relative to
compute time for text processing — this is expected since text normalization dominates. The
streaming functions (data/instance) show significant but lower speedup than native Rust criterion
benchmarks because the PyO3 binding receives the full `bytes` at once rather than streaming from
io.BytesIO, so the comparison measures different I/O patterns.

**Notes:** The `conftest.py` uses `pytest_configure` hook to dynamically add `bench_*.py` to
`python_files` — this is needed because the project's `pyproject.toml` doesn't set `python_files`
(using pytest default of `test_*.py`), and the benchmark files use `bench_` prefix per the
established convention. The conftest only affects collection when pytest runs from the
`benchmarks/python/` directory.
