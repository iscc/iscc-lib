<!-- assessed-at: 51161d4 -->

# Project State

## Status: IN_PROGRESS

## Phase: Binding Propagation — 30/30 Tier 1 symbols in Rust core; 0/7 new symbols in bindings

CID iteration 4 implemented `json_to_data_url` as the 30th and final Tier 1 symbol in the Rust core.
All 30 target symbols now exist in `crates/iscc-lib`. However, no new symbols have been propagated
to any binding crate — all 6 bindings remain at 23/30. CI is fully green on all 7 jobs.

## Rust Core Crate

**Status**: met (30/30 Tier 1 symbols)

- All 30 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities
    (`text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives
    (`sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`,
    `encode_base64`, `iscc_decompose`, `encode_component`, `iscc_decode`, `json_to_data_url` (NEW —
    implemented this iteration), `DataHasher`, `InstanceHasher`, `conformance_selftest`, and 4
    algorithm constants: `META_TRIM_NAME` (128), `META_TRIM_DESCRIPTION` (4096), `IO_READ_SIZE`
    (4_194_304), `TEXT_NGRAM_SIZE` (13)
- `json_to_data_url` parses JSON, JCS-canonicalizes via `serde_json_canonicalizer`, detects
    `@context` for `application/ld+json` media type, encodes as base64 data URL
- 299 total tests (245 src unit tests + 31 integration tests + 22 additional integration tests + 1
    doc-test); `cargo clippy --workspace` clean; all conformance vectors pass (CI-verified)
- Tier 2 codec module remains Rust-only: `MainType`/`SubType`/`Version` enums, header encode/decode
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen)
- **Nothing missing** in Rust core

## Python Bindings

**Status**: partially met (23/30 Tier 1 symbols; 7 new symbols not yet propagated)

- 23/23 old Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- `IsccResult(dict)` base class + 9 typed subclasses (`MetaCodeResult`, `TextCodeResult`, etc.)
    implemented in `__init__.py` — dict-style and attribute-style access both work
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with file-like object support
- `__all__` exports 35 symbols (23 API + 10 result type classes + `__version__`)
- **Not yet propagated** from Rust core: `encode_component`, `iscc_decode`, `json_to_data_url`,
    `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`
- **Still missing iscc-core drop-in extensions** (per `specs/python-bindings.md`):
    - PIL pixel data for `gen_image_code_v0`: function signature still `pixels: bytes` only
    - dict `meta` parameter for `gen_meta_code_v0`: accepts only `str | None` (depends on
        `json_to_data_url` propagation)
    - `MT`, `ST`, `VS` `IntEnum` classes — not present
    - `core_opts` `SimpleNamespace` — not present
- 117 test functions across 5 files; all pass (CI-verified at HEAD)
- `ruff check` and `ruff format --check` pass (CI-verified at HEAD)
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered)

## Node.js Bindings

**Status**: partially met (23/30 Tier 1 symbols; 7 new symbols not yet propagated)

- 23/23 old Tier 1 symbols exported via napi-rs; 31 `#[napi]` annotations (includes struct methods)
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass; 103 tests (CI-verified)
- **Not yet propagated**: `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants
- `repository` field added to `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)

## WASM Bindings

**Status**: partially met (23/30 Tier 1 symbols; 7 new symbols not yet propagated)

- 23/23 old Tier 1 symbols exported; 25 `#[wasm_bindgen]` annotations
- `DataHasher` and `InstanceHasher` as `#[wasm_bindgen]` structs; 54 tests pass (CI-verified)
- WASM release build fix in place (`wasm-opt` flags in `Cargo.toml`)
- `docs/howto/wasm.md` package name corrected to `@iscc/wasm`
- **Not yet propagated**: `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)

## C FFI

**Status**: partially met (23/30 Tier 1 symbols; 7 new symbols not yet propagated)

- 25 exported `extern "C"` functions: 23 Tier 1 symbols + `iscc_alloc` / `iscc_dealloc`
- `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle; 62 `#[test]` Rust unit tests
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- **Not yet propagated**: `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants
    (constants are typically exposed as `#define` in the C header)

## Java Bindings

**Status**: partially met (JNI bridge functional; 7 new symbols not yet propagated; Maven Central
publishing absent)

