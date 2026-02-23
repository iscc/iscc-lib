# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion — 17 of 22 symbols promoted

All 9 `gen_*_v0` functions are implemented and conformance-tested across 5 targets (Rust, Python,
Node.js, WASM, C FFI). Text utilities (4) and all 4 algorithm primitives (`sliding_window`,
`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`) are promoted to Tier 1 `pub` API. 188 Rust tests
and 63 Python tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils + 4 algo primitives (Tier 1
    `pub`), 188 tests (143 unit + 23 algo integration + 22 text integration), clippy clean
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

- **Tier 1 API (5 symbols)**: `soft_hash_video_v0` (exists as private fn), `encode_base64` (not
    implemented), `iscc_decompose` (not implemented), `DataHasher`/`InstanceHasher` streaming types
    (not implemented), `conformance_selftest` (not implemented)
- **Binding wrappers**: all binding crates need wrappers for promoted text utils + algo primitives +
    future Tier 1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22305770191) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **188 passed** (143 unit + 23 algo + 22 text integration), 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Promote `soft_hash_video_v0` to Tier 1 public API — it already exists as a private function, so this
follows the proven promotion pattern. Alternatively, implement `encode_base64` or `iscc_decompose`
as new codec utility functions — these are straightforward implementations. After those:
`DataHasher`/`InstanceHasher` streaming types and `conformance_selftest`.
