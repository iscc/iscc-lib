<!-- assessed-at: 0983881a71d71a28028a21826e0079b831fabc28 -->

# Project State

## Status: IN_PROGRESS

## Phase: JNI bridge 23/23 complete — Java wrapper, build config, and tests pending

The `iscc-jni` crate now exposes all 23 Tier 1 symbols via 29 `extern "system"` JNI functions
(streaming hashers expand to 4 JNI functions each). The Rust side of the Java bindings is complete.
The Java side — wrapper class, Maven/Gradle build, native library bundling, loader, devcontainer
JDK, CI job, and tests — does not exist yet. Both CI and Docs workflows are green on the latest run.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 206 `#[test]` functions in `src/`; 53 additional tests in `tests/`
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- All prior correctness and robustness fixes in place; `sliding_window` returns `IsccResult` on
    `width < 2`; `alg_simhash` validated on digest length
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
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
- `sliding_window` returns `PyResult<Vec<String>>` and raises `ValueError` on `width < 2`
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
- `sliding_window` returns `napi::Result<Vec<String>>` and throws on `width < 2`
- 66 tests: 9 in `conformance.test.mjs` + 57 in `functions.test.mjs`
- `npm test` passes all conformance vectors (CI-verified at HEAD)
- Structured results not returned — all gen functions return only the `.iscc` string field

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- `sliding_window` propagates `IsccError` as `JsError` on `width < 2`
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
- `iscc_sliding_window` propagates error via thread-local last-error and returns null on `width < 2`
- Infrastructure in place: `IsccByteBuffer`/`IsccByteBufferArray` `#[repr(C)]` types, cbindgen
    config, C test program (`tests/test_iscc.c`), thread-local last-error pattern
- 62 `#[test]` Rust unit tests including 11 streaming hasher tests
- C test program covers streaming hasher lifecycle (tests 14–17 in `test_iscc.c`)
- cbindgen generates valid C headers (CI-verified at HEAD)
- C test program compiles with gcc and runs correctly (CI-verified at HEAD)

## Java Bindings

**Status**: partially met (JNI bridge complete; Java-side absent)

- `crates/iscc-jni/` crate: `Cargo.toml` with `crate-type = ["cdylib"]`, `publish = false`,
    `iscc-lib` and `jni = "0.21"` workspace dependencies; workspace member in root `Cargo.toml`
- `crates/iscc-jni/src/lib.rs` (763 lines): **all 23 Tier 1 symbols** implemented as 29
    `extern "system"` JNI functions (streaming hashers expand to 4 JNI functions each: new, update,
    finalize, free)
- `throw_and_default` helper implemented and used consistently (51 call sites)
- Helper functions: `extract_int_array`, `extract_byte_array`, `extract_int_array_2d`,
    `extract_string_array`, `build_string_array` keep the bridge DRY
- `#[unsafe(no_mangle)]` correctly used per Rust 2024 edition requirements
- `cargo clippy -p iscc-jni -- -D warnings` passes (CI-verified at HEAD)
- Missing: Java source tree (`io.iscc.iscc_lib.IsccLib` class with native method declarations)
- Missing: Maven or Gradle build configuration
- Missing: platform-specific native library bundling (`META-INF/native/`)
- Missing: runtime native library loader class
- Missing: devcontainer JDK 17+ and Maven/Gradle
- Missing: Java CI job in `ci.yml`
- Missing: Java tests

## README

**Status**: partially met

- Rewritten as public-facing polyglot developer README (179 lines)
- ✅ CI badge; crate/PyPI/npm version badges present
- ✅ Experimental notice, tagline, Key Features (6 bullets), ISCC Architecture diagram, MainTypes
    table
- ✅ "What is the ISCC" and "What is iscc-lib" sections
- ✅ Installation: Rust, Python, Node.js, WASM sections present
- ✅ Quick Start: Rust, Python, Node.js, WASM examples
- ✅ Implementors Guide, Documentation link, Contributing, Apache-2.0 license, Maintainers
- ❌ **Key Features line reads** "Polyglot: Rust core with bindings for Python, Node.js, WASM, and C
    FFI" — Java is missing
- ❌ **Java/Maven installation section** not present
- ❌ **Java quick start code example** not present
- ❌ Maven Central badge not present

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
    [Run 22354707416](https://github.com/iscc/iscc-lib/actions/runs/22354707416))
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations
- Missing: `howto/java.md` how-to guide for Java
- Missing: Java API reference page
- Missing: Java code examples in existing tabbed code blocks (target requires Python, Rust, Java,
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
- `ci.yml` covers all 5 existing binding targets: Rust (fmt, clippy, test), Python (ruff, pytest),
    Node.js (napi build, test), WASM (wasm-pack test), C FFI (cbindgen, gcc, test)
- Latest CI run: **PASSING** —
    [Run 22354707450](https://github.com/iscc/iscc-lib/actions/runs/22354707450) — all 5 jobs
    success (Rust, Python, Node.js, WASM, C FFI)
- Latest Docs run: **PASSING** —
    [Run 22354707416](https://github.com/iscc/iscc-lib/actions/runs/22354707416) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready
- Missing: Java CI job (not applicable until Java wrapper class and tests exist)

## Next Milestone

CI is green. The JNI bridge in `crates/iscc-jni/src/lib.rs` is fully complete (23/23 Tier 1 symbols
via 29 `extern "system"` functions). The next goal is to create the Java-side of the bindings:

1. Create `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java class with
    `native` method declarations matching all 29 JNI signatures, a static `System.loadLibrary()`
    block, and idiomatic Java API
2. Create a native library loader class (`NativeLoader.java`) that extracts the platform-specific
    `.so`/`.dll`/`.dylib` from `META-INF/native/` to a temp directory and loads it
3. Add Maven `pom.xml` (or `build.gradle`) in `crates/iscc-jni/java/` with build and test config
4. Add conformance and smoke tests in Java (`src/test/java/`)
5. Add JDK 17+ to `.devcontainer/Dockerfile` and Maven/Gradle to mise.toml
6. Add Java CI job to `.github/workflows/ci.yml`
7. Update README with Java/Maven installation section and quick start example
8. Add `howto/java.md` documentation page
