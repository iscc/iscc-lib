# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion & binding enrichment

All 9 `gen_*_v0` functions work in Rust core with structured result types, and Python bindings now
return `dict` objects matching iscc-core. A spec for hybrid `IsccResult(dict)` objects (attribute +
dict access) has been written but not yet implemented. Node.js, WASM, and C FFI bindings still
return plain strings. 13 additional Tier 1 API symbols are not yet implemented.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions with `*CodeResult` structs, 143 tests, clippy
    clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions returning `dict` with all fields, 49 tests
    pass
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns), 46 conformance tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns), conformance tests in CI
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups measured
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green
- **Spec**: hybrid `IsccResult(dict)` objects designed but not implemented

## What's Missing

- **Hybrid IsccResult objects**: spec written, pure Python implementation in `__init__.py` pending
- **Python streaming**: `gen_data_code_v0`/`gen_instance_code_v0` must accept file-like objects
- **Tier 1 API (13 symbols)**: text utils (4), algorithm primitives (4), `soft_hash_video_v0`,
    `encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher` streaming types,
    `conformance_selftest`
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings
- **Binding Tier 1 expansion**: all binding crates need the 13 new Tier 1 symbols
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22302342007) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **49 passed**
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Implement hybrid `IsccResult(dict)` objects in `crates/iscc-py/python/iscc_lib/__init__.py` per the
spec already written. This is a pure Python change — no Rust modifications needed. Then add Python
streaming support for `gen_data_code_v0`/`gen_instance_code_v0` accepting `BinaryIO` file-like
objects, completing the Python drop-in compatibility goal.
