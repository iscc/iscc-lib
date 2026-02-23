# Project State

## Status: IN_PROGRESS

## Phase: Python drop-in compatibility — dict returns, then Tier 1 API expansion

All 9 `gen_*_v0` functions in Rust core now return dedicated `*CodeResult` structs with all fields
matching iscc-core dicts (metahash, name, description, meta, characters, parts, datahash, filesize).
143 Rust tests and 49 Python tests pass, CI green. Binding crates still return plain strings —
Python dict returns are the immediate next step.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions returning structured `*CodeResult` types,
    `types.rs` module, 143 tests, clippy/fmt clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions via PyO3, type stubs, 49 tests — returns
    strings (not dicts yet)
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions via napi-rs, 46 conformance tests —
    returns strings
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions via wasm-bindgen, conformance tests in CI —
    returns strings
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program — returns strings
- **Criterion benchmarks**: all 9 gen functions benchmarked
- **Comparative pytest-benchmark**: measured 1.3x–158x speedups vs iscc-core
- **Documentation**: 5 pages (landing, Python API, Rust API, architecture, benchmarks) — stock theme
- **CI**: 3 workflows (ci.yml, docs.yml, release.yml), all green
- **Infrastructure**: GitHub Pages (lib.iscc.codes), `@iscc` npm org, NPM_TOKEN secret

## What's Missing

- **Python dict returns**: PyO3 bindings extract `.iscc` string — must return `dict` with all
    `*CodeResult` fields to match iscc-core API
- **Tier 1 API expansion**: 13 additional public symbols needed — text utils (4), algorithm
    primitives (4), `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`,
    `DataHasher`/`InstanceHasher` streaming types, `conformance_selftest`
- **Python streaming**: `gen_data_code_v0`/`gen_instance_code_v0` must accept file-like objects
- **Binding updates for Tier 1**: Node.js, WASM, C FFI need new Tier 1 symbols + structured returns
- **Documentation branding**: missing ISCC CSS, logo/favicon, copy-page feature, llms.txt, Diátaxis
    navigation, tabbed multi-language code examples

## CI

- **Latest run: PASSING** — [Run](https://github.com/iscc/iscc-lib/actions/runs/22301377257) —
    conclusion: success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- Python tests (`pytest tests/`): **49 passed**
- Node.js tests: 46 passed (9 suites)
- WASM: conformance tests pass in CI
- No `unsafe` in core crate

## Next Milestone

Implement Python dict returns in the PyO3 binding layer. Convert all 9 `gen_*_v0` Python functions
to return `dict` objects with all fields from `*CodeResult` structs (matching iscc-core API). Update
`.pyi` type stubs and Python conformance tests to expect dicts instead of strings.
