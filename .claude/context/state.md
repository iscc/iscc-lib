<!-- assessed-at: 38eccefba2417cc2eb71bc5b6ef649a834a8afaa -->

# Project State

## Status: IN_PROGRESS

## Phase: README rewrite complete — documentation gaps (tutorials, Rust howto, abbreviations, CNAME, nav icons) remain

All 23 Tier 1 API symbols are implemented in the Rust core and exposed in all four binding targets:
Python (23/23), Node.js (23/23), WASM (23/23), and C FFI (23/23). This iteration completed the
README rewrite — the public-facing polyglot developer README now meets all target criteria. CI is
green. Remaining gaps are all in documentation: Tutorials section, Rust how-to guide, abbreviations
file + snippets config, CNAME, and nav icons for top-level nav sections.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 198 `#[test]` functions across `src/` (lib.rs: 40, cdc.rs: 15, codec.rs: 71, simhash.rs: 16,
    streaming.rs: 15, utils.rs: 20, conformance.rs: 1, dct.rs: 8, minhash.rs: 7, wtahash.rs: 5)
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- All prior correctness fixes in place: empty Data-URL payload routing, `soft_hash_codes_v0`
    bit-length validation, `iscc_decompose` truncated input guards, `alg_cdc_chunks` infinite loop
    guard
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- JSON metadata canonicalization uses `serde_json_canonicalizer::to_writer` for RFC 8785 (JCS)
    compliance
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23
- **Open issues** (tracked in `issues.md`): `alg_simhash` panics on mismatched digest sizes
    [normal]; `sliding_window` panics on width < 2 [normal]; `sliding_window` O(n) String
    allocations [normal]; codec header parsing expands bytes to `Vec<bool>` [normal];
    `DataHasher::update` copies input on every call [normal]; `alg_dct` allows non-power-of-two even
    lengths [low]; `alg_wtahash` panics on short vectors [low]

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

## README

**Status**: met

- Rewritten as public-facing polyglot developer README (215 lines after advance + minor em dash fix)
- ✅ CI badge; version badges commented out with TODO (packages not yet published — correct behavior)
- ✅ Tagline: "High-performance polyglot implementation of ISO 24138:2024 — International Standard
    Content Code (ISCC)"
- ✅ Key Features: 6 bullet points (similarity-preserving, multi-level, self-describing, ISO
    standardized, polyglot, conformance-tested)
- ✅ "What is the ISCC" section with explanation of ISCC purpose and algorithmic design
- ✅ "What is iscc-lib" section with relationship to iscc-core and target ecosystems
- ✅ ISCC Architecture diagram (image from iscc-core repo)
- ✅ ISCC MainTypes table (6 rows: META, SEMANTIC, CONTENT, DATA, INSTANCE, ISCC)
- ✅ Installation per-language (Rust/cargo, Python/pip, Node.js/npm, WASM/npm)
- ✅ Quick Start code examples for all 4 languages (gen_meta_code_v0)
- ✅ Implementors Guide listing all 9 `gen_*_v0` entry points and conformance vector link
- ✅ Documentation link to lib.iscc.codes
- ✅ Contributing section with Telegram developer chat link
- ✅ Apache-2.0 license
- ✅ Maintainers: @titusz
- ✅ No development workflow content (CID loop, dev container, pre-commit hooks absent)
- All 7 target.md verification criteria met

## Documentation

**Status**: partially met

- 9 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md` (348 lines), `howto/nodejs.md` (276 lines), `howto/wasm.md`
    (333 lines), `development.md` (235 lines)
- Navigation in `zensical.toml` has: How-to Guides (Python, Node.js, WebAssembly), Explanation
    (Architecture), Reference (Rust API, Python API), Benchmarks, Development
- Site builds and deploys via GitHub Pages (Docs CI: PASSING at HEAD)
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- **Missing**: Nav icons — `zensical.toml` nav sections have no `icon` attribute; the docs spec
    (updated in commit `270bf18`) requires "Each top-level nav section has a Material for MkDocs
    icon, matching the style of iscc/iscc-usearch at usearch.iscc.codes"; verification criteria
    explicitly checks for icons
- **Missing**: Tutorials section — no `docs/tutorials/` directory, no Tutorials nav group in
    `zensical.toml`; spec requires "getting started guide (installation, first ISCC code
    generation)"
- **Missing**: Rust how-to guide — `docs/howto/rust.md` does not exist; spec lists "per-language
    usage guides (Python, Rust, Node.js, WASM)"
- **Missing**: `docs/includes/abbreviations.md` with ISCC-specific abbreviations;
    `pymdownx.snippets` extension not configured in `zensical.toml`
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
- Latest CI run (HEAD `38eccef`): **PASSING** —
    [Run 22349439644](https://github.com/iscc/iscc-lib/actions/runs/22349439644) — all 5 jobs
    success
- Latest Docs run: **PASSING** —
    [Run 22349439653](https://github.com/iscc/iscc-lib/actions/runs/22349439653) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

CI is green; all commits are pushed. The README rewrite is now complete and meets all target
criteria. All remaining gaps are in the documentation section. Priority order:

1. **Nav icons** — add Material for MkDocs icon attributes to each top-level nav section in
    `zensical.toml` (newly required by docs spec added in `270bf18`); query iscc/iscc-usearch via
    deepwiki MCP for exact icon names and config format
2. **Abbreviations file + snippets config** — add `docs/includes/abbreviations.md` with
    ISCC-specific terms and configure `pymdownx.snippets` auto-append in `zensical.toml` (small,
    self-contained)
3. **CNAME file** — add `docs/CNAME` containing `lib.iscc.codes` (trivial)
4. **Tutorials section** — create `docs/tutorials/getting-started.md` covering installation and
    first ISCC code generation across languages; add Tutorials nav group to `zensical.toml`
5. **Rust how-to guide** — create `docs/howto/rust.md` covering Rust crate usage; add to nav
6. **OIDC publishing configuration** — configure crates.io and PyPI trusted publishing in
    `release.yml` so releases require no long-lived API keys
