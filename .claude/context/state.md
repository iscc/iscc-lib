# Project State

## Status: IN_PROGRESS

## Phase: Python bindings complete (23/23) — other bindings gen-only, docs unbranded

All 23 Tier 1 API symbols are implemented in the Rust core crate with 250 tests. Python bindings are
feature-complete at 23/23 Tier 1 symbols with 147 tests. Node.js, WASM, and C FFI still expose only
the 9 gen functions. CI green. No changes since last assessment.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 250 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 gen functions (string returns only), 46 tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 gen functions (string returns only), conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Node.js/WASM/C FFI binding expansion**: text utils, algo primitives, streaming types,
    `iscc_decompose`, `encode_base64`, `conformance_selftest` — all non-gen Tier 1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI return plain strings (not dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22312404407) —
    conclusion: success

## Verification

- `cargo test --workspace`: **250 passed** (20 + 180 + 28 + 22)
- `pytest tests/`: **147 passed** in 0.80s
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass (CI-verified)

## Next Milestone

Expand Node.js bindings beyond the 9 gen functions — add text utils, algo primitives, streaming
hashers, and remaining Tier 1 symbols. Python bindings provide the pattern to follow.
