# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion — algorithm primitives next

All 9 `gen_*_v0` functions are implemented and conformance-tested across all 5 targets (Rust,
Python, Node.js, WASM, C FFI). Text utilities (`text_clean`, `text_remove_newlines`, `text_trim`,
`text_collapse`) were promoted to Tier 1 `pub` API with 22 integration tests. Remaining: 9 more Tier
1 symbols, binding wrappers for all promoted symbols, structured returns for non-Python bindings,
and documentation branding.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions + 4 text utils (Tier 1 `pub`), 165 tests (143
    unit + 22 integration), clippy clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions returning hybrid `IsccResult(dict)` objects,
    BinaryIO streaming, 63 tests (46 conformance + 17 smoke/streaming), `ty` clean
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns only), 46 conformance
    tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns only), conformance tests in
    CI
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups measured
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Tier 1 API (9 symbols)**: algorithm primitives (`sliding_window`, `alg_minhash_256`,
    `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`,
    `DataHasher`/`InstanceHasher` streaming types, `conformance_selftest`
- **Binding wrappers**: all binding crates need wrappers for the 4 promoted text utils + future Tier
    1 symbols
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22304526571) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **165 passed** (143 unit + 22 integration), 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Promote algorithm primitives to Tier 1 `pub` API: `sliding_window`, `alg_simhash` (from `simhash`
module), `alg_minhash_256` (from `minhash` module), `alg_cdc_chunks` (from `cdc` module). These are
already implemented as `pub(crate)` — follow the same promotion pattern as text utils (make module
`pub`, promote functions to `pub`, add `pub use` re-exports at crate root, add integration tests).
