<!-- assessed-at: 8a40ff36f54f46b0937cd5f60bfc597132bd37bf -->

# Project State

## Status: IN_PROGRESS

## Phase: Java CI job green — native loader, Go bindings, and per-crate READMEs pending

The Java CI job (`Java (JNI build, mvn test)`) was added to `ci.yml` and passes in the latest CI run
alongside all 5 existing jobs. All 6 CI jobs are now green. Remaining gaps: native library loader
for JAR distribution, Go bindings (not started), per-crate READMEs, and Java/Go README and docs
pages.

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

**Status**: partially met (JNI bridge + Java wrapper + Maven config + conformance tests + CI job
complete; native loader/publishing/docs absent)

- `crates/iscc-jni/` crate: `Cargo.toml` with `crate-type = ["cdylib"]`, `publish = false`,
    `iscc-lib` and `jni = "0.21"` workspace dependencies; workspace member in root `Cargo.toml`
- `crates/iscc-jni/src/lib.rs` (763 lines): all 23 Tier 1 symbols implemented as 29
    `extern "system"` JNI functions (streaming hashers expand to 4 JNI functions each)
- `throw_and_default` helper implemented and used consistently at 51 call sites
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (331 lines): 29 `native` method
    declarations matching all Rust JNI bridge function signatures, Javadoc coverage, static
    `System.loadLibrary("iscc_jni")` initializer, private constructor
- `crates/iscc-jni/java/pom.xml`: Maven build config, JDK 17 target, JUnit 5 + Gson test
    dependencies, Surefire 3.5.2 plugin with `java.library.path=target/debug`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (338 lines): 9
    `@TestFactory` / `DynamicTest` methods, one per `gen_*_v0` function, covering all 46 official
    conformance vectors
- Java CI job (`Java (JNI build, mvn test)`) added to `.github/workflows/ci.yml`:
    `actions/setup-java@v4` (temurin JDK 17), `cargo build -p iscc-jni`,
    `mvn test -f crates/iscc-jni/java/pom.xml` — **PASSING** in latest CI run
- `.devcontainer/Dockerfile`: `openjdk-17-jdk-headless` and `maven` added
- `cargo clippy -p iscc-jni -- -D warnings` passes (CI-verified at HEAD)
- Missing: native library loader class (extracts platform `.so`/`.dll`/`.dylib` from
    `META-INF/native/` to temp dir at runtime)
- Missing: platform-specific native library bundling inside JAR
- Missing: README Java/Maven installation section and quick start
- Missing: `docs/howto/java.md`
- Missing: Maven Central publishing configuration

## Go Bindings

**Status**: not started

- No `packages/go/` directory exists
- No WASM WASI target built for Go consumption
- Target spec calls for WASM/wazero approach (pure Go, no cgo), Go module under `packages/go/`
- Requires: `wasm32-wasip1` build of `iscc-ffi` with `iscc_alloc`/`iscc_dealloc` helpers, pre-built
    `.wasm` embedded via `//go:embed`, idiomatic Go wrapper with `error` returns and `io.Reader`
    support, Go conformance tests, CI job, README and docs pages

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
- ❌ **Key Features** line still reads "Python, Node.js, WASM, and C FFI" — Java and Go missing
- ❌ **Java/Maven installation section** not present
- ❌ **Java quick start code example** not present
- ❌ **Go installation section** not present
- ❌ **Go quick start code example** not present
- ❌ Maven Central and Go badges not present

## Per-Crate READMEs

**Status**: not started

- Target requires a `README.md` in every publishable crate directory (`crates/iscc-lib`,
    `crates/iscc-py`, `crates/iscc-napi`, `crates/iscc-wasm`, `crates/iscc-ffi`, `crates/iscc-jni`,
    `packages/go`)
- No per-crate README files exist in any crate directory (only the root `README.md` exists)
- `Cargo.toml` `readme` field and `pyproject.toml` `readme` field not yet pointing to crate-local
    READMEs

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
    [Run 22363927531](https://github.com/iscc/iscc-lib/actions/runs/22363927531))
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button implemented: `docs/javascripts/copypage.js`
- `scripts/gen_llms_full.py` generates `site/llms-full.txt` and per-page `.md` files
- `docs/llms.txt` exists with site metadata
- Open Graph and Twitter Card social meta tags implemented via `overrides/main.html`
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations
- Missing: `howto/java.md` how-to guide for Java
- Missing: `howto/go.md` how-to guide for Go
- Missing: Java and Go code examples in existing tabbed code blocks (target requires Python, Rust,
    Java, Go, Node.js, WASM tabs)

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
- `ci.yml` covers all 6 binding targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js
    (napi build, test), WASM (wasm-pack test), C FFI (cbindgen, gcc, test), Java (JNI build, mvn
    test)
- Latest CI run: **PASSING** —
    [Run 22363927537](https://github.com/iscc/iscc-lib/actions/runs/22363927537) — all 6 jobs
    success (Rust, Python, Node.js, WASM, C FFI, Java)
- Latest Docs run: **PASSING** —
    [Run 22363927531](https://github.com/iscc/iscc-lib/actions/runs/22363927531) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: Go CI job (Go bindings not started)
- Missing: OIDC trusted publishing for crates.io and PyPI not configured (no publish step in CI)
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

CI is green on all 6 jobs (including the new Java job). The most impactful next step is adding the
native library loader class to enable JAR self-containment (platform `.so`/`.dll`/`.dylib`
extraction from `META-INF/native/` at runtime), followed by per-crate READMEs and Java docs:

1. Add native library loader class to `IsccLib.java` (extracts platform-specific `.so`/`.dll`/
    `.dylib` from `META-INF/native/` to a temp dir at runtime) — enables JAR self-containment
2. Create per-crate `README.md` files for all publishable crates (target requirement; currently none
    exist)
3. Update root README with Java/Maven installation section and quick start example
4. Add `docs/howto/java.md` documentation page
5. Begin Go bindings (`packages/go/`) — `wasm32-wasip1` WASM target for wazero consumption
