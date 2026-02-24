<!-- assessed-at: 80dbfa67507eb48185709184e2edabb60a2046f5 -->

# Project State

## Status: IN_PROGRESS

## Phase: sliding_window hardened — Java bindings not started

All 23 Tier 1 API symbols remain implemented and exposed in all four existing binding targets
(Python, Node.js, WASM, C FFI). The `sliding_window` panic on `width < 2` has been resolved in this
iteration — all four binding crates now propagate the error idiomatically. Java/Maven Central
bindings were added to the target by Titusz; no Java crate exists yet. The README and documentation
site are now **partially met** because both require Java content that is not yet written.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 206 `#[test]` functions in `src/` (lib.rs: 40, cdc.rs: 15, codec.rs: 71, simhash.rs: 24,
    streaming.rs: 15, utils.rs: 20, conformance.rs: 1, dct.rs: 8, minhash.rs: 7, wtahash.rs: 5); 53
    additional tests in `tests/` (test_algorithm_primitives.rs: 31, test_text_utils.rs: 22)
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- All prior correctness fixes in place: empty Data-URL payload routing, `soft_hash_codes_v0`
    bit-length validation, `iscc_decompose` truncated input guards, `alg_cdc_chunks` infinite loop
    guard, `alg_simhash` digest-length validation
- **`sliding_window` hardened** (this iteration): `pub fn sliding_window` now returns
    `IsccResult<Vec<String>>`; returns
    `Err(IsccError::InvalidInput("Sliding window width must be 2   or bigger."))` for `width < 2`
    instead of panicking. Internal `sliding_window_strs` and `sliding_window_bytes` retain
    `debug_assert!` (called only from validated paths).
- `sliding_window_strs` is `pub(crate)` in `simhash.rs` — zero-copy slice variant used by
    `soft_hash_meta_v0` and `soft_hash_text_v0`; eliminates per-n-gram `String` allocation
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- JSON metadata canonicalization uses `serde_json_canonicalizer::to_writer` for RFC 8785 (JCS)
    compliance
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23
- **Open issues** (tracked in `issues.md`): codec header parsing expands bytes to `Vec<bool>`
    [normal]; `DataHasher::update` copies input on every call [normal]; `alg_dct` allows
    non-power-of-two even lengths [low]; `alg_wtahash` panics on short vectors [low]; TypeScript
    port evaluation [low]

## Python Bindings

**Status**: met

- 23/23 Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- All `gen_*_v0` functions return `PyDict` (translated to typed `IsccResult` subclasses in Python)
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with `Option<inner>` finalize-once pattern
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | BinaryIO` in the Python layer
- **`sliding_window` updated** (this iteration): returns `PyResult<Vec<String>>` and raises
    `ValueError` on `width < 2`
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
- **`sliding_window` updated** (this iteration): returns `napi::Result<Vec<String>>` and throws on
    `width < 2`
- 66 tests: 9 in `conformance.test.mjs` + 57 in `functions.test.mjs`
- `npm test` passes all conformance vectors (CI-verified at HEAD)
- Structured results not returned — all gen functions return only the `.iscc` string field

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- **`sliding_window` updated** (this iteration): propagates `IsccError` as `JsError` on `width < 2`
- 56 tests: 10 in `conformance.rs` + 46 in `unit.rs` (all run via wasm-pack test --node)
- `wasm-pack test --node crates/iscc-wasm` passes all 56 tests (CI-verified at HEAD)
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported

## C FFI

**Status**: met

- 23/23 Tier 1 symbols as `extern "C"` functions in `crates/iscc-ffi/src/lib.rs` (1,880+ lines)
- All streaming hasher types fully implemented: `FfiDataHasher` and `FfiInstanceHasher` opaque
    pointer types with complete `new/update/finalize/free` lifecycle functions
- Finalize-once semantics enforced via `Option<Inner>` in the opaque wrapper struct
- **`iscc_sliding_window` updated** (this iteration): propagates error via thread-local last-error
    and returns null on `width < 2`; verified in new test `test_sliding_window_width_too_small`
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 62 `#[test]` Rust unit tests including 11 streaming hasher tests
- C test program covers streaming hasher lifecycle (tests 14–17 in `test_iscc.c`)
- cbindgen generates valid C headers (CI-verified at HEAD)
- C test program compiles with gcc and runs correctly (CI-verified at HEAD)

