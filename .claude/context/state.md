# Project State

## Status: IN_PROGRESS

## Phase: All non-Python bindings at 17/23 Tier 1 symbols, Python complete (23/23)

All 23 Tier 1 API symbols are implemented in the Rust core crate with 268 tests. Python bindings are
feature-complete (23/23). Node.js, WASM, and C FFI are now all at parity: 17/23 Tier 1 symbols each.
The same 6 complex symbols remain across all three non-Python bindings. CI green.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 23 Tier 1 symbols, codec module (Tier 2), 268 tests,
    clippy clean workspace-wide
- **Python bindings (`crates/iscc-py/`)**: 23/23 Tier 1 symbols with `IsccResult(dict)` returns,
    `DataHasher`/`InstanceHasher` streaming classes, BinaryIO support, 147 tests, `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 17/23 Tier 1 symbols (9 gen + 8 utils/helpers), 73
    tests (46 conformance + 27 unit)
- **WASM bindings (`crates/iscc-wasm/`)**: 17/23 Tier 1 symbols (9 gen + 8 utils/helpers), 29 tests
    (9 conformance + 20 unit)
- **C FFI (`crates/iscc-ffi/`)**: 20 exported symbols (17 Tier 1 + 3 infrastructure), cbindgen, C
    test program, 38 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **6 complex Tier 1 symbols in Node.js, WASM, C FFI**: `alg_simhash`, `alg_minhash_256`,
    `alg_cdc_chunks`, `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`
- **Binding structured returns**: Node.js, WASM, C FFI return plain `.iscc` strings (not full result
    dicts/objects like Python)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
- **100% test coverage**: not yet measured or enforced
- **Publishing**: OIDC trusted publishing for crates.io and PyPI not configured

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22316954830) —
    conclusion: success

## Verification

- `cargo test --workspace`: **268 passed** (38 FFI + 180 core + 28 integration + 22 text utils)
- `wasm-pack test --node`: **29 passed** (9 conformance + 20 unit)
- `pytest tests/`: **147 passed** in 1.06s
- `node --test`: **73 passed** (17 suites, 0 failures)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (CI-verified)

## Next Milestone

Add the 4 algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
`soft_hash_video_v0`) to one binding crate (Node.js or WASM first, then replicate). These have
complex type signatures (slice inputs/outputs) requiring careful FFI design per binding. After
algorithm primitives, add `DataHasher`/`InstanceHasher` streaming classes to reach 23/23 across all
bindings.
