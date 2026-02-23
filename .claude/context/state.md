# Project State

## Status: IN_PROGRESS

## Phase: Python bindings complete (23/23) — other bindings gen-only, docs unbranded

All 23 Tier 1 API symbols are implemented in the Rust core crate. All 9 `gen_*_v0` functions pass
conformance vectors across 5 targets (Rust, Python, Node.js, WASM, C FFI). Python bindings are
feature-complete at 23/23 Tier 1 symbols with 147 tests. Node.js, WASM, and C FFI still expose only
the 9 gen functions. 250 Rust tests pass. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols (9 gen functions, 4 text utils, 4 algo
    primitives, `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`, `DataHasher`,
    `InstanceHasher`, `conformance_selftest`), codec module (Tier 2), 250 tests, clippy clean
    workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 gen functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 gen functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green
- **CID workflow**: iteration log tracked in git, 20-min interruptible pause, hill-climbing
    improvements (anti-criteria, boolean-testable verification, structured review grid)

## What's Missing

- **Node.js/WASM/C FFI binding wrappers**: all non-gen Tier 1 symbols (text utils, algo primitives,
    streaming types, `iscc_decompose`, `encode_base64`, `conformance_selftest`)
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not dicts/objects
    matching iscc-core)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing setup for crates.io and PyPI not yet configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22311496247) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test --workspace`: **250 passed**
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `pytest tests/`: **147 passed**
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Expand Node.js bindings beyond the 9 gen functions — add text utils, algo primitives, streaming
hashers, and remaining Tier 1 symbols. The Python bindings provide the pattern to follow.
