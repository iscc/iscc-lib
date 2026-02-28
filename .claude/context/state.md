<!-- assessed-at: 37100bf117196aa0870ddde635b1f9c28b7237c7 -->

# Project State

## Status: IN_PROGRESS

## Phase: All 7 Bindings + Docs Complete — Benchmarks, CI, and Publishing Remain

All seven language bindings (Rust, Python, Node.js, WASM, C FFI, Java, Go) export the full 30/30
Tier 1 symbols and pass conformance. CI is green on all 7 jobs. The Go CI job, README, and howto
guide have been cleaned up to fully reflect the pure Go architecture — no WASM vestiges remain.
Remaining gaps are non-binding items: benchmark CI integration, Maven Central publishing
configuration, and npm/crates.io release triggers.

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
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered)

## Node.js Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported via napi-rs; 39 `#[napi]` annotations
- 4 algorithm constants exported; `iscc_decode` returns `IsccDecodeResult` object
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass
- 124 tests (CI-verified); `cargo clippy -p iscc-napi --all-targets -- -D warnings` clean
    (CI-verified)
- `repository` field in `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)
- **Nothing missing** in Node.js bindings

## WASM Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported; 35 `#[wasm_bindgen]` annotations
- 4 constants exposed as getter functions with uppercase names via `js_name`
- `iscc_decode` returns `IsccDecodeResult` struct; `DataHasher` and `InstanceHasher` fully
    implemented
- `conformance_selftest` gated behind `#[cfg(feature = "conformance")]`
- 69 total `#[wasm_bindgen_test]` tests; CI-verified passing
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)
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
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested

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
- **Nothing missing** in Go bindings
- Minor: 5 test files retain vestigial "do NOT require the WASM binary" comments (cosmetic only)

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

- **14 pages** deployed to lib.iscc.codes; all navigation sections complete
- `docs/howto/go.md` updated to reflect pure Go: removed wazero/Runtime setup section, all code
    examples use `iscc.GenMetaCodeV0(...)` returning typed `result` structs
- All 6 binding howto guides have "Codec operations" and "Constants" sections
- Site builds and deploys via GitHub Pages; latest Docs run on main: PASSING
- ISCC branding, copy-page split-button, Open Graph meta tags in place

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions + `bench_data_hasher_streaming`
- pytest-benchmark comparison files present
- Missing: CI does not run benchmarks automatically; no published benchmark results in CI artifacts

## CI/CD and Publishing

**Status**: partially met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers 7 binding targets: Rust, Python, Node.js, WASM, C FFI, Java, Go
- Go CI job is now fully clean: checkout → setup-go → `go test` → `go vet` (no Rust toolchain, no
    WASM target, no WASM build/copy steps)
- **Latest CI run on develop: PASSING** —
    [Run 22511013392](https://github.com/iscc/iscc-lib/actions/runs/22511013392) — all 7 jobs
    SUCCESS
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Merge develop → main (first stable milestone release):**

All 7 binding types are complete at 30/30 with clean CI and fully updated documentation. The pure Go
architecture cleanup is done. The `develop` branch is in its cleanest state to date — a good
candidate for a PR to `main`. Steps:

1. Create PR from `develop` → `main` via `mise run pr:main` or
    `gh pr create -B main -H develop --title "feat: complete polyglot ISCC implementation (7 bindings, 30/30)"`
2. After merge, tag `v0.0.2` on `main` to trigger coordinated publishing across all registries
3. Trigger OIDC publishing for crates.io (registry-side setup; human task)

If merging to main is deferred, the next CI priority is **benchmark CI integration**: add a `bench`
job that runs `cargo bench --no-run` (compile-only) to keep benchmarks buildable, and optionally
upload benchmark artifacts.
