<!-- assessed-at: a908f95 -->

# Project State

## Status: IN_PROGRESS

## Phase: WASM silent-null closed — FFI video allocation and Go bindings pending

WASM `alg_cdc_chunks` silent null return is resolved: function now returns
`Result<JsValue, JsError>` and propagates serialization errors instead of swallowing them as
`JsValue::NULL`. CI is green on all 6 jobs. Remaining normal-priority issues are FFI video frame
allocation and codec header parsing; Go bindings remain unstarted.

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
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | bytearray | memoryview | BinaryIO`
    in the Python layer; `bytearray`/`memoryview` correctly converted to `bytes` before Rust FFI
- File-like stream inputs use 64 KiB chunked reads via `_DataHasher`/`_InstanceHasher` (no more
    unbounded `.read()` memory exhaustion) — `_CHUNK_SIZE = 65536` constant in `__init__.py`
- `DataHasher.update` and `InstanceHasher.update` accept `bytes | bytearray | memoryview | BinaryIO`
    with same chunked-read logic
- `sliding_window` returns `PyResult<Vec<String>>` and raises `ValueError` on `width < 2`
- 115 test functions across 5 files (`test_conformance.py`, `test_smoke.py`, `test_text_utils.py`,
    `test_algo.py`, `test_streaming.py`); 157 total pytest tests pass
- `ruff check` and `ruff format --check` clean (CI-verified at HEAD)
- `pytest` passes all conformance vectors and bytes-like input tests (CI-verified at HEAD)
- abi3-py310 wheel configuration in place
- `ty` type checking configured
- OIDC trusted publishing not yet configured
- **Open issues** \[low\]: missing `__version__`; module docstring references wrong package name

## Node.js Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via napi-rs in `crates/iscc-napi/src/lib.rs`
- `DataHasher` and `InstanceHasher` implemented as `#[napi(js_name)]` structs with constructor/
    update/finalize methods
- `sliding_window` returns `napi::Result<Vec<String>>` and throws on `width < 2`
- 103 tests: 9 in `conformance.test.mjs` + 57 in `functions.test.mjs` (103 as counted by node:test
    runner including sub-tests; CI-verified at HEAD)
- `npm test` passes all conformance vectors (CI-verified at HEAD)
- Version skew resolved: `index.js` regenerated with `0.0.1` matching `package.json`
- npm packaging fixed: `"files"` allowlist in `package.json` ensures `index.js`, `index.d.ts`,
    `*.node`, `README.md` are included in published tarball
- `alg_cdc_chunks` clone eliminated: `.into_iter().map(Buffer::from)` avoids per-chunk allocation
- Structured results not returned — all gen functions return only the `.iscc` string field

## WASM Bindings

**Status**: met

- 23/23 Tier 1 symbols exported via wasm-bindgen in `crates/iscc-wasm/src/lib.rs`
- `DataHasher` and `InstanceHasher` added as `#[wasm_bindgen]` structs
- `sliding_window` propagates `IsccError` as `JsError` on `width < 2`
- 54 tests: 9 in `conformance.rs` + 45 in `unit.rs` (all run via wasm-pack test --node)
- `wasm-pack test --node crates/iscc-wasm` passes all 54 tests (CI-verified at HEAD)
- `alg_cdc_chunks` fixed: now returns `Result<JsValue, JsError>` and propagates serialization errors
    via `.map_err(|e| JsError::new(&e.to_string()))` — no more silent null on failure
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported
- **Open issues** \[low\]: `conformance_selftest` exported without feature gate increases binary
    size; stale CLAUDE.md says DataHasher/InstanceHasher not yet bound

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
- **Open issues** \[normal\]: video functions allocate/copy every frame signature

## Java Bindings

**Status**: partially met (JNI bridge + Java wrapper + Maven config + conformance tests + CI job
complete; native loader/publishing/docs absent)

- `crates/iscc-jni/` crate: `Cargo.toml` with `crate-type = ["cdylib"]`, `publish = false`,
    `iscc-lib` and `jni = "0.21"` workspace dependencies; workspace member in root `Cargo.toml`
- `crates/iscc-jni/src/lib.rs` (866 lines): all 23 Tier 1 symbols implemented as 29
    `extern "system"` JNI functions (streaming hashers expand to 4 JNI functions each)
- `throw_and_default` helper implemented and used consistently at 72 call sites; zero `unwrap()`
    calls — all error paths throw Java exceptions instead of aborting the JVM
- **Negative `jint` validation**: 3 guards added — `textTrim` (`nbytes < 0`), `slidingWindow`
    (`width < 0`), `algCdcChunks` (`avg_chunk_size < 0`) — all throw `IllegalArgumentException`
- **Local reference frame safety**: `push_local_frame(16)`/`pop_local_frame` added to all 5 loops —
    prevents JVM local reference table overflow on large arrays
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (331 lines): 29 `native` method
    declarations, Javadoc coverage, static `System.loadLibrary("iscc_jni")` initializer
