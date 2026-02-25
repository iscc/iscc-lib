<!-- assessed-at: 69bfb2d49c0fcac14eb60281fcf3d4be7713f370 -->

# Project State

## Status: IN_PROGRESS

## Phase: Release readiness — [normal] version sync tooling + [low] housekeeping remain

All core bindings are complete and CI is green on all 7 jobs. The last `[critical]` issue (release
idempotency) was resolved in iteration 25 — all 4 publish jobs now have pre-publish
version-existence checks that skip gracefully when the version already exists on the target
registry. One `[normal]` issue (version sync tooling) and three `[low]` issues remain.

## Rust Core Crate

**Status**: met

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Tier 2 codec module (`codec.rs`) with `MainType`/`SubType`/`Version` enums and all encode/decode
    helpers — correctly Rust-only, not bound to foreign languages
- 216 `#[test]` functions in `src/`; 53 additional tests in `tests/`; 269 total
- `decode_header` and `decode_varnibble_from_bytes` use direct bitwise extraction from `&[u8]` — no
    `Vec<bool>` allocation in any production decode path; `bytes_to_bits` and `bits_to_u32` are
    `#[cfg(test)]`-gated
- `DataHasher::update` uses persistent `buf: Vec<u8>` reused across calls — no `to_vec()` or
    `.concat()` allocations on any update path; tail shift done via `copy_within` + `truncate`
- `bench_data_hasher_streaming` Criterion benchmark added: 1 MB input, 64 KiB chunks, ~1.0 GiB/s
- `soft_hash_video_v0` and `gen_video_code_v0` now generic: `S: AsRef<[i32]> + Ord` — accepts both
    `&[Vec<i32>]` (owned) and `&[&[i32]]` (borrowed); backward-compatible with all binding crates
- `alg_dct`: validation now strictly enforces `n.is_power_of_two()` (lengths like 6, 10, 12 are now
    rejected); error message updated; 4 unit tests covering non-power-of-two cases
- `alg_wtahash`: return type changed from `Vec<u8>` to `IsccResult<Vec<u8>>`; validates
    `vec.len() >= 380` and `bits > 0 && bits % 8 == 0 && bits <= 256`; 4 unit tests covering short
    input and invalid bit values; `soft_hash_video_v0` propagates error directly
- All conformance vectors from `data.json` pass for every `gen_*_v0` function (CI-verified at HEAD)
- All prior correctness and robustness fixes in place; `sliding_window` returns `IsccResult` on
    `width < 2`; `alg_simhash` validated on digest length
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen in `iscc-lib`)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (CI-verified at HEAD)
- Note: target.md header says "22 public symbols" but the enumerated list totals 23; the crate
    implements 23
- **Open issues**: none

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
- `__version__ = version("iscc-lib")` via `importlib.metadata` — present in `__init__.py` and
    included in `__all__`
- Module docstring in `crates/iscc-py/src/lib.rs` corrected to `iscc_lib._lowlevel` (was
    `iscc._lowlevel`)
- 117 test functions across 5 files (`test_conformance.py`, `test_smoke.py`, `test_text_utils.py`,
    `test_algo.py`, `test_streaming.py`); 159 total pytest tests (2 new `__version__` tests added)
- `ruff check` and `ruff format --check` clean (CI-verified at HEAD)
- `pytest` passes all conformance vectors and bytes-like input tests (CI-verified at HEAD)
- abi3-py310 wheel configuration in place
- `ty` type checking configured
- OIDC trusted publishing not yet configured
- **Open issues**: none

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
- `wasm-pack test --node crates/iscc-wasm --features conformance` passes all 54 tests (CI-verified
    at HEAD)
- `alg_cdc_chunks` fixed: now returns `Result<JsValue, JsError>` and propagates serialization errors
    via `.map_err(|e| JsError::new(&e.to_string()))` — no more silent null on failure
- Structured results not returned — gen functions return only the `.iscc` string field
- Browser and Node.js build targets supported
- `conformance_selftest` is now gated behind `#[cfg(feature = "conformance")]` — production builds
    omit the function and embedded test vectors; `Cargo.toml` has `[features] conformance = []`; CI
    tests with `--features conformance`
- **Open issues** \[low\]: stale CLAUDE.md says DataHasher/InstanceHasher not yet bound

## C FFI

**Status**: met

