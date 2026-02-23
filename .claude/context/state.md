<!-- assessed-at: 9b06e53e8dfc20dd02b90c7f4c47e063e96b578d -->

# Project State

## Status: IN_PROGRESS

## Phase: Full binding parity — all four binding targets at 23/23 Tier 1

All 23 Tier 1 API symbols are implemented in the Rust core and exposed in all four binding targets:
Python (23/23), Node.js (23/23), WASM (23/23), and C FFI (23/23). CI is fully green across all 5
jobs. Remaining gaps are in documentation branding and publishing pipeline configuration.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 180 `#[test]` functions in `lib.rs` alone; 280+ total across workspace Rust crates
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified)
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23

## Python Bindings

**Status**: met

- 23/23 Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- All `gen_*_v0` functions return `PyDict` (translated to typed `IsccResult` subclasses in Python)
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with `Option<inner>` finalize-once pattern
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | BinaryIO` in the Python layer
- 105 test functions across 5 files (`test_conformance.py`, `test_smoke.py`, `test_text_utils.py`,
    `test_algo.py`, `test_streaming.py`)
- `ruff check` and `ruff format --check` clean (CI-verified)
- `pytest` passes all conformance vectors (CI-verified)
- abi3-py310 wheel configuration in place
- `ty` type checking configured
- OIDC trusted publishing not yet configured

## Node.js Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via napi-rs in `crates/iscc-napi/src/lib.rs`
- `DataHasher` and `InstanceHasher` implemented as `#[napi(js_name)]` structs with constructor/
    update/finalize methods
- 66 tests: 9 in `conformance.test.mjs` + 57 in `functions.test.mjs`
- `npm test` passes all conformance vectors (CI-verified)
- Structured results not returned — all gen functions return only the `.iscc` string field

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- 56 tests: 10 in `conformance.rs` + 46 in `unit.rs` (all run via wasm-pack test --node)
- `wasm-pack test --node crates/iscc-wasm` passes all 56 tests (CI-verified)
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported

## C FFI

**Status**: met

- 23/23 Tier 1 symbols as `extern "C"` functions in `crates/iscc-ffi/src/lib.rs` (1,880 lines)
- All streaming hasher types fully implemented: `FfiDataHasher` and `FfiInstanceHasher` opaque
    pointer types with complete `new/update/finalize/free` lifecycle functions (8 new exported
    symbols: `iscc_data_hasher_new`, `iscc_data_hasher_update`, `iscc_data_hasher_finalize`,
    `iscc_data_hasher_free`, `iscc_instance_hasher_new`, `iscc_instance_hasher_update`,
    `iscc_instance_hasher_finalize`, `iscc_instance_hasher_free`)
- Finalize-once semantics enforced via `Option<Inner>` in the opaque wrapper struct
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 62 `#[test]` Rust unit tests including 11 streaming hasher tests
- C test program covers streaming hasher lifecycle (tests 14–17 in `test_iscc.c`)
- cbindgen generates valid C headers (CI-verified)
- C test program compiles with gcc and runs correctly (CI-verified)

## Documentation

**Status**: partially met

- 5 pages deployed to lib.iscc.codes via GitHub Pages: `index.md`, `architecture.md`, `rust-api.md`,
    `api.md`, `benchmarks.md`
- Site builds and deploys via GitHub Pages (CI-verified, Docs workflow: success)
- Uses `zensical` build tool with `zensical.toml` config
- Tabbed code examples configured via `pymdownx.tabbed` with `alternate_style = true`
- Missing: ISCC branding — no custom CSS, no ISCC logo/favicon, no primary/accent color
    customization matching iscc-usearch
- Missing: copy-page feature and `llms-full.txt` generation
- Missing: Diátaxis navigation framework (tutorials, howto, explanation, reference sections)
- Missing: per-language how-to guides (Node.js, WASM guides not present in nav)

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions in
    `crates/iscc-lib/benches/benchmarks.rs`
- pytest-benchmark comparison files exist: `benchmarks/python/bench_iscc_lib.py` and
    `benchmarks/python/bench_iscc_core.py` (101 lines each) plus `conftest.py`
- Speedup factors documented in `docs/benchmarks.md`
- Missing: CI does not run benchmarks automatically; no published benchmark results in CI artifacts

## CI/CD and Publishing

**Status**: partially met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers all 5 targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js (napi
    build, test), WASM (wasm-pack test), C FFI (cbindgen, gcc, test)
- Latest CI run: **PASSING** —
    [Run 22322848137](https://github.com/iscc/iscc-lib/actions/runs/22322848137) — all 5 jobs
    success (Rust, Node.js, WASM, Python, C FFI)
- Latest Docs run: **PASSING** —
    [Run 22322848154](https://github.com/iscc/iscc-lib/actions/runs/22322848154)
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired (npm does not support OIDC; `NODE_AUTH_TOKEN`
    secret setup not documented as complete)
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

With all four binding targets at 23/23 Tier 1 parity and CI fully green, the highest-value next
priorities are:

1. **Documentation branding** — add ISCC custom CSS (primary/accent colors), logo, and favicon
    matching iscc-usearch at `usearch.iscc.codes`; add copy-page feature and `llms-full.txt`
    generation; restructure navigation to follow Diátaxis framework with per-language how-to guides
2. **OIDC publishing configuration** — configure crates.io and PyPI trusted publishing in
    `release.yml` so releases require no long-lived API keys
