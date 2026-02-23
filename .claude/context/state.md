# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion & binding enrichment

All 9 `gen_*_v0` functions work in Rust core with structured result types. Python bindings return
hybrid `IsccResult(dict)` objects. BinaryIO streaming support was added for `gen_data_code_v0` and
`gen_instance_code_v0` but has a `ty` type checker error blocking pre-push hooks. Node.js, WASM, and
C FFI bindings still return plain strings. 13 additional Tier 1 API symbols remain unimplemented.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions with `*CodeResult` structs, 143 tests, clippy
    clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions returning hybrid `IsccResult(dict)` objects,
    BinaryIO streaming added (but `ty` fails), 63 tests pass (46 conformance + 17 smoke/streaming)
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns only), conformance tests in
    CI
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups measured
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **`ty` type checker fix**: `hasattr()` pattern in `__init__.py` causes 2 `call-non-callable`
    errors — pre-push hook fails. Must fix before any other Python work
- **Tier 1 API (13 symbols)**: text utils (4), algorithm primitives (4), `soft_hash_video_v0`,
    `encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher` streaming types,
    `conformance_selftest`
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Binding Tier 1 expansion**: all binding crates need the 13 new Tier 1 symbols
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22302746283) —
    conclusion: success
- All 3 workflows (ci, docs, release) green
- Note: `ty` pre-push hook fails locally (2 errors) — not caught by CI

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- `ty check __init__.py`: **2 errors** (`call-non-callable` on `data.read()`)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Fix the `ty` type checker error in Python `__init__.py` (replace `hasattr(data, "read")` with
`isinstance(data, bytes)` inversion). This is the sole blocking issue from the last review cycle and
must be resolved before any further work.
