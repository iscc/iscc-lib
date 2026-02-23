# Project State

## Status: IN_PROGRESS

## Phase: API expansion — structured returns, Tier 1 promotion, Python drop-in compatibility

All 9 `gen_*_v0` functions implemented in Rust core with 143 tests passing. All 4 binding crates
(Python, Node.js, WASM, C FFI) complete with conformance tests. CI green, docs workflow green. Three
major areas remain before the target is fully met: structured return types + Tier 1 API expansion,
Python drop-in compatibility, and documentation branding.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 9 gen functions, 8 modules, 143 tests, clippy/fmt clean
- **Python bindings (`crates/iscc-py/`)**: 9 functions via PyO3, type stubs, 49 tests passing
- **Node.js bindings (`crates/iscc-napi/`)**: 9 functions via napi-rs, 46 conformance tests
- **WASM bindings (`crates/iscc-wasm/`)**: 9 functions via wasm-bindgen, conformance tests in CI
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen, C test program
- **Criterion benchmarks**: all 9 gen functions benchmarked
- **Comparative pytest-benchmark**: measured 1.3x–158x speedups vs iscc-core
- **Documentation**: 5 pages (landing, Python API, Rust API, architecture, benchmarks) — stock theme
- **CI**: 3 workflows (ci.yml, docs.yml, release.yml), all green
- **Infrastructure**: GitHub Pages (lib.iscc.codes), `@iscc` npm org, NPM_TOKEN secret

## What's Missing

- **Structured return types**: gen functions return `String`, need structs with fields (metahash,
    name, characters, datahash, filesize, parts) matching iscc-core dicts
- **Tier 1 API expansion**: 13 additional public symbols needed — text utils (4), algorithm
    primitives (4), `soft_hash_video_v0`, `encode_base64`, `iscc_decompose`, `DataHasher`/
    `InstanceHasher` streaming types, `conformance_selftest`
- **Python drop-in compatibility**: functions must return dicts (not strings), streaming functions
    must accept file-like objects, new Tier 1 functions must be bound
- **Binding updates**: Node.js, WASM, C FFI need to expose new Tier 1 symbols
- **Documentation branding**: missing ISCC CSS, logo/favicon, copy-page feature, llms.txt, Diátaxis
    navigation, tabbed multi-language code examples, Open Graph tags
- **Untracked napi artifacts**: `crates/iscc-napi/` has local build artifacts not in `.gitignore`

## CI

- **Latest run: PASSING** —
    [Run #22300608500](https://github.com/iscc/iscc-lib/actions/runs/22300608500) — conclusion:
    success
- All 3 workflows (ci, docs, release) green

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- Python tests (`pytest tests/`): **49 passed**
- Node.js tests: **46 passed** (9 suites, 0 failures)
- WASM: conformance tests pass in CI (requires wasm-pack runner)
- No `unsafe` in core crate

## Next Milestone

Implement structured return types for the 9 `gen_*_v0` functions (prerequisite for Python dict
returns and additional conformance fields). Then promote existing `pub(crate)` functions to `pub`
for Tier 1 API expansion. See `specs/rust-core.md` for full spec and `handoff.md` for priority
order.
