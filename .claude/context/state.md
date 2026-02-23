# Project State

## Status: IN_PROGRESS

## Phase: Binding parity — Node.js at 23/23, WASM and C FFI at 21/23

All 23 Tier 1 API symbols are implemented in the Rust core crate with 280 tests. Python (23/23) and
Node.js (23/23) bindings are feature-complete including DataHasher/InstanceHasher streaming classes.
WASM and C FFI remain at 21/23, each needing DataHasher and InstanceHasher. CI green, all tests
pass.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 280 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 23/23 Tier 1 symbols including
    `DataHasher`/`InstanceHasher` streaming classes, 57 tests (14 suites, 0 failures)
- **WASM bindings (`crates/iscc-wasm/`)**: 21/23 Tier 1 symbols (missing streaming hashers), 31
    tests
- **C FFI (`crates/iscc-ffi/`)**: 21/23 Tier 1 symbols + infrastructure (cbindgen, C test program,
    `IsccByteBuffer`/`IsccByteBufferArray` types), 50 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **2 Tier 1 symbols in WASM**: `DataHasher`, `InstanceHasher` (streaming classes)
- **2 Tier 1 symbols in C FFI**: `DataHasher`, `InstanceHasher` (streaming classes)
- **Binding structured returns**: Node.js, WASM, C FFI return plain `.iscc` strings (not full result
    dicts/objects like Python)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22320731962) —
    conclusion: success

## Verification

- `cargo test --workspace`: **280 passed** (50 FFI + 180 core + 28 integration + 22 text utils)
- `wasm-pack test --node`: **31 passed**
- `pytest tests/`: **147 passed** in 1.04s
- `node --test`: **57 passed** (14 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)

## Next Milestone

Add `DataHasher`/`InstanceHasher` streaming classes to WASM bindings to reach 23/23 Tier 1 parity.
WASM is the natural next target since `#[wasm_bindgen]` supports class methods similarly to napi-rs.
C FFI streaming hashers (opaque pointer lifecycle pattern) follow after.
