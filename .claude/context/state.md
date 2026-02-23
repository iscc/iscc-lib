# Project State

## Status: IN_PROGRESS

## Phase: Binding parity — WASM and Node.js at 21/23, C FFI at 17/23, Python complete

All 23 Tier 1 API symbols are implemented in the Rust core crate with 268 tests. Python bindings are
feature-complete (23/23). Node.js and WASM both advanced to 21/23 (4 algorithm primitives added in
latest iterations). C FFI remains at 17/23 Tier 1 symbols. CI green, all tests passing.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 268 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 21/23 Tier 1 symbols (9 gen + 4 text + 4 algo
    primitives + `encode_base64` + `iscc_decompose` + `sliding_window` + `conformance_selftest`), 89
    tests (21 suites, 0 failures)
- **WASM bindings (`crates/iscc-wasm/`)**: 21/23 Tier 1 symbols (same 21 as Node.js), 40 tests (9
    conformance + 31 unit)
- **C FFI (`crates/iscc-ffi/`)**: 20 exported symbols (17 Tier 1 + 3 infrastructure), cbindgen, C
    test program, 38 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **2 Tier 1 symbols in Node.js**: `DataHasher`, `InstanceHasher` (streaming classes)
- **2 Tier 1 symbols in WASM**: `DataHasher`, `InstanceHasher` (streaming classes)
- **6 Tier 1 symbols in C FFI**: 4 algorithm primitives + `DataHasher`, `InstanceHasher`
- **Binding structured returns**: Node.js, WASM, C FFI return plain `.iscc` strings (not full result
    dicts/objects like Python)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22319105493) —
    conclusion: success

## Verification

- `cargo test --workspace`: **268 passed** (38 FFI + 180 core + 28 integration + 22 text utils)
- `wasm-pack test --node`: **40 passed** (9 conformance + 31 unit)
- `pytest tests/`: **147 passed** in 0.77s
- `node --test`: **89 passed** (21 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)

## Next Milestone

Add the 4 algorithm primitives to C FFI bindings (bringing it from 17/23 to 21/23, matching Node.js
and WASM). Then add `DataHasher`/`InstanceHasher` streaming classes across all three non-Python
bindings to reach 23/23 parity.
