# Project State

## Status: IN_PROGRESS

## Phase: Python complete (23/23), Node.js & WASM at 17/23, C FFI at 11/23

All 23 Tier 1 API symbols are implemented in the Rust core crate with 250 tests. Python bindings are
feature-complete. Node.js and WASM bindings are both at 17/23 Tier 1 symbols after the latest WASM
expansion. C FFI still has only the 9 gen functions plus 2 helpers (11 symbols). CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 250 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 17/23 Tier 1 symbols (9 gen + 8 utils/helpers), 73
    tests (46 conformance + 27 unit)
- **WASM bindings (`crates/iscc-wasm/`)**: 17/23 Tier 1 symbols (9 gen + 8 utils/helpers), 29 tests
    (9 conformance + 20 unit)
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols (9 gen + 2 helpers), cbindgen, C test
    program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **C FFI expansion (12 symbols)**: text utils, algo primitives, encode_base64, iscc_decompose,
    conformance_selftest, sliding_window, DataHasher, InstanceHasher
- **Node.js remaining (6 symbols)**: `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`
- **WASM remaining (6 symbols)**: same 4 algo primitives + 2 streaming classes as Node.js
- **Binding structured returns**: Node.js, WASM, C FFI return plain strings (not dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22315569921) —
    conclusion: success

## Verification

- `cargo test --workspace`: **250 passed** (20 FFI + 180 core + 28 integration + 22 text utils)
- `wasm-pack test --node`: **29 passed** (9 conformance + 20 unit)
- `pytest tests/`: **147 passed** in 1.04s
- `node --test`: **73 passed** (17 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)

## Next Milestone

Expand C FFI bindings with the 8 simple functions (text utils, encode_base64, iscc_decompose,
conformance_selftest, sliding_window) to bring it to parity with Node.js and WASM at 17+ symbols.
This is the lowest-risk next step per the review handoff, continuing the pattern of expanding all
binding crates to the same level before tackling complex types (algo primitives + streaming
hashers).