- 25 exported `extern "C"` functions in `crates/iscc-ffi/src/lib.rs` (1,934 lines): 23 Tier 1
    symbols as FFI wrappers + `iscc_alloc` and `iscc_dealloc` memory-management helpers
- `iscc_alloc(size: usize) -> *mut u8` — allocates WASM-side memory for host use; handles
    `size == 0` by returning a dangling non-null pointer
- `iscc_dealloc(ptr: *mut u8, size: usize)` — frees memory previously allocated by `iscc_alloc`;
    no-op on null or zero-size; `iscc-ffi` now compiles to `wasm32-wasip1` (~10.5 MB debug binary)
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
- **Video frame allocation eliminated**: `iscc_gen_video_code_v0` and `iscc_soft_hash_video_v0` now
    pass `Vec<&[i32]>` (zero-copy borrows) into the Rust core; no `to_vec()` call remains in the
    video path; only 1 `.to_vec()` remains in the entire iscc-ffi crate (in `alg_cdc_chunks`)
- **Open issues**: none

## Java Bindings

**Status**: partially met (JNI bridge + Java wrapper + NativeLoader + Maven config + conformance
tests + CI job + how-to guide complete; platform native bundling inside JAR and Maven Central
publishing absent)

- `crates/iscc-jni/` crate: `Cargo.toml` with `crate-type = ["cdylib"]`, `publish = false`,
    `iscc-lib` and `jni = "0.21"` workspace dependencies; workspace member in root `Cargo.toml`
- `crates/iscc-jni/src/lib.rs` (866 lines): all 23 Tier 1 symbols implemented as 29
    `extern "system"` JNI functions (streaming hashers expand to 4 JNI functions each)
- `throw_and_default` helper implemented and used consistently at 72 call sites; zero `unwrap()`
    calls — all error paths throw Java exceptions instead of aborting the JVM
- Negative `jint` validation: 3 guards — `textTrim`, `slidingWindow`, `algCdcChunks` — throw
    `IllegalArgumentException`
- Local reference frame safety: `push_local_frame(16)`/`pop_local_frame` in all 5 array loops
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (331 lines): 29 `native` method
    declarations, Javadoc coverage, static initializer now delegates to `NativeLoader.load()`
- **`NativeLoader.java`** (169 lines): detects OS (`linux`/`macos`/`windows`) and arch
    (`x86_64`/`aarch64`), extracts `META-INF/native/{os}-{arch}/{libname}` from JAR to temp dir,
    falls back to `System.loadLibrary("iscc_jni")`; thread-safe via `synchronized` + `volatile`
    guard; package-private helpers (`detectOs`, `detectArch`, `libraryFileName`) are testable
    without reflection; extraction path currently inactive (no native binaries bundled yet)
- `crates/iscc-jni/java/pom.xml`: Maven build config, JDK 17 target, JUnit 5 + Gson test
    dependencies, Surefire 3.5.2 plugin with `java.library.path=target/debug`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (362 lines): 9
    `@TestFactory` / `DynamicTest` conformance methods (46 vectors) + 3 `@Test` negative-value
    validation methods — 49 total tests, all passing
- Java CI job (`Java (JNI build, mvn test)`) in `.github/workflows/ci.yml`: passing on all runs
- `.devcontainer/Dockerfile`: `openjdk-17-jdk-headless` and `maven` added
- `cargo clippy -p iscc-jni -- -D warnings` passes (CI-verified at HEAD)
- `docs/howto/java.md` (319 lines): complete
- Java entry in `zensical.toml` How-to Guides navigation: `{ "Java" = "howto/java.md" }` ✅
- Missing: platform-specific native library bundling inside JAR (`META-INF/native/` population
    requires multi-platform CI matrix to produce `.so`/`.dll`/`.dylib` per target)
- Missing: Maven Central publishing configuration (sonatype staging plugin, POM metadata, GPG
    signing, release.yml wiring)
- **Open issues** \[low\]: all exceptions map to `IllegalArgumentException` (state errors should use
    `IllegalStateException`)

## Go Bindings

**Status**: partially met (23/23 Tier 1 symbols + 35 test functions + Go CI job passing + README
done + howto/go.md done; io.Reader streaming interface absent)

