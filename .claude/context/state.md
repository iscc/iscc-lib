# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion — 16 of 22 symbols promoted

All 9 `gen_*_v0` functions are implemented and conformance-tested across 5 targets (Rust, Python,
Node.js, WASM, C FFI). Text utilities (4) and 3 of 4 algorithm primitives (`sliding_window`,
`alg_simhash`, `alg_minhash_256`) are promoted to Tier 1 `pub` API. 182 Rust tests and 63 Python
tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils + 3 algo primitives (Tier 1
    `pub`), 182 tests (143 unit + 17 algo integration + 22 text integration), clippy clean
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

- **Tier 1 API (7 symbols)**: `alg_cdc_chunks` (exists as `pub(crate)`), `soft_hash_video_v0`
    (exists as private fn), `encode_base64` (not implemented), `iscc_decompose` (not implemented),
    `DataHasher`/`InstanceHasher` streaming types (not implemented), `conformance_selftest` (not
    implemented)
- **Binding wrappers**: all binding crates need wrappers for promoted text utils + algo primitives +
    future Tier 1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22305198577) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **182 passed** (143 unit + 17 algo + 22 text integration), 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Promote `alg_cdc_chunks` from the `cdc` module to Tier 1 — last of the 4 algorithm primitives.
Follow the established pattern: make `cdc` module `pub`, promote `alg_cdc_chunks` to `pub fn`, add
`pub use cdc::alg_cdc_chunks` re-export, add integration tests. After that: `soft_hash_video_v0`,
`encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher`, `conformance_selftest`.
