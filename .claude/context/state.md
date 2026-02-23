# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API — 22 of 23 listed symbols implemented, `conformance_selftest` remaining

All 9 `gen_*_v0` functions are implemented and conformance-tested across 5 targets (Rust, Python,
Node.js, WASM, C FFI). All utility, algorithm, codec, and streaming Tier 1 symbols are promoted to
`pub` API. Only `conformance_selftest` (diagnostic) remains. 229 Rust tests and 63 Python tests
pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils + 4 algo primitives +
    `soft_hash_video_v0` + `encode_base64` + `iscc_decompose` + `DataHasher`/`InstanceHasher` (all
    Tier 1 `pub`), codec module (Tier 2), 229 tests (179 unit + 28 algo/video + 22 text), clippy
    clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 9 gen functions returning hybrid `IsccResult(dict)`,
    BinaryIO streaming, 63 tests (46 conformance + 17 smoke/streaming), `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Tier 1 API (1 symbol)**: `conformance_selftest` diagnostic function
- **Binding wrappers**: all binding crates need wrappers for promoted Tier 1 symbols beyond the 9
    gen functions (text utils, algo primitives, streaming types, `iscc_decompose`, `encode_base64`)
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22308399136) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **229 passed** (179 unit + 28 algo/video + 22 text integration)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Implement `conformance_selftest` — the last remaining Tier 1 symbol. This diagnostic function runs
all conformance vectors from `data.json` and returns pass/fail results. After that, the Rust core
Tier 1 API is complete and focus shifts to binding wrappers for the newly promoted Tier 1 symbols.