- `packages/go/go.mod` — module `github.com/iscc/iscc-lib/packages/go`, Go 1.24.0, wazero v1.11.0
- `packages/go/iscc.go` (1,165 lines): `Runtime` struct with `NewRuntime`/`Close` plus internal
    memory helpers and 23 Tier 1 exported symbols: all 9 `Gen*CodeV0` wrappers, 4 text utilities
    (`TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`), `SlidingWindow`,
    `IsccDecompose`, `EncodeBase64`, `ConformanceSelftest`, `AlgSimhash`, `AlgMinhash256`,
    `AlgCdcChunks`, `SoftHashVideoV0`, `GenIsccCodeV0`, `NewDataHasher` → `DataHasher` type,
    `NewInstanceHasher` → `InstanceHasher` type
- `DataHasher` / `InstanceHasher` structs with `New*/Update/Finalize/Close` lifecycle methods
    wrapping `FfiDataHasher`/`FfiInstanceHasher` WASM opaque pointers via `writeBytes`/`lastError`
    pattern; finalize-once semantics enforced
- `packages/go/iscc_test.go` (1,069 lines): 36 function declarations including TestMain + 35 actual
    test functions covering all 46 conformance vectors, 8 streaming hasher tests, error paths, and
    edge cases
- `TestMain` skips gracefully if `iscc_ffi.wasm` is not present (binary is gitignored)
- `CGO_ENABLED=0 go test ./...` passes all 35 tests (CI-verified at HEAD)
- `packages/go/*.wasm` added to `.gitignore`
- **Go CI job** (`Go (go test, go vet)`) in `.github/workflows/ci.yml` — passes (CI-verified at
    HEAD)
- `packages/go/README.md` (104 lines): complete
- `docs/howto/go.md` (388 lines): complete
- Go entry in `zensical.toml` How-to Guides navigation: `{ "Go" = "howto/go.md" }` ✅
- Missing: `io.Reader` interface for `Update` methods (architecture describes it; current
    implementation accepts `[]byte` only — callers must chunk themselves)
- Note: target.md `verified-when` criteria do not explicitly require `io.Reader`; the gap is in the
    architecture description

## README

**Status**: met

- ✅ Rewritten as public-facing polyglot developer README (238 lines)
- ✅ CI badge; DeepWiki badge added (iteration 22); Crate (crates.io), PyPI, npm, and Go Reference
    (`pkg.go.dev`) version badges present
