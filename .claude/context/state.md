<!-- assessed-at: 9f228df -->

# Project State

## Status: RELEASE_READY

## Phase: All Functional Work Complete — Ready for 0.0.3 Release

All seven language bindings (Rust, Python, Node.js, WASM, C FFI, Java, Go) export 30/30 Tier 1
symbols and pass conformance. CI is green on all 9 jobs. v0.0.2 is published to all registries
(PyPI, crates.io, npm @iscc/lib, npm @iscc/wasm). PR #10 merged to main. Java API reference page
complete. All functional requirements are met. OIDC trusted publishing configured for crates.io.
Maven Central external setup (GPG signing, Sonatype) complete.

## Rust Core Crate

**Status**: met (30/30 Tier 1 symbols)

- All 30 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities
    (`text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives
    (`sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`,
    `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`, `json_to_data_url`,
    `DataHasher`, `InstanceHasher`, `conformance_selftest`, and 4 algorithm constants:
    `META_TRIM_NAME` (128), `META_TRIM_DESCRIPTION` (4096), `IO_READ_SIZE` (4_194_304),
    `TEXT_NGRAM_SIZE` (13)
- 299 total tests (245 src unit tests + 31 integration tests + 22 additional integration tests + 1
    doc-test); `cargo clippy --workspace` clean; all conformance vectors pass (CI-verified)
- Tier 2 codec module remains Rust-only: `MainType`/`SubType`/`Version` enums, header encode/decode
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen)
- **Nothing missing** in Rust core

## Python Bindings

**Status**: met (30/30 Tier 1 symbols + all iscc-core drop-in extensions)

- All 30/30 Tier 1 symbols accessible from Python
- `__all__` has 45 entries: 4 constants + `__version__` + `MT`/`ST`/`VS` IntEnums + `core_opts` +
    `IsccResult` + 9 typed result classes + `DataHasher` + `InstanceHasher` + 27 API symbols
- `IsccResult(dict)` base class + 9 typed subclasses — dict-style and attribute-style access both
    work
- `MT` (`IntEnum`, 8 values), `ST` (`IntEnum`, 8 values), `VS` (`IntEnum`, V0=0) all exported
- `core_opts` `SimpleNamespace` with all 4 constants exported in `__all__`
- `iscc_decode` Python wrapper returns `(MT, ST, VS, length, bytes)` with IntEnum-typed values
- `gen_meta_code_v0` accepts `meta: str | dict | None`; `gen_image_code_v0` accepts multiple buffer
    types
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with file-like object support
- 198 tests passing across 6 files (CI-verified); `ruff check` and `ruff format --check` pass
    (CI-verified)
- `iscc-lib 0.0.2` published to PyPI

## Node.js Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported via napi-rs; 39 `#[napi]` annotations
- 4 algorithm constants exported; `iscc_decode` returns `IsccDecodeResult` object
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass
- 124 tests (CI-verified); `cargo clippy -p iscc-napi --all-targets -- -D warnings` clean
    (CI-verified)
- `repository` field in `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` published to npm
- **Nothing missing** in Node.js bindings

## WASM Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported; 35 `#[wasm_bindgen]` annotations
- 4 constants exposed as getter functions with uppercase names via `js_name`
- `iscc_decode` returns `IsccDecodeResult` struct; `DataHasher` and `InstanceHasher` fully
    implemented
- `conformance_selftest` gated behind `#[cfg(feature = "conformance")]`
- 69 total `#[wasm_bindgen_test]` tests; CI-verified passing
- `@iscc/wasm 0.0.2` published to npm
- **Nothing missing** in WASM bindings

## C FFI

**Status**: met (30/30 Tier 1 symbols)

- 44 exported `extern "C"` functions covering all 30 Tier 1 symbols + memory management helpers
- 4 constants exported as getter functions; `FfiDataHasher` and `FfiInstanceHasher` with complete
    lifecycle
- 77 `#[test]` Rust unit tests; C test program covers 23 test cases — CI-verified passing
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- **Nothing missing** in C FFI bindings

## Java Bindings

**Status**: met (30/30 Tier 1 symbols)

- `crates/iscc-jni/` crate: 32 `extern "system"` JNI functions covering all 30 Tier 1 symbols
- `IsccLib.java` (382 lines): all 30 Tier 1 symbols as `public static native` methods
- 4 algorithm constants as `public static final int` fields; `IsccDecodeResult.java` present
- `NativeLoader.java` (169 lines) handles platform JAR extraction
- `IsccLibTest.java` (472 lines): 9 `@TestFactory` sections + 12 `@Test` unit methods — CI-verified
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` present
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- Maven Central external setup complete (GPG signing, Sonatype); end-to-end release untested

## Go Bindings

**Status**: met — 30/30 Tier 1 symbols

- **Architecture**: Pure Go, no CGO, no WASM, no binary artifacts — target fully met
- **30/30 Tier 1 symbols**: All 9 `gen_*_v0` functions, `ConformanceSelftest`, `DataHasher`,
    `InstanceHasher`, 4 text utilities, `SlidingWindow`, `AlgMinhash256`, `AlgCdcChunks`,
    `AlgSimhash`, `SoftHashVideoV0`, `EncodeBase64`, `EncodeComponent`, `IsccDecode`,
    `IsccDecompose`, `JsonToDataUrl`, 4 constants (`MetaTrimName`, `MetaTrimDescription`,
    `IoReadSize`, `TextNgramSize`)
- 147 pure Go test functions across 18+ test files; `go test ./...` and `go vet ./...` pass
    (CI-verified)
- `go.mod` has minimal deps: `zeebo/blake3`, `golang.org/x/text`, `klauspost/cpuid` (indirect)
- All 5 vestigial "do NOT require the WASM binary" comments removed from test files
- **Nothing missing** in Go bindings

## README

**Status**: met

- Rewritten public-facing polyglot developer README (238 lines)
- All 6 language bindings mentioned; per-language install + Quick Start; all 9 `gen_*_v0` listed
- CI badge, DeepWiki badge, version badges for all registries

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- `packages/go/README.md` updated to reflect pure Go: no wazero references, package-level functions,
    `Push` → `Finalize` streaming API, no binary artifact description
- **Nothing missing** in Per-Crate READMEs

## Documentation

**Status**: met

- **16 pages** deployed to lib.iscc.codes; all navigation sections complete
- `docs/llms.txt` references all doc pages; `scripts/gen_llms_full.py` generates
    `site/llms-full.txt`
- Getting-started tutorial in tabbed multi-language format: 7 sections × 6 languages (Python, Rust,
    Node.js, Java, Go, WASM)
- Landing page Go tab: correct pure Go API (`result, _ := iscc.GenTextCodeV0(...)`)
- All 6 binding howto guides have "Codec operations" and "Constants" sections
- Site builds and deploys via GitHub Pages; latest Docs run on main: PASSING
- ISCC branding, copy-page split-button, Open Graph meta tags in place
- `docs/architecture.md`: Go correctly shown as standalone module (separate from Rust binding
    crates) with green styling and accurate prose; all stale wazero references removed
- **Reference section** (zensical.toml): Rust API, Python API, C FFI, Java API — all present and
    complete
- **C FFI API reference** (`docs/c-ffi-api.md`, 694 lines): all 44 exported `extern "C"` symbols
    documented with C type mappings, struct layouts, memory management guidance, and error handling
- **Java API reference** (`docs/java-api.md`): all 30 Tier 1 symbols documented
- **Known issue (low priority, needs human decision):**
    - Tab order inconsistency across pages: spec says "Python, Rust, Java, Node.js, WASM" (no Go),
        landing page uses "Rust, Python, ...", tutorial uses "Python, Rust, Node.js, Java, Go, WASM" —
        spec update needed to add Go; `HUMAN REVIEW REQUESTED` for canonical tab order

## Benchmarks

**Status**: met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions + `bench_data_hasher_streaming`
    (`crates/iscc-lib/benches/benchmarks.rs`)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and
    `benchmarks/python/bench_iscc_lib.py` compare Python reference vs Rust-backed bindings
- **Speedup factors published** in `docs/benchmarks.md` (117 lines): full table of Python comparison
    (1.3×–158× speedup) and Native Rust Criterion results (with throughput for streaming functions);
    page linked in navigation under "Benchmarks"
- `Bench (compile check)` job in CI verifies all 7 benchmark targets compile
    (`cargo bench --no-run`)
- **Nothing missing** in Benchmarks

## CI/CD and Publishing

**Status**: met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers **9 jobs**: Version consistency, Rust (fmt, clippy, test), Python (ruff, pytest),
    Node.js (napi build, test), WASM (wasm-pack test), Java (JNI build, mvn test), Go (go test, go
    vet), C FFI (cbindgen, gcc, test), Bench (compile check) — all 9 SUCCESS
- **version-check** job: runs `python scripts/version_sync.py --check` using only Python 3.10
- Go CI job: `CGO_ENABLED=0 go test` + `go vet` (pure Go, no Rust toolchain)
- **PR #10 merged** to main
- **v0.0.2 published** to all registries: PyPI, crates.io, npm (@iscc/lib, @iscc/wasm)
- OIDC trusted publishing configured for crates.io
- Maven Central external setup complete (GPG signing, Sonatype)
- **Nothing missing** in CI/CD and Publishing

## Next Milestone

**Ready for 0.0.3 release.** All functional work complete.

Remaining low-priority item:

1. **Tab order** — canonical language tab order needs human decision (Python-first vs Rust-first)
