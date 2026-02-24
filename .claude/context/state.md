<!-- assessed-at: 58183aa52315e09a9f65cb0bf6d73b8548932e24 -->

# Project State

## Status: IN_PROGRESS

## Phase: Documentation complete — robustness issues remain

All 23 Tier 1 API symbols are implemented in the Rust core and exposed in all four binding targets:
Python (23/23), Node.js (23/23), WASM (23/23), and C FFI (23/23). All 11 documentation pages now
have `icon: lucide/...` and `description:` front matter (page-level, matching iscc-usearch pattern),
completing the Documentation target. CI is green across all 5 jobs. Remaining work is robustness
fixes for two [normal] public API panic issues and the partially-met Benchmarks/CI-CD sections.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 205 `#[test]` functions across `src/` (lib.rs: 40, cdc.rs: 15, codec.rs: 71, simhash.rs: 23,
    streaming.rs: 15, utils.rs: 20, conformance.rs: 1, dct.rs: 8, minhash.rs: 7, wtahash.rs: 5)
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- All prior correctness fixes in place: empty Data-URL payload routing, `soft_hash_codes_v0`
    bit-length validation, `iscc_decompose` truncated input guards, `alg_cdc_chunks` infinite loop
    guard
- `sliding_window_strs` added as `pub(crate)` in `simhash.rs` — zero-copy slice variant used by
    `soft_hash_meta_v0` (name and extra) and `soft_hash_text_v0`; eliminates per-n-gram `String`
    allocation while keeping the public `sliding_window` API unchanged
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- JSON metadata canonicalization uses `serde_json_canonicalizer::to_writer` for RFC 8785 (JCS)
    compliance
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23
- **Open issues** (tracked in `issues.md`): `alg_simhash` panics on mismatched digest sizes
    [normal]; `sliding_window` panics on width < 2 [normal]; codec header parsing expands bytes to
    `Vec<bool>` [normal]; `DataHasher::update` copies input on every call [normal]; `alg_dct` allows
    non-power-of-two even lengths [low]; `alg_wtahash` panics on short vectors [low]

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

- Rewritten as public-facing polyglot developer README (215 lines)
- ✅ CI badge; version badges commented out with TODO (packages not yet published — correct behavior)
- ✅ Tagline: "High-performance polyglot implementation of ISO 24138:2024 — International Standard
    Content Code (ISCC)"
- ✅ Key Features: 6 bullet points
- ✅ "What is the ISCC" and "What is iscc-lib" sections
- ✅ ISCC Architecture diagram, MainTypes table
- ✅ Installation per-language (Rust/cargo, Python/pip, Node.js/npm, WASM/npm)
- ✅ Quick Start code examples for all 4 languages
- ✅ Implementors Guide, Documentation link, Contributing section, Apache-2.0 license, Maintainers
- All 7 target.md verification criteria met

## Documentation

**Status**: met

- 11 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md`, `howto/nodejs.md`, `howto/wasm.md`, `howto/rust.md`,
    `development.md`, `tutorials/getting-started.md`
- Navigation in `zensical.toml` has: Tutorials (Getting Started), How-to Guides (Rust, Python,
    Node.js, WebAssembly), Explanation (Architecture), Reference (Rust API, Python API), Benchmarks,
    Development
- All 11 pages have `icon: lucide/...` and `description:` YAML front matter (page-level, matching
    iscc-usearch pattern); icon assignments: house (index), rocket (getting-started), cog (rust
    howto), terminal (python howto), hexagon (nodejs howto), globe (wasm howto), blocks
    (architecture), book-open (rust-api, api), gauge (benchmarks), git-pull-request (development)
- Site builds and deploys via GitHub Pages (Docs CI: PASSING —
    [Run 22351784303](https://github.com/iscc/iscc-lib/actions/runs/22351784303))
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations; `pymdownx.snippets`
    auto-appended to all pages
- ✅ `docs/tutorials/getting-started.md` (154 lines): covers installation, conformance self-test,
    Meta-Code, Text-Code, Instance-Code, streaming, next steps
- ✅ All 4 language how-to guides complete (Rust, Python, Node.js, WASM)

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
    [Run 22351784323](https://github.com/iscc/iscc-lib/actions/runs/22351784323) — all 5 jobs
    success (Rust, Python, Node.js, WASM, C FFI)
- Latest Docs run: **PASSING** —
    [Run 22351784303](https://github.com/iscc/iscc-lib/actions/runs/22351784303) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

CI is green; documentation target is now fully met. Focus on the two [normal] public API robustness
issues tracked in `issues.md`:

1. **`alg_simhash` panics on mismatched digest sizes** — Tier 1 public API; fix by validating all
    digest lengths are equal and returning `IsccError` (requires changing return type to
    `IsccResult<Vec<u8>>`)
2. **`sliding_window` panics on `width < 2`** — Tier 1 public API bound to all languages; fix by
    returning `IsccResult<Vec<String>>` for the public function (DoS vector for untrusted input)

These two correctness hardening tasks should be addressed before performance optimizations
(`DataHasher::update` buffer allocation, codec `bytes_to_bits` allocation).
