# Project State

## Status: IN_PROGRESS

## Phase: Rust core Tier 1 complete — Python bindings 17/23, other bindings gen-only

All 23 Tier 1 API symbols are implemented in the Rust core crate. All 9 `gen_*_v0` functions pass
conformance vectors across 5 targets (Rust, Python, Node.js, WASM, C FFI). Python bindings now
expose 17 of 23 Tier 1 symbols (added `encode_base64`, `iscc_decompose`, `sliding_window` since last
state). 250 Rust tests and 98 Python tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols (9 gen functions, 4 text utils, 4 algo
    primitives, `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`, `DataHasher`,
    `InstanceHasher`, `conformance_selftest`), codec module (Tier 2), 250 tests (20 conformance +
    180 unit + 28 algo/video + 22 text), clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 17 functions (9 gen with `IsccResult(dict)` returns,
    `conformance_selftest`, 4 text utils, `encode_base64`, `iscc_decompose`, `sliding_window`),
    BinaryIO streaming, 98 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 gen functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 gen functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Python bindings (6 remaining)**: `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`
- **Node.js/WASM/C FFI binding wrappers**: all non-gen Tier 1 symbols (text utils, algo primitives,
    streaming types, `iscc_decompose`, `encode_base64`, `conformance_selftest`)
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not dicts/objects
    matching iscc-core)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing setup for crates.io and PyPI not yet configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22310671794) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test --workspace`: **250 passed** (20 conformance + 180 unit + 28 algo/video + 22 text)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `pytest tests/`: **98 passed**
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Continue expanding Python binding API surface toward 23/23 Tier 1 symbols. Next batch: algo
primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`) — straightforward `Vec<u8>` /
`Vec<Vec<u8>>` returns. Then `soft_hash_video_v0` and streaming types (`DataHasher`,
`InstanceHasher`) to complete Python bindings.
