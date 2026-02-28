<!-- assessed-at: fe7e381 -->

# Project State

## Status: IN_PROGRESS

## Phase: Go Bindings Rewrite — Step 5 Complete (9/9 Gen Functions), Cleanup Remaining

All non-Go bindings are at 30/30 Tier 1 symbols. The Go pure rewrite has completed all 9 gen
functions — `GenIsccCodeV0` was implemented and reviewed PASS (all 5 vectors pass). Remaining work:
implement `ConformanceSelftest` as pure Go, then remove the WASM bridge (`iscc.go`, `iscc_ffi.wasm`,
wazero dependency) and restore the 256KB large-file threshold. CI is green across all 7 jobs.

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

**Status**: not met — 9/9 gen functions done, `ConformanceSelftest` and WASM bridge cleanup remain

- **Target requires**: pure Go, no WASM/wazero, no binary artifacts
- **Steps 1–4 COMPLETE**: codec, text utils, 5 algorithm modules (CDC, MinHash, SimHash, DCT,
    WTA-Hash) — all reviewed PASS
- **Step 5 COMPLETE** — all 9 gen functions implemented and reviewed PASS:
    - `code_meta.go`: `GenMetaCodeV0` — 16/16 conformance vectors pass
    - `code_content_text.go`: `GenTextCodeV0` — 5/5 conformance vectors pass
    - `code_data.go` (90 lines): `GenDataCodeV0` + `DataHasher` streaming struct — 4/4 vectors pass
    - `code_instance.go` (67 lines): `GenInstanceCodeV0` + `InstanceHasher` streaming struct — 3/3
        vectors pass
    - `code_content_image.go` (134 lines): `GenImageCodeV0` — 3/3 vectors pass
    - `code_content_audio.go` (112 lines): `GenAudioCodeV0` — 5/5 vectors pass
    - `code_content_video.go` (61 lines): `GenVideoCodeV0` + `SoftHashVideoV0` — 3/3 vectors pass
    - `code_content_mixed.go` (92 lines): `GenMixedCodeV0` — 2/2 vectors pass
    - `code_iscc.go` (148 lines): `GenIsccCodeV0` + `IsccCodeResult` struct — 5/5 vectors pass
        (reviewed PASS 2026-02-27)
- 187 total test functions across Go files (CI-verified passing)
- **Still remaining**:
    - `ConformanceSelftest` — pure Go function validating all 46 vectors from `data.json`; only exists
        as WASM bridge method in `iscc.go` (not as standalone pure Go)
    - Cleanup: remove `iscc.go` (1,357 lines WASM bridge), `iscc_ffi.wasm` (667KB binary), wazero dep
        from `go.mod` (`github.com/tetratelabs/wazero v1.11.0`), restore `.pre-commit-config.yaml`
        large-file threshold to 256KB (currently 1024KB)

## README

**Status**: met

- Rewritten public-facing polyglot developer README (238 lines)
- All 6 language bindings mentioned; per-language install + Quick Start; all 9 `gen_*_v0` listed
- CI badge, DeepWiki badge, version badges for all registries

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- `packages/go/README.md` (150 lines): complete API tables for all 30 Tier 1 symbols
- **Nothing missing** in Per-Crate READMEs

## Documentation

**Status**: met

- **14 pages** deployed to lib.iscc.codes; all navigation sections complete
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
- **Latest CI run on develop: PASSING** —
    [Run 22508122569](https://github.com/iscc/iscc-lib/actions/runs/22508122569) — all 7 jobs
    SUCCESS
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Complete pure Go rewrite — implement `ConformanceSelftest` and remove WASM bridge (final step):**

All 9 gen functions are done and reviewed PASS (46 conformance vectors total). Implement:

1. **`ConformanceSelftest`** in `packages/go/conformance.go` — pure Go function that reads
    `data.json`, runs all 9 gen functions against all 46 vectors, and returns `(bool, error)`.
    Reference: `crates/iscc-lib/src/lib.rs` for the Rust implementation logic.
2. **Cleanup**: remove `iscc.go` (1,357 lines), `iscc_ffi.wasm` (667KB), wazero from `go.mod`;
    restore `.pre-commit-config.yaml` large-file threshold from 1024KB back to 256KB.
3. Verify `go test ./...` and `go vet ./...` still pass after cleanup; ensure CI stays green.
