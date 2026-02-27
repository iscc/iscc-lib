<!-- assessed-at: 5df058f -->

# Project State

## Status: IN_PROGRESS

## Phase: Extended Tier 1 API — 2 new symbols remaining, then binding propagation

CID iteration 2 implemented 5 of 7 missing Tier 1 symbols in the Rust core (4 algorithm constants +
`encode_component` wrapper). The Rust core is now at 28/30 Tier 1 symbols. Still missing:
`json_to_data_url` and `iscc_decode`. No new symbols have been propagated to any binding crate yet —
all 6 bindings remain at 23/30. CI is fully green on all 7 jobs.

## Rust Core Crate

**Status**: partially met (28/30 Tier 1 symbols)

- 28 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `encode_component` (now Tier 1 wrapper with `u8` parameters), `DataHasher`,
    `InstanceHasher`, `conformance_selftest`, and 4 algorithm constants: `META_TRIM_NAME` (128),
    `META_TRIM_DESCRIPTION` (4096), `IO_READ_SIZE` (4_194_304), `TEXT_NGRAM_SIZE` (13)
- **Still missing 2 Tier 1 symbols:**
    - `json_to_data_url` — not implemented anywhere in `crates/iscc-lib/src/`
    - `iscc_decode` — not implemented anywhere in `crates/iscc-lib/src/`
- Tier 2 codec module remains Rust-only: `MainType`/`SubType`/`Version` enums, header encode/decode
- 280 total tests (227 in lib.rs + 31 + 22 doc-tests); `cargo clippy --workspace` clean; all
    conformance vectors pass (CI-verified)
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen)

## Python Bindings

**Status**: partially met (23/30 Tier 1 symbols; new symbols not yet propagated)

- 23/23 old Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- `IsccResult(dict)` base class + 9 typed subclasses (`MetaCodeResult`, `TextCodeResult`, etc.)
    implemented in `__init__.py` — dict-style and attribute-style access both work
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with file-like object support
- `__all__` exports 35 symbols (23 API + 10 result type classes + `__version__`)
- **Not yet propagated** from Rust core: `encode_component`, `META_TRIM_NAME`,
    `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`
- **Not yet implemented** (blocked by missing Rust core symbols): `iscc_decode`, `json_to_data_url`
- **Still missing iscc-core drop-in extensions** (per updated `specs/python-bindings.md`):
    - PIL pixel data for `gen_image_code_v0`: function signature still `pixels: bytes` only
    - dict `meta` parameter for `gen_meta_code_v0`: accepts only `str | None`
    - `MT`, `ST`, `VS` `IntEnum` classes — not present
    - `core_opts` `SimpleNamespace` — not present
- 117 test functions across 5 files; all pass (CI-verified at HEAD)
- `ruff check` and `ruff format --check` pass (CI-verified at HEAD)
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered)

## Node.js Bindings

**Status**: partially met (23/30 Tier 1 symbols; new symbols not yet propagated)

- 23/23 old Tier 1 symbols exported via napi-rs; 31 `#[napi]` annotations (includes struct methods)
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass; 103 tests (CI-verified)
- **Not yet propagated**: `encode_component`, 4 constants; **not yet implemented**: `iscc_decode`,
    `json_to_data_url`
- `repository` field added to `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)

## WASM Bindings

**Status**: partially met (23/30 Tier 1 symbols; new symbols not yet propagated)

- 23/23 old Tier 1 symbols exported; 25 `#[wasm_bindgen]` annotations
- `DataHasher` and `InstanceHasher` as `#[wasm_bindgen]` structs; 54 tests pass (CI-verified)
- WASM release build fix in place (`wasm-opt` flags in `Cargo.toml`)
- `docs/howto/wasm.md` package name corrected to `@iscc/wasm`
- **Not yet propagated**: `encode_component`, 4 constants; **not yet implemented**: `iscc_decode`,
    `json_to_data_url`
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)

## C FFI

**Status**: partially met (23/30 Tier 1 symbols; new symbols not yet propagated)

- 25 exported `extern "C"` functions: 23 Tier 1 symbols + `iscc_alloc` / `iscc_dealloc`
- `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle; 62 `#[test]` Rust unit tests
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- **Not yet propagated**: `encode_component`, 4 constants; **not yet implemented**: `iscc_decode`,
    `json_to_data_url`

## Java Bindings

**Status**: partially met (JNI bridge functional; Maven Central publishing absent)

- `crates/iscc-jni/` crate: 29 `extern "system"` JNI functions covering 23 Tier 1 symbols
- `IsccLib.java` (331 lines), `NativeLoader.java` (169 lines), `IsccLibTest.java` (51 tests)
- Java CI job passing (CI-verified at HEAD)
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` ✅
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- **Not yet propagated**: `encode_component`, 4 constants; **not yet implemented**: `iscc_decode`,
    `json_to_data_url`
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested

## Go Bindings

**Status**: partially met (23/30 Tier 1 symbols; new symbols not yet propagated)

- `packages/go/iscc.go` (1,220 lines): `Runtime` struct + 23 Tier 1 exported symbols
- `DataHasher` / `InstanceHasher` with `UpdateFrom(ctx, io.Reader)` streaming
- `packages/go/iscc_test.go` (1,208 lines): 39 test functions; 93 total subtests; all pass
- `CGO_ENABLED=0 go test ./...` passes (CI-verified at HEAD)
- `docs/howto/go.md` complete; navigation entry in `zensical.toml` ✅
- **Not yet propagated**: `encode_component`, 4 constants; **not yet implemented**: `iscc_decode`,
    `json_to_data_url`

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
    [Run 22480614770](https://github.com/iscc/iscc-lib/actions/runs/22480614770) — all 7 jobs
    SUCCESS — triggered at HEAD `5df058f`
- Release workflow fixed: crates.io OIDC token, npm provenance, `macos-14` for x86_64-apple-darwin
- PR #3 merged (develop → main); version bumped to 0.0.2 across all manifests
- `pyproject.toml` metadata enriched; `scripts/test_install.py` present; idempotency checks in place
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `build-jni` / `assemble-jar` untested end-to-end (no release tag triggered since adding)

## Next Milestone

**Complete remaining 2 Rust core Tier 1 symbols, then propagate all 5 new symbols to 6 binding
crates:**

1. **Implement `iscc_decode(iscc_unit: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>`** in
    `crates/iscc-lib/src/lib.rs` using existing `codec.rs` helpers (`decode_header`,
    `decode_base32`, etc.)
2. **Implement `json_to_data_url(json: &str) -> String`** using `data_encoding::BASE64`; detect
    `@context` key for `application/ld+json` media type vs `application/json`
3. **Propagate all 5 new symbols** (`encode_component`, `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`,
    `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) + 2 new implementations (`iscc_decode`, `json_to_data_url`)
    to all 6 binding crates: Python (`crates/iscc-py`), Node.js (`crates/iscc-napi`), WASM
    (`crates/iscc-wasm`), C FFI (`crates/iscc-ffi`), Java (`crates/iscc-jni`), Go (`packages/go`)
4. **Python-only iscc-core drop-in extensions**: PIL pixel data, dict `meta`, `MT`/`ST`/`VS` enums,
    `core_opts` `SimpleNamespace`