- ✅ Experimental notice, tagline, Key Features (6 bullets with "Python, Java, Go, Node.js, WASM, and
    C FFI")
- ✅ ISCC Architecture diagram and MainTypes table
- ✅ "What is the ISCC" section
- ✅ "What is iscc-lib" body text — "Python, Java, Go, Node.js, WebAssembly, and C" (fixed)
- ✅ Installation: Rust, Python, Node.js, Java, Go, WASM sections all present
- ✅ Quick Start: Rust, Python, Node.js, Java, Go, WASM examples all present
- ✅ Implementors Guide with all 9 `gen_*_v0` entry points listed
- ✅ Documentation link to `lib.iscc.codes`
- ✅ Contributing, Apache-2.0 license, Maintainers
- ✅ No development workflow content (CID loop, dev container, pre-commit hooks absent)
- Maven Central badge not added (Java not yet published to Maven Central; not blocking)

## Per-Crate READMEs

**Status**: partially met (6 of 6 publishable crates/packages done; iscc-ffi not published
separately)

- ✅ `crates/iscc-lib/README.md` — complete
- ✅ `crates/iscc-py/README.md` — complete
- ✅ `crates/iscc-napi/README.md` — complete
- ✅ `crates/iscc-wasm/README.md` — complete
- ✅ `crates/iscc-jni/README.md` — complete
- ✅ `packages/go/README.md` — complete
- ❌ `crates/iscc-ffi/README.md` — not created (not published to a registry; lower priority)

## Documentation

**Status**: met

- 13 pages deployed to lib.iscc.codes: `index.md`, `architecture.md`, `rust-api.md`, `api.md`,
    `benchmarks.md`, `howto/python.md`, `howto/nodejs.md`, `howto/wasm.md`, `howto/rust.md`,
    `howto/go.md`, `howto/java.md`, `development.md`, `tutorials/getting-started.md`
- Navigation in `zensical.toml` has: Tutorials (Getting Started), How-to Guides (Rust, Python,
    Node.js, WebAssembly, Go, Java), Explanation (Architecture), Reference (Rust API, Python API),
    Benchmarks, Development — all entries present ✅
- All pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages (Docs CI: PASSING —
    [Run 22390109757](https://github.com/iscc/iscc-lib/actions/runs/22390109757))
- ISCC branding in place: `docs/stylesheets/extra.css`, logo, favicon, dark mode inversion
- Copy-page split-button (`docs/javascripts/copypage.js`), `scripts/gen_llms_full.py`, Open Graph
    meta tags all in place
- ✅ `docs/CNAME` contains `lib.iscc.codes`
- ✅ `docs/includes/abbreviations.md` with 19 ISCC-specific abbreviations
- ✅ `docs/howto/java.md` (319 lines): complete
- ✅ `docs/howto/go.md` (388 lines): complete
- ✅ `docs/development.md`: covers dev container setup, CID workflow, quality gates, project
    structure
- Note: `docs/index.md` quick-start tabs show only Rust and Python (not all 6 languages); not
    flagged as blocking

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions in
    `crates/iscc-lib/benches/benchmarks.rs`
- `bench_data_hasher_streaming` Criterion benchmark added: 1 MB input, 64 KiB chunks, ~1.0 GiB/s
- pytest-benchmark comparison files exist: `benchmarks/python/bench_iscc_lib.py` and
    `benchmarks/python/bench_iscc_core.py` (101 lines each) plus `conftest.py`
- Speedup factors documented in `docs/benchmarks.md`
- Missing: CI does not run benchmarks automatically; no published benchmark results in CI artifacts

## CI/CD and Publishing

**Status**: partially met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers 7 binding targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js (napi
    build, test), WASM (wasm-pack test --features conformance), C FFI (cbindgen, gcc, test), Java
    (JNI build, mvn test), Go (go test, go vet)
- `ci.yml` triggers on push to `main` and `develop` branches (and PRs to `main`) — added in
    iteration 25
- Latest CI run: **PASSING** —
    [Run 22391282792](https://github.com/iscc/iscc-lib/actions/runs/22391282792) — all 7 jobs
    success (Rust, Python, Node.js, WASM, C FFI, Java, Go)
- Latest Docs run: **PASSING** —
    [Run 22390109757](https://github.com/iscc/iscc-lib/actions/runs/22390109757) — build + deploy
    success
- All local commits are pushed; remote HEAD matches local HEAD
- ✅ **Resolved (iteration 24)**: `release.yml` `workflow_dispatch` now has `inputs:` block with
    three boolean checkboxes (`crates-io`, `pypi`, `npm`) and `if:` conditions on all 8 jobs
- ✅ **Resolved (iteration 25)**: Idempotency checks — all 4 publish jobs now have pre-publish
    version-existence checks: crates.io uses `cargo info iscc-lib`, PyPI uses PyPI JSON API
    (`https://pypi.org/pypi/iscc-lib/$VERSION/json`), npm lib/wasm use
    `npm view "@iscc/lib@$VERSION"` / `npm view "@iscc/wasm@$VERSION"`; each sets `skip=true/false`
    output; all publish/auth/test steps conditioned on `steps.check.outputs.skip != 'true'`
- **[normal]** Missing: `mise run version:sync` / `mise run version:check` tooling —
    `scripts/version_sync.py` not yet created; `package.json` and `pom.xml` must be updated by hand
    before version bumps; spec in `.claude/context/specs/ci-cd.md#sync-tooling`
- Missing: OIDC trusted publishing for crates.io and PyPI not yet configured in registry settings
    (workflow code is correct but registry-side trusted publisher setup is outside CI scope)
- Missing: npm publishing pipeline not fully wired
- Missing: Java platform native bundling in CI matrix (needed to populate `META-INF/native/`)
- Missing: Maven Central publishing configuration

## Next Milestone

CI is green on all 7 jobs and all `[critical]` blockers are resolved. Recommended priority order:

1. **[normal] Version sync tooling** — create `scripts/version_sync.py` and add
    `mise run  version:sync` / `mise run version:check` tasks to `mise.toml`; cross-platform
    Python, stdlib only; updates `crates/iscc-napi/package.json` and `crates/iscc-jni/java/pom.xml`
    from root `Cargo.toml` workspace version; spec in `.claude/context/specs/ci-cd.md#sync-tooling`
2. **Low-priority code quality fixes** (any order): iscc-wasm stale CLAUDE.md update, iscc-jni
    `IllegalStateException` for state errors, TypeScript port evaluation
3. **`crates/iscc-ffi/README.md`** — completes the per-crate README set
