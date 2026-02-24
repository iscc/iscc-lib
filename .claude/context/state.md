<!-- assessed-at: 587e97bf47130c2bdc3a77b46e26a64a204f8f92 -->

# Project State

## Status: IN_PROGRESS

## Phase: iscc_decompose truncation hardened — doc gaps, open correctness issues, and OIDC remain

All 23 Tier 1 API symbols are implemented in the Rust core and exposed in all four binding targets:
Python (23/23), Node.js (23/23), WASM (23/23), and C FFI (23/23). The `iscc_decompose` function has
been hardened against truncated input (returns `IsccError::InvalidInput` instead of panicking).
Documentation remains the primary gap: Tutorials section, Rust how-to guide, abbreviations file, and
CNAME are all missing. OIDC publishing pipeline is not yet configured.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 193 `#[test]` functions across `src/` (cdc.rs: 15, codec.rs: 71, lib.rs: 35, simhash.rs: 16,
    streaming.rs: 15, utils.rs: 20, conformance.rs: 1, dct.rs: 8, minhash.rs: 7, wtahash.rs: 5); 244
    total in the `iscc-lib` crate including integration tests; 307 across the full workspace
- `iscc_decompose` hardened against truncated input in `codec.rs`: three new guards return
    `IsccError::InvalidInput` when body bytes are fewer than expected — (1) standard unit path, (2)
    Wide ISCC-CODE path (< 32 bytes), (3) non-wide ISCC-CODE path; 6 regression tests added
- `alg_cdc_chunks` infinite loop bug fixed in `cdc.rs`: guard prevents zero-cut stall when remaining
    buffer < 4 bytes and `utf32=true`; 5 targeted regression tests in place
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- JSON metadata canonicalization uses `serde_json_canonicalizer::to_writer` for RFC 8785 (JCS)
    compliance — `parse_meta_json()` correctly serializes `1.0` as `1` and `1e20` as
    `100000000000000000000`, matching iscc-core's `jcs.canonicalize()` behavior
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD)
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23
- **Open issues** (tracked in `issues.md`): `soft_hash_codes_v0` over-short content-code acceptance
    diverges from reference [normal]; `gen_meta_code_v0` empty Data-URL payload routing [normal];
    `alg_simhash` panics on mismatched digest sizes [normal]; `sliding_window` panics on width < 2
    [normal]; `sliding_window` O(n) String allocations [normal]; codec header parsing expands bytes
    to `Vec<bool>` [normal]; `DataHasher::update` copies input on every call [normal]; several [low]
    issues around dct validation, wtahash bounds

## Python Bindings

**Status**: met

- 23/23 Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- All `gen_*_v0` functions return `PyDict` (translated to typed `IsccResult` subclasses in Python)
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with `Option<inner>` finalize-once pattern
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | BinaryIO` in the Python layer
- 105 test functions across 5 files (`test_conformance.py`, `test_smoke.py`, `test_text_utils.py`,
    `test_algo.py`, `test_streaming.py`)
- `ruff check` and `ruff format --check` clean (CI-verified at HEAD)
- `pytest` passes all conformance vectors (CI-verified at HEAD)
- abi3-py310 wheel configuration in place
- `ty` type checking configured
- OIDC trusted publishing not yet configured

## Node.js Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via napi-rs in `crates/iscc-napi/src/lib.rs`
- `DataHasher` and `InstanceHasher` implemented as `#[napi(js_name)]` structs with constructor/
    update/finalize methods
- 66 tests: 9 in `conformance.test.mjs` + 57 in `functions.test.mjs`
- `npm test` passes all conformance vectors (CI-verified at HEAD)
- Structured results not returned — all gen functions return only the `.iscc` string field

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- 56 tests: 10 in `conformance.rs` + 46 in `unit.rs` (all run via wasm-pack test --node)
- `wasm-pack test --node crates/iscc-wasm` passes all 56 tests (CI-verified at HEAD)
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported

## C FFI

**Status**: met

- 23/23 Tier 1 symbols as `extern "C"` functions in `crates/iscc-ffi/src/lib.rs` (1,880 lines)
- All streaming hasher types fully implemented: `FfiDataHasher` and `FfiInstanceHasher` opaque
    pointer types with complete `new/update/finalize/free` lifecycle functions
- Finalize-once semantics enforced via `Option<Inner>` in the opaque wrapper struct
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 62 `#[test]` Rust unit tests including 11 streaming hasher tests
- C test program covers streaming hasher lifecycle (tests 14–17 in `test_iscc.c`)
- cbindgen generates valid C headers (CI-verified at HEAD)
- C test program compiles with gcc and runs correctly (CI-verified at HEAD)

## Documentation

**Status**: partially met

- 8 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md` (348 lines), `howto/nodejs.md` (276 lines), `howto/wasm.md`
    (333 lines)
- Navigation restructured into Diátaxis categories in `zensical.toml`: How-to Guides (Python,
    Node.js, WebAssembly), Explanation (Architecture), Reference (Rust API, Python API), Benchmarks
- Site builds and deploys via GitHub Pages (Docs CI: PASSING at HEAD)
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- **Missing**: Tutorials section — no tutorials page exists, no `Tutorials` nav group in
    `zensical.toml`; target.md requires "getting started guide (installation, first ISCC code
    generation)" per `specs/documentation.md`
- **Missing**: Rust how-to guide — `docs/howto/rust.md` does not exist; `specs/documentation.md`
    lists "per-language usage guides (Python, Rust, Node.js, WASM)"
- **Missing**: `docs/includes/abbreviations.md` with ISCC-specific abbreviations (ISCC, CDC, DCT,
    MinHash, SimHash, WTA-Hash, BLAKE3, FFI, WASM, PyPI, etc.); `pymdownx.snippets` extension not
    configured in `zensical.toml`
- **Missing**: `docs/CNAME` file with `lib.iscc.codes`

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
- Latest CI run (HEAD `587e97b`): **PASSING** —
    [Run 22347263528](https://github.com/iscc/iscc-lib/actions/runs/22347263528) — all 5 jobs
    success (Rust, Python, Node.js, WASM, C FFI)
- Latest Docs run (HEAD `587e97b`): **PASSING** —
    [Run 22347263520](https://github.com/iscc/iscc-lib/actions/runs/22347263520) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

CI is green; all commits are pushed. Documentation is the primary remaining gap. Priority order:

1. **Abbreviations file** — add `docs/includes/abbreviations.md` with ISCC-specific terms and
    configure `pymdownx.snippets` auto-append in `zensical.toml` (small, self-contained)
2. **CNAME file** — add `docs/CNAME` containing `lib.iscc.codes` (trivial)
3. **Tutorials section** — create a getting started guide (`docs/tutorials/getting-started.md`)
    covering installation and first ISCC code generation across languages; add Tutorials nav group
    to `zensical.toml`
4. **Rust how-to guide** — create `docs/howto/rust.md` covering Rust crate usage, add to nav
5. **Correctness hardening** — fix `soft_hash_codes_v0` divergence from reference (rejects too-short
    Content-Codes), `gen_meta_code_v0` empty Data-URL payload routing, and `alg_simhash` panic on
    mismatched digest sizes [normal issues in issues.md]
6. **OIDC publishing configuration** — configure crates.io and PyPI trusted publishing in
    `release.yml` so releases require no long-lived API keys
