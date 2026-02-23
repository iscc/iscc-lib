# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion & binding enrichment

All 9 `gen_*_v0` functions are implemented and conformance-tested across Rust, Python, Node.js,
WASM, and C FFI. Python bindings return hybrid `IsccResult(dict)` objects with BinaryIO streaming
support. The `ty` type checker fix is complete — all quality gates now pass. Remaining work: 13
additional Tier 1 API symbols, structured returns for non-Python bindings, and documentation
branding.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions with `*CodeResult` structs, 143 tests, clippy
    clean
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

- **Tier 1 API (13 symbols)**: text utils (`text_clean`, `text_remove_newlines`, `text_trim`,
    `text_collapse`), algorithm primitives (`sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`,
    `alg_simhash`), `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`,
    `DataHasher`/`InstanceHasher` streaming types, `conformance_selftest`
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Binding Tier 1 expansion**: all binding crates need the 13 new Tier 1 symbols
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code
    examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22303920539) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **63 passed** (46 conformance + 17 smoke/streaming)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Promote existing `pub(crate)` text utilities and algorithm primitives to `pub` Tier 1 API — these 8
functions (`text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`, `sliding_window`,
`alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`) are already implemented internally and need
visibility promotion, documentation, and binding wrappers.
