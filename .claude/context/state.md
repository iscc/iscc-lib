<!-- assessed-at: 98f2238896944ced6785fec248913e9817a9213f -->

# Project State

## Status: IN_PROGRESS

## Phase: Binding parity — WASM at 23/23, C FFI at 21/23

All 23 Tier 1 API symbols are implemented in the Rust core. Python (23/23), Node.js (23/23), and
WASM (23/23) bindings are feature-complete including DataHasher/InstanceHasher streaming classes. C
FFI remains at 21/23, missing the two streaming hasher types. CI is fully green across all 5 jobs.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 180 `#[test]` functions in `lib.rs` alone; 280 total across workspace Rust crates
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
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs (added in iteration 8)
- 56 tests: 10 in `conformance.rs` + 46 in `unit.rs` (all run via wasm-pack test --node)
- `wasm-pack test --node crates/iscc-wasm` passes all 56 tests (CI-verified)
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported

## C FFI

**Status**: partially met

- 21/23 Tier 1 symbols as `extern "C"` functions in `crates/iscc-ffi/src/lib.rs` (1,495 lines)
- Missing: `DataHasher` and `InstanceHasher` streaming classes (no opaque pointer lifecycle
    functions: `iscc_data_hasher_new`, `iscc_data_hasher_update`, `iscc_data_hasher_finalize`,
    `iscc_data_hasher_free` and equivalents for InstanceHasher)
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 50 `#[test]` Rust unit tests
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
    [Run 22322401441](https://github.com/iscc/iscc-lib/actions/runs/22322401441) — all 5 jobs
    success
- Latest Docs run: **PASSING** —
    [Run 22322405183](https://github.com/iscc/iscc-lib/actions/runs/22322405183)
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired (npm does not support OIDC; `NODE_AUTH_TOKEN`
    secret setup not documented as complete)
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

Add `DataHasher` and `InstanceHasher` streaming classes to C FFI using the opaque pointer lifecycle
pattern (`*mut T` + create/update/finalize/free functions). This is the last remaining binding gap
before all four binding targets reach 23/23 Tier 1 parity. The pattern: allocate on heap via
`Box::into_raw`, accept the raw pointer in update/finalize functions, free via `Box::from_raw` in a
dedicated `iscc_data_hasher_free` function. After C FFI reaches 23/23, the next priorities are
documentation branding (ISCC colors, logo, favicon) and OIDC publishing configuration.
