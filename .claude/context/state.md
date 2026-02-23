# Project State

## Status: IN_PROGRESS

## Phase: Rust core Tier 1 complete — binding wrappers and structured returns next

All 23 Tier 1 API symbols are implemented in the Rust core crate, including `conformance_selftest`.
All 9 `gen_*_v0` functions pass conformance vectors across 5 targets (Rust, Python, Node.js, WASM, C
FFI). 230 Rust tests and 63 Python tests pass. CI green. Focus shifts to binding completeness,
structured returns, and documentation branding.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols (9 gen functions, 4 text utils, 4 algo
    primitives, `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`, `DataHasher`,
    `InstanceHasher`, `conformance_selftest`), codec module (Tier 2), 230 tests (180 unit + 28
    algo/video + 22 text), clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 9 gen functions returning hybrid `IsccResult(dict)`,
    BinaryIO streaming, 63 tests (46 conformance + 17 smoke/streaming), `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 gen functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 gen functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Binding wrappers**: all binding crates need wrappers for promoted Tier 1 symbols beyond the 9
    gen functions (text utils, algo primitives, streaming types, `iscc_decompose`, `encode_base64`,
    `conformance_selftest`)
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not dicts/objects
    matching iscc-core)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing setup for crates.io and PyPI not yet configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22309136796) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **230 passed** (180 unit + 28 algo/video + 22 text integration)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Add binding wrappers for `conformance_selftest` and the other promoted Tier 1 symbols (text utils,
algo primitives, streaming types, `iscc_decompose`, `encode_base64`) to Python, Node.js, WASM, and C
FFI crates. Then address structured return objects for Node.js/WASM/C FFI bindings.
