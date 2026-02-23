# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion — 20 of 22 symbols implemented

All 9 `gen_*_v0` functions are implemented and conformance-tested across 5 targets (Rust, Python,
Node.js, WASM, C FFI). Text utilities (4), algorithm primitives (4), `soft_hash_video_v0`,
`encode_base64`, and `iscc_decompose` are promoted to Tier 1 `pub` API. 214 Rust tests and 63 Python
tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils + 4 algo primitives +
    `soft_hash_video_v0` + `encode_base64` + `iscc_decompose` (Tier 1 `pub`), codec module with
    `decode_units`, 214 tests (164 unit + 28 algo/video + 22 text integration), clippy clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions returning hybrid `IsccResult(dict)` objects,
    BinaryIO streaming, 63 tests (46 conformance + 17 smoke/streaming), `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Tier 1 API (2 symbols)**: `DataHasher`/`InstanceHasher` (streaming types with
    `new() → update(&[u8]) → finalize()` pattern)
- **Tier 1 API (1 symbol)**: `conformance_selftest` (diagnostic function)
- **Binding wrappers**: all binding crates need wrappers for promoted text utils + algo primitives +
    `iscc_decompose` + remaining Tier 1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22307693696) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **214 passed** (164 unit + 28 algo/video + 22 text integration), 0
    failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Implement `DataHasher` and `InstanceHasher` streaming types — the two remaining functional Tier 1
symbols. These wrap `gen_data_code_v0` and `gen_instance_code_v0` in the streaming
`new() → update(&[u8]) → finalize()` pattern central to the library's design. After that:
`conformance_selftest` completes the 22-symbol Tier 1 API.
