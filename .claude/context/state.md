# Project State

## Status: IN_PROGRESS

## Phase: Python bindings complete (23/23) — Node.js expanded to 17/23, WASM/C FFI gen-only

All 23 Tier 1 API symbols are implemented in the Rust core crate with 250 tests. Python bindings are
feature-complete at 23/23 Tier 1 symbols. Node.js bindings expanded from 9 to 17 Tier 1 symbols
(added text utils, encode_base64, iscc_decompose, conformance_selftest, sliding_window). WASM and C
FFI still expose only the 9 gen functions. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 250 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 17/23 Tier 1 symbols (9 gen + 8 utils/helpers), 73
    tests (46 conformance + 27 new)
- **WASM bindings (`crates/iscc-wasm/`)**: 9 gen functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Node.js remaining (6 symbols)**: `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`
- **WASM binding expansion**: all 14 non-gen Tier 1 symbols
- **C FFI binding expansion**: text utils, algo primitives, streaming types, remaining Tier 1
    symbols
- **Binding structured returns**: Node.js, WASM, C FFI return plain strings (not dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22314468014) —
    conclusion: success

## Verification

- `cargo test --workspace`: **250 passed** (20 + 180 + 28 + 22)
- `pytest tests/`: **147 passed** in 1.19s
- `node --test`: **73 passed** (17 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)
- WASM: conformance tests pass (CI-verified)

## Next Milestone

Expand WASM bindings with the 8 simple functions (text utils, encode_base64, iscc_decompose,
conformance_selftest, sliding_window) — lowest-risk step per review handoff, reuses documented
wasm-bindgen patterns. Alternatively, complete Node.js with the remaining 6 symbols (algo primitives

- streaming hashers).
