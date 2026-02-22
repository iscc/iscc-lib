## 2026-02-22 — Add Python conformance tests for all 9 gen\_\*\_v0 functions

**Done:** Created `tests/test_conformance.py` with parametrized tests for all 9 `gen_*_v0` functions
against the 46 conformance vectors from `data.json`. Each function's test cases handle the correct
input type conversions (hex-decoded bytes for stream data, `json.dumps` for meta dicts, `bytes()`
for pixel arrays, etc.). All 46 tests pass, validating the full Python → PyO3 → Rust → result
pipeline.

**Files changed:**

- `tests/test_conformance.py`: new file with 9 parametrized test functions covering all 46
    conformance vectors. Uses `load_vectors` helper for DRY data loading, `_prepare_meta_arg` for
    meta type dispatch, and `_decode_stream` for hex stream parsing.

**Verification:**

- `pytest tests/test_conformance.py -v`: 46 passed
- `pytest tests/`: 49 passed (3 smoke + 46 conformance)
- `ruff check tests/test_conformance.py`: clean
- `ruff format --check tests/test_conformance.py`: clean
- `uv run prek run --all-files`: all 14 hooks passed

**Next:** The Python bindings target criteria are now fully met (all 9 functions exposed via PyO3,
all 46 conformance vectors pass from Python). Consider CI/CD setup (GitHub Actions workflows),
benchmarks (criterion for Rust, pytest-benchmark for Python), or starting the next binding crate
(Node.js via napi-rs, WASM via wasm-bindgen, or C FFI via cbindgen).

**Notes:** The `description` parameter for `gen_meta_code_v0` uses `description or None` to map
empty string to `None`, matching the Rust conformance test behavior (the Rust `gen_meta_code_v0`
treats `Some("")` identically to `None` via `unwrap_or("")`, so both approaches produce the same
result). No surprises or shortcuts taken.