- `crates/iscc-jni/java/pom.xml`: Maven build config, JDK 17 target, JUnit 5 + Gson test
    dependencies, Surefire 3.5.2 plugin with `java.library.path=target/debug`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (362 lines): 9
    `@TestFactory` / `DynamicTest` conformance methods (46 vectors) + 3 `@Test` negative-value
    validation methods — 49 total tests, all passing
- Java CI job (`Java (JNI build, mvn test)`) in `.github/workflows/ci.yml`: passing on all runs
- `.devcontainer/Dockerfile`: `openjdk-17-jdk-headless` and `maven` added
- `cargo clippy -p iscc-jni -- -D warnings` passes (CI-verified at HEAD)
- Missing: native library loader class (extracts platform `.so`/`.dll`/`.dylib` from
    `META-INF/native/` to temp dir at runtime)
- Missing: platform-specific native library bundling inside JAR
- Missing: `docs/howto/java.md`
- Missing: Maven Central publishing configuration
- **Open issues** \[low\]: all exceptions map to `IllegalArgumentException` (state errors should use
    `IllegalStateException`)

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

- Rewritten as public-facing polyglot developer README (200 lines)
- ✅ CI badge; crate/PyPI/npm version badges present
- ✅ Experimental notice, tagline, Key Features (6 bullets), ISCC Architecture diagram, MainTypes
    table
- ✅ "What is the ISCC" and "What is iscc-lib" sections
- ✅ **Key Features** line reads "Python, Java, Node.js, WASM, and C FFI" — Java present
- ✅ Installation: Rust, Python, Node.js, Java, WASM sections present
- ✅ Quick Start: Rust, Python, Node.js, Java, WASM examples
- ✅ Implementors Guide, Documentation link, Contributing, Apache-2.0 license, Maintainers
- ❌ **"What is iscc-lib" body text** still says "Python, Node.js, WebAssembly, and C" — Java not
    mentioned (line 47)
- ❌ **Go installation section** not present
- ❌ **Go quick start code example** not present
- ❌ Maven Central and Go badges not present

## Per-Crate READMEs

**Status**: partially met (5 of 6 publishable crates done; only iscc-ffi remains, lower priority)

- ✅ `crates/iscc-lib/README.md` — complete
- ✅ `crates/iscc-py/README.md` — complete
- ✅ `crates/iscc-napi/README.md` — complete
- ✅ `crates/iscc-wasm/README.md` — complete
- ✅ `crates/iscc-jni/README.md` — complete
- ❌ `crates/iscc-ffi/README.md` — not created (not published to a registry; lower priority)
- ❌ `packages/go/README.md` — not applicable (Go bindings not started)

## Documentation

**Status**: partially met

- 11 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md`, `howto/nodejs.md`, `howto/wasm.md`, `howto/rust.md`,
    `development.md`, `tutorials/getting-started.md`
- Navigation in `zensical.toml` has: Tutorials (Getting Started), How-to Guides (Rust, Python,
    Node.js, WebAssembly), Explanation (Architecture), Reference (Rust API, Python API), Benchmarks,
    Development
- All 11 pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages (Docs CI: PASSING)
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button, `scripts/gen_llms_full.py`, Open Graph meta tags all in place
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations
- Missing: `howto/java.md` how-to guide for Java
- Missing: `howto/go.md` how-to guide for Go
- Missing: Java and Go code examples in existing tabbed code blocks

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
    [Run 22372060652](https://github.com/iscc/iscc-lib/actions/runs/22372060652) — all 6 jobs
    success (Rust, Python, Node.js, WASM, C FFI, Java)
- Latest Docs run: **PASSING** —
    [Run 22372060644](https://github.com/iscc/iscc-lib/actions/runs/22372060644) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- Missing: Go CI job (Go bindings not started)
- Missing: OIDC trusted publishing for crates.io and PyPI not configured
- Missing: npm publishing pipeline not fully wired
- Missing: version sync automation across workspace not verified as release-ready

## Next Milestone

CI is green on all 6 jobs. WASM silent-null issue is closed. Recommended next work (in priority
order):

1. **FFI video allocation** [normal] — `iscc_gen_video_code_v0` and `iscc_soft_hash_video_v0` in
    `crates/iscc-ffi/src/lib.rs` allocate/copy every frame signature via `.to_vec()`; consider
    changing the `iscc_lib` video API to accept `&[&[i32]]` to avoid per-frame copies
2. **Codec header parsing** [normal] — `bytes_to_bits` in `crates/iscc-lib/src/codec.rs:121-127`
    expands bytes to `Vec<bool>` on every codec operation; replace with bit-level indexing
3. **DataHasher allocation** [normal] — `streaming.rs:88-93` copies input data on every `update()`
    call; rework the tail-buffering logic to avoid redundant allocations
4. **Go bindings** — largest unstarted feature; requires WASM/wazero approach, `wasm32-wasip1` build
    of `iscc-ffi`, Go module under `packages/go/`, conformance tests, CI job