- `crates/iscc-jni/` crate: 29 `extern "system"` JNI functions covering 23 Tier 1 symbols
- `IsccLib.java` (331 lines), `NativeLoader.java` (169 lines), `IsccLibTest.java` (51 tests)
- Java CI job passing (CI-verified at HEAD)
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` ✅
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- **Not yet propagated**: `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested

## Go Bindings

**Status**: partially met (23/30 Tier 1 symbols; 7 new symbols not yet propagated)

- `packages/go/iscc.go` (1,220 lines): `Runtime` struct + 23 Tier 1 exported symbols
- `DataHasher` / `InstanceHasher` with `UpdateFrom(ctx, io.Reader)` streaming
- `packages/go/iscc_test.go` (1,208 lines): 39 test functions; 93 total subtests; all pass
- `CGO_ENABLED=0 go test ./...` passes (CI-verified at HEAD)
- `docs/howto/go.md` complete; navigation entry in `zensical.toml` ✅
- **Not yet propagated**: `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants

## README

**Status**: met

- Rewritten public-facing polyglot developer README (238 lines)
- All 6 language bindings mentioned; per-language install + Quick Start; all 9 `gen_*_v0` listed
- CI badge, DeepWiki badge, version badges for all registries

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present and complete: `crates/iscc-lib/README.md`,
    `crates/iscc-py/README.md`, `crates/iscc-napi/README.md`, `crates/iscc-wasm/README.md`,
    `crates/iscc-jni/README.md`, `packages/go/README.md`, `crates/iscc-ffi/README.md`

## Documentation

**Status**: met

- **14 pages** deployed to lib.iscc.codes; all navigation sections complete (Tutorials, How-to
    Guides, Explanation, Reference, Benchmarks, Development) plus Ecosystem top-level page
- `docs/ecosystem.md` (100 lines): Official + Community Implementations; Contributing guide
- `docs/architecture.md` and `docs/development.md` updated for JNI and Go bindings
- ISCC branding, copy-page split-button, Open Graph meta tags, `gen_llms_full.py` in place
- All pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages; latest Docs run on main: **PASSING**

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions + `bench_data_hasher_streaming`
- pytest-benchmark comparison files: `benchmarks/python/bench_iscc_lib.py` and
    `benchmarks/python/bench_iscc_core.py` (101 lines each) plus `conftest.py`
- Speedup factors documented in `docs/benchmarks.md`
- Missing: CI does not run benchmarks automatically; no published benchmark results in CI artifacts

## CI/CD and Publishing

**Status**: partially met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers 7 binding targets: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js (napi
    build, test), WASM (wasm-pack test --features conformance), C FFI (cbindgen, gcc, test), Java
    (JNI build, mvn test), Go (go test, go vet)
- **Latest CI run on develop: PASSING** —
    [Run 22482230713](https://github.com/iscc/iscc-lib/actions/runs/22482230713) — all 7 jobs
    SUCCESS — triggered at HEAD `51161d4`
- Release workflow fixed: crates.io OIDC token, npm provenance, `macos-14` for x86_64-apple-darwin
- PR #3 merged (develop → main); version bumped to 0.0.2 across all manifests
- `pyproject.toml` metadata enriched; `scripts/test_install.py` present; idempotency checks in place
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `build-jni` / `assemble-jar` untested end-to-end (no release tag triggered since adding)

## Next Milestone

**Propagate all 7 new symbols to all 6 binding crates:**

1. **Python** (`crates/iscc-py`): Add `encode_component`, `iscc_decode`, `json_to_data_url` as
    `#[pyfunction]`; expose 4 constants as module-level Python constants; add tests; also implement
    dict `meta` parameter for `gen_meta_code_v0` (depends on `json_to_data_url`)
2. **Node.js** (`crates/iscc-napi`): Add 3 functions + 4 constants with `#[napi]`; update TS types;
    add tests
3. **WASM** (`crates/iscc-wasm`): Add 3 functions + 4 constants with `#[wasm_bindgen]`; add tests
4. **C FFI** (`crates/iscc-ffi`): Add `extern "C"` wrappers for 3 functions; add `#define` constants
    in cbindgen header; add C tests
5. **Java** (`crates/iscc-jni`): Add JNI functions + Java method declarations; add tests
6. **Go** (`packages/go`): Extend `iscc.go` with 3 functions + 4 constants; update `.wasm` embed;
    add tests
7. **Python-only extensions**: PIL pixel data for `gen_image_code_v0`; `MT`/`ST`/`VS` `IntEnum`
    classes; `core_opts` `SimpleNamespace`
