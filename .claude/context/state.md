<!-- assessed-at: 371a808622b28455d2ab6c388b3a9c891e41ea20 -->

# Project State

## Status: IN_PROGRESS

## Phase: Diátaxis nav and howto guides added — abbreviations, CNAME, Rust howto, Tutorials, and OIDC remaining

All 23 Tier 1 API symbols are implemented in the Rust core and exposed in all four binding targets:
Python (23/23), Node.js (23/23), WASM (23/23), and C FFI (23/23). JSON metadata canonicalization now
properly uses RFC 8785 (JCS) via `serde_json_canonicalizer`. Navigation has been restructured into
Diátaxis categories, and per-language how-to guides for Python, Node.js, and WASM have been added.
Remaining documentation gaps: Tutorials section, Rust how-to guide, abbreviations file (with
pymdownx.snippets), and CNAME. OIDC publishing pipeline not yet configured. 8 local commits are
ahead of remote; CI has not yet run on the JCS fix or Diátaxis restructuring.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 184 `#[test]` functions in `lib.rs` (2 new JCS tests added in commit `9f9bfdd`); 280+ total across
    workspace Rust crates
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- JSON metadata canonicalization uses `serde_json_canonicalizer::to_writer` for RFC 8785 (JCS)
    compliance — `parse_meta_json()` correctly serializes `1.0` as `1` and `1e20` as
    `100000000000000000000`, matching iscc-core's `jcs.canonicalize()` behavior (fixed in `9f9bfdd`,
    previously only claimed but not implemented)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified on remote HEAD)
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
    pointer types with complete `new/update/finalize/free` lifecycle functions
- Finalize-once semantics enforced via `Option<Inner>` in the opaque wrapper struct
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 62 `#[test]` Rust unit tests including 11 streaming hasher tests
- C test program covers streaming hasher lifecycle (tests 14–17 in `test_iscc.c`)
- cbindgen generates valid C headers (CI-verified)
- C test program compiles with gcc and runs correctly (CI-verified)

## Documentation

**Status**: partially met

- 8 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md` (348 lines), `howto/nodejs.md` (276 lines), `howto/wasm.md`
    (333 lines) — last 3 added in commit `486c539` (local, not yet deployed)
- Navigation restructured into Diátaxis categories in `zensical.toml` (commit `486c539`): How-to
    Guides (Python, Node.js, WebAssembly), Explanation (Architecture), Reference (Rust API, Python
    API), Benchmarks
- Site builds and deploys via GitHub Pages (CI-verified, latest Docs run: success on remote HEAD)
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
- Latest CI run (against remote HEAD `ef62d13`): **PASSING** —
    [Run 22324845560](https://github.com/iscc/iscc-lib/actions/runs/22324845560) — all 5 jobs
    success (Python, C FFI, Rust, Node.js, WASM)
- Latest Docs run: **PASSING** —
    [Run 22324844023](https://github.com/iscc/iscc-lib/actions/runs/22324844023) — build + deploy
    success
- **8 local commits not yet pushed to remote** — CI has not run on JCS fix (`9f9bfdd`), Diátaxis nav
    restructuring (`486c539`), or subsequent CID workflow commits
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

Documentation is the primary remaining gap. Priority order:

1. **Push local commits to remote** — 8 commits are ahead of `origin/main`; CI must run against the
    JCS fix and Diátaxis nav restructuring to confirm they pass
2. **Abbreviations file** — add `docs/includes/abbreviations.md` with ISCC-specific terms and
    configure `pymdownx.snippets` auto-append in `zensical.toml` (small, self-contained)
3. **CNAME file** — add `docs/CNAME` containing `lib.iscc.codes` (trivial)
4. **Tutorials section** — create a getting started guide (`docs/tutorials/getting-started.md`)
    covering installation and first ISCC code generation across languages; add Tutorials nav group
    to `zensical.toml`
5. **Rust how-to guide** — create `docs/howto/rust.md` covering Rust crate usage, add to nav
6. **OIDC publishing configuration** — configure crates.io and PyPI trusted publishing in
    `release.yml` so releases require no long-lived API keys
