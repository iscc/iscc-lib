# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion — 19 of 22 symbols implemented

All 9 `gen_*_v0` functions are implemented and conformance-tested across 5 targets (Rust, Python,
Node.js, WASM, C FFI). Text utilities (4), algorithm primitives (4), `soft_hash_video_v0`, and
`encode_base64` are promoted to Tier 1 `pub` API. 197 Rust tests and 63 Python tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils + 4 algo primitives +
    `soft_hash_video_v0` + `encode_base64` (Tier 1 `pub`), 197 tests (147 unit + 28 algo/video + 22
    text integration), clippy clean
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

- **Tier 1 API (3 symbols)**: `iscc_decompose` (codec utility), `DataHasher`/`InstanceHasher`
    (streaming types), `conformance_selftest` (diagnostic)
- **Binding wrappers**: all binding crates need wrappers for promoted text utils + algo primitives +
    remaining Tier 1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22306943494) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **197 passed** (147 unit + 28 algo/video + 22 text integration), 0
    failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Implement `iscc_decompose` — a pure codec function that splits a composite ISCC code into its
constituent unit codes. It depends only on existing `codec` module internals (`decode_header`,
`decode_base32`). After that: `DataHasher`/`InstanceHasher` streaming types and
`conformance_selftest` complete the 22-symbol Tier 1 API.
