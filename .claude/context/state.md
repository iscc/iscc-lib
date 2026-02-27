<!-- assessed-at: c44b63b -->

# Project State

## Status: IN_PROGRESS

## Phase: Go Bindings Rewrite — Step 5 In Progress (4/9 Gen Functions Done)

All non-Go bindings are at 30/30 Tier 1 symbols. The Go pure rewrite is progressing through step 5
(gen functions): `GenMetaCodeV0`, `GenTextCodeV0`, `GenDataCodeV0`, and `GenInstanceCodeV0` are
complete with `DataHasher`/`InstanceHasher` streaming types — all 4 reviewed PASS. Five gen
functions remain (`GenImageCodeV0`, `GenVideoCodeV0`, `GenAudioCodeV0`, `GenMixedCodeV0`,
`GenIsccCodeV0`) plus `conformance_selftest` and WASM bridge cleanup. CI is green across all 7 jobs.

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

**Status**: not met — pure Go rewrite in progress (step 5 of ~5, 4/9 gen functions done)

- **Target requires**: pure Go, no WASM/wazero, no binary artifacts
- **Steps 1–4 COMPLETE**: codec, text utils, 5 algorithm modules (CDC, MinHash, SimHash, DCT,
    WTA-Hash) — all reviewed PASS
- **Step 5 in progress** — gen functions layer:
    - **DONE** (review verdict PASS, 2026-02-27):
        - `packages/go/code_meta.go`: `GenMetaCodeV0` — 16/16 conformance vectors pass; `MetaCodeResult`
            struct; JCS canonicalization via stdlib `json.Marshal`; BLAKE3 multihash
        - `packages/go/code_content_text.go`: `GenTextCodeV0` — 5/5 conformance vectors pass;
            `TextCodeResult` struct
        - `packages/go/code_data.go` (90 lines): `GenDataCodeV0` + `DataHasher` streaming struct
            (`NewDataHasher`, `Push`, `Finalize`) — 4/4 conformance vectors pass
        - `packages/go/code_instance.go` (67 lines): `GenInstanceCodeV0` + `InstanceHasher` streaming
            struct (`NewInstanceHasher`, `Push`, `Finalize`) — 3/3 conformance vectors pass
        - `packages/go/xxh32.go` (81 lines): pure Go xxHash32 — 8 unit tests pass
        - `github.com/zeebo/blake3 v0.2.4` in `go.mod`
    - **Remaining** (5 gen functions + selftest + cleanup):
        - `GenImageCodeV0` — needs DCT (already implemented)
        - `GenVideoCodeV0` — needs WTA-Hash (already implemented)
        - `GenAudioCodeV0`, `GenMixedCodeV0`, `GenIsccCodeV0`
        - `conformance_selftest` — validates all vectors against `data.json`
        - Cleanup: remove `iscc.go` (1,357 lines WASM bridge), `iscc_ffi.wasm` (683KB binary), wazero
            dep from `go.mod`; restore `.pre-commit-config.yaml` large-file threshold to 256KB
- 182 total test functions across Go files (47 codec + 21 utils + 15 CDC + 8 MinHash + 14 SimHash +
    10 DCT + 9 WTA-Hash + 8 xxh32 + 1 meta + 1 text + 1 data + 1 instance + 46 WASM bridge);
    `go test ./...` and `go vet ./...` passing (CI-verified)
- `iscc_ffi.wasm` (683KB) still committed to git; `go.mod` still has `wazero` dependency
- `.pre-commit-config.yaml` large-file threshold still raised to 1024KB (must restore after cleanup)

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
    [Run 22505179763](https://github.com/iscc/iscc-lib/actions/runs/22505179763) — all 7 jobs
    SUCCESS
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Continue pure Go rewrite — implement `GenImageCodeV0` + `GenVideoCodeV0` (step 5, sub-step 3):**

`GenDataCodeV0`, `GenInstanceCodeV0`, `DataHasher`, and `InstanceHasher` are done and reviewed PASS.
All algorithm dependencies are now in place. Implement:

1. **`GenImageCodeV0`** — pixel data → DCT fingerprint (already implemented in `dct.go`). Needs
    `packages/go/code_content_image.go` + `ImageCodeResult` struct. Conformance vectors in
    `data.json`
2. **`GenVideoCodeV0`** — frame signatures → WTA-Hash (already implemented in `wtahash.go`). Needs
    `packages/go/code_content_video.go` + `VideoCodeResult` struct. Conformance vectors in
    `data.json`

After review PASS on both, proceed to `GenAudioCodeV0`, `GenMixedCodeV0`, `GenIsccCodeV0`,
`conformance_selftest`, and cleanup (remove WASM bridge + restore 256KB large-file threshold).
