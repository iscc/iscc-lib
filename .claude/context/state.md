# Project State

## Status: IN_PROGRESS

## Phase: Binding parity — all non-Python bindings at 21/23, streaming hashers next

All 23 Tier 1 API symbols are implemented in the Rust core crate with 280 tests. Python bindings are
feature-complete (23/23). Node.js, WASM, and C FFI all reached 21/23 parity (C FFI caught up with 4
algorithm primitives in latest iteration). Remaining gap: `DataHasher`/`InstanceHasher` streaming
classes across all three non-Python bindings. CI green, all tests passing.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 280 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 21/23 Tier 1 symbols (9 gen + 4 text + 4 algo
    primitives + `encode_base64` + `iscc_decompose` + `sliding_window` + `conformance_selftest`), 89
    tests (21 suites, 0 failures)
- **WASM bindings (`crates/iscc-wasm/`)**: 21/23 Tier 1 symbols (same 21 as Node.js), 40 tests (9
    conformance + 31 unit)
- **C FFI (`crates/iscc-ffi/`)**: 21/23 Tier 1 symbols + infrastructure (cbindgen, C test program,
    `IsccByteBuffer`/`IsccByteBufferArray` types), 50 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **2 Tier 1 symbols in Node.js**: `DataHasher`, `InstanceHasher` (streaming classes)
- **2 Tier 1 symbols in WASM**: `DataHasher`, `InstanceHasher` (streaming classes)
- **2 Tier 1 symbols in C FFI**: `DataHasher`, `InstanceHasher` (streaming classes)
- **Binding structured returns**: Node.js, WASM, C FFI return plain `.iscc` strings (not full result
    dicts/objects like Python)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22320215892) —
    conclusion: success

## Verification

- `cargo test --workspace`: **280 passed** (50 FFI + 180 core + 28 integration + 22 text utils)
- `wasm-pack test --node`: **40 passed** (9 conformance + 31 unit)
- `pytest tests/`: **147 passed** in 0.74s
- `node --test`: **89 passed** (21 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)

## Next Milestone

Add `DataHasher`/`InstanceHasher` streaming classes to all three non-Python bindings (Node.js, WASM,
C FFI) to reach 23/23 Tier 1 parity. Node.js is the easiest starting point (napi-rs has `#[napi]`
class support with methods).
