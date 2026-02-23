# Project State

## Status: IN_PROGRESS

## Phase: Tier 1 API expansion & binding enrichment

All 9 `gen_*_v0` functions work in Rust core with structured result types. Python bindings now
return hybrid `IsccResult(dict)` objects with both dict and attribute access, matching iscc-core.
Node.js, WASM, and C FFI bindings still return plain strings. 13 additional Tier 1 API symbols
remain unimplemented. Python streaming support for file-like objects is missing.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions with `*CodeResult` structs, 143 tests, clippy
    clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions returning hybrid `IsccResult(dict)` objects
    (dict + attribute access), 57 tests pass (46 conformance + 11 smoke)
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions (string returns), 46 conformance tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions (string returns), conformance tests in CI
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program, 20 tests
- **Benchmarks**: Criterion (Rust) + pytest-benchmark (Python), 1.3x–158x speedups measured
- **Documentation**: 5 pages at lib.iscc.codes (stock theme, no ISCC branding)
- **CI**: 3 workflows (ci, docs, release), all green

## What's Missing

- **Python streaming**: `gen_data_code_v0`/`gen_instance_code_v0` must accept `BinaryIO` file-like
    objects (not just `bytes`)
- **Tier 1 API (13 symbols)**: text utils (4), algorithm primitives (4), `soft_hash_video_v0`,
    `encode_base64`, `iscc_decompose`, `DataHasher`/`InstanceHasher` streaming types,
    `conformance_selftest`
- **Binding structured returns**: Node.js, WASM, C FFI still return plain strings (not
    dicts/objects)
- **Binding Tier 1 expansion**: all binding crates need the 13 new Tier 1 symbols
- **Documentation branding**: ISCC CSS, logo/favicon, copy-page, llms.txt, Diátaxis nav, tabbed code

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22302746283) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `pytest tests/`: **57 passed** (46 conformance + 11 smoke)
- Node.js: 46 conformance tests (CI-verified)
- WASM: conformance tests pass in CI

## Next Milestone

Add Python streaming support for `gen_data_code_v0`/`gen_instance_code_v0` to accept `BinaryIO`
file-like objects, completing the Python drop-in compatibility with iscc-core. Alternatively, expose
the remaining 13 Tier 1 API symbols in the Rust core (text utils, algorithm primitives, streaming
types, etc.) — these are needed before any binding crate can offer them.