## Java Bindings

**Status**: not started

- Target added by Titusz (commit 9ca94f2): JNI bridge crate (`iscc-jni`), Java wrapper with
    idiomatic API, platform-specific native libraries bundled in JAR under `META-INF/native/`,
    loader class extracts correct native library at runtime
- No `crates/iscc-jni/` exists; no Java source tree exists; no Maven/Gradle configuration exists
- Devcontainer does not yet include JDK 17+ or Maven/Gradle
- Requires JDK 17+ and Maven (or Gradle) in the devcontainer before any implementation begins

## README

**Status**: partially met

- Rewritten as public-facing polyglot developer README (182 lines)
- ✅ CI badge; version badges commented out with TODO (packages not yet published)
- ✅ Tagline: "High-performance polyglot implementation of ISO 24138:2024 — International Standard
    Content Code (ISCC)"
- ✅ Key Features: 6 bullet points — but **Polyglot** line reads "Python, Node.js, WASM, and C FFI"
    (missing Java)
- ✅ "What is the ISCC" and "What is iscc-lib" sections
- ✅ ISCC Architecture diagram, MainTypes table
- ✅ Installation: Rust, Python, Node.js, WASM sections present — **Java/Maven section missing**
- ✅ Quick Start: Rust, Python, Node.js, WASM examples — **Java quick start missing**
- ✅ Implementors Guide, Documentation link, Contributing, Apache-2.0 license, Maintainers
- Missing: Java/Maven installation section, Java quick start code example, Java in Key Features

## Documentation

**Status**: partially met

- 11 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md`, `howto/nodejs.md`, `howto/wasm.md`, `howto/rust.md`,
    `development.md`, `tutorials/getting-started.md`
- Navigation in `zensical.toml` has: Tutorials (Getting Started), How-to Guides (Rust, Python,
    Node.js, WebAssembly), Explanation (Architecture), Reference (Rust API, Python API), Benchmarks,
    Development
- All 11 pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages (Docs CI: PASSING —
    [Run 22353146329](https://github.com/iscc/iscc-lib/actions/runs/22353146329))
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations
- Missing: `howto/java.md` how-to guide for Java
- Missing: Java API reference page
- Missing: Java code examples in existing tabbed code blocks (spec requires Python, Rust, Java,
    Node.js, WASM tabs)

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
- `ci.yml` covers all 5 existing targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js
    (napi build, test), WASM (wasm-pack test), C FFI (cbindgen, gcc, test)
- Latest CI run: **PASSING** —
    [Run 22353146351](https://github.com/iscc/iscc-lib/actions/runs/22353146351) — all 5 jobs
    success (Rust, Python, Node.js, WASM, C FFI)
- Latest Docs run: **PASSING** —
    [Run 22353146329](https://github.com/iscc/iscc-lib/actions/runs/22353146329) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready
- Missing: Java CI job (not applicable until Java bindings exist)

## Next Milestone

CI is green. `sliding_window` robustness is fully resolved. Java bindings are the next major feature
addition per the updated target. Before implementing, the devcontainer needs JDK 17+ and Maven (or
Gradle). The implementation order is:

1. **Devcontainer setup**: add JDK 17+ and Maven to `.devcontainer/Dockerfile`; update
    `notes/02-language-bindings.md` with Java/JNI binding architecture
2. **`crates/iscc-jni`**: JNI bridge crate using `jni` crate, exposing all 23 Tier 1 symbols as
    Java-callable native methods
3. **Java wrapper library**: idiomatic Java API (`IsccLib.genMetaCodeV0("title")`), native library
    loader class, `META-INF/native/` bundling for all platforms
4. **README + docs**: add Java installation (Maven/Gradle), quick start example, howto guide, Java
    API reference, Java tabs in existing code examples

The existing [normal] allocation issues (`DataHasher::update` copies, codec `bytes_to_bits`
intermediate allocation) are lower priority than the Java bindings milestone.
