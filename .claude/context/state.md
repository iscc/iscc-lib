<!-- assessed-at: 4e213a58d2d50fda3ab1070b6732f1a3d6a31bd1 -->

# Project State

## Status: IN_PROGRESS

## Phase: Extended Tier 1 API — 7 new symbols needed for iscc-sdk compatibility

Target was updated (commits `0df17ca`) to require **30 Tier 1 public symbols** (up from 23). Seven
new symbols are missing from the Rust core (`json_to_data_url`, Tier 1 `encode_component`,
`iscc_decode`, and 4 algorithm constants). Python bindings also need iscc-core drop-in extensions
(PIL pixel input, dict meta, `MT`/`ST`/`VS` enums, `core_opts`). All other infrastructure is
healthy: PR #3 merged, version bumped to 0.0.2, CI green on all 7 jobs, release workflow fixed.

## Rust Core Crate

**Status**: partially met (23/30 Tier 1 symbols)

- 23 Tier 1 public symbols at crate root: 9 `gen_*_v0` functions, 4 text utilities (`text_clean`,
    `text_remove_newlines`, `text_trim`, `text_collapse`), 4 algorithm primitives (`sliding_window`,
    `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`), `soft_hash_video_v0`, `encode_base64`,
    `iscc_decompose`, `DataHasher`, `InstanceHasher`, `conformance_selftest`
- Missing **7 new Tier 1 symbols** required by updated target:
    - `json_to_data_url` — new function; does not exist anywhere in `crates/iscc-lib/src/`
    - `encode_component` — exists as `pub fn encode_component` in `codec.rs` (Tier 2) but is NOT
        re-exported at crate root (`lib.rs` calls it via `codec::encode_component` internally)
    - `iscc_decode` — does not exist anywhere in `crates/iscc-lib/src/`
    - `META_TRIM_NAME` (128) — no `pub const` in any source file
    - `META_TRIM_DESCRIPTION` (4096) — no `pub const` in any source file
    - `IO_READ_SIZE` (4_194_304) — no `pub const` in any source file
    - `TEXT_NGRAM_SIZE` (13) — no `pub const` in any source file
- Tier 2 codec module remains Rust-only: `MainType`/`SubType`/`Version` enums, header encode/decode
- 269 total tests; `cargo clippy --workspace` clean; all conformance vectors pass (CI-verified)
- Pure Rust: zero binding dependencies (no PyO3, napi, wasm-bindgen)

## Python Bindings

**Status**: partially met (old 23-symbol target fully met; new extensions not yet started)

- 23/23 old Tier 1 symbols exposed via PyO3 in `crates/iscc-py/src/lib.rs`
- `IsccResult(dict)` base class + 9 typed subclasses (`MetaCodeResult`, `TextCodeResult`, etc.)
    implemented in `__init__.py` — dict-style and attribute-style access both work
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with file-like object support
- `__all__` exports 35 symbols (23 API + 10 result type classes + `__version__`)
- Missing **new iscc-core drop-in extensions** (per updated `specs/python-bindings.md`):
    - PIL pixel data for `gen_image_code_v0`: function signature still `pixels: bytes` only
    - dict `meta` parameter for `gen_meta_code_v0`: accepts only `str | None`
    - `encode_component` wrapper — not in `__init__.py` or `__all__`
    - `iscc_decode` wrapper — not in `__init__.py` or `__all__`
    - `MT`, `ST`, `VS` `IntEnum` classes — not present
    - `core_opts` `SimpleNamespace` — not present
    - `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE` constants — not
        present (blocked by Rust core missing them)
- 117 test functions across 5 files; all pass (CI-verified at HEAD)
- `ruff check` and `ruff format --check` pass (CI-verified at HEAD)
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered)

## Node.js Bindings

**Status**: partially met (23/30 Tier 1 symbols; missing 7 new symbols)

- 23/23 old Tier 1 symbols exported via napi-rs; 31 `#[napi]` annotations (includes struct methods)
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass; 103 tests (CI-verified)
- Missing new 7 Tier 1 symbols — depends on Rust core adding them first
- `repository` field added to `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)

## WASM Bindings

**Status**: partially met (23/30 Tier 1 symbols; missing 7 new symbols)

- 23/23 old Tier 1 symbols exported; 25 `#[wasm_bindgen]` annotations
- `DataHasher` and `InstanceHasher` as `#[wasm_bindgen]` structs; 54 tests pass (CI-verified)
- WASM release build fix in place (`wasm-opt` flags in `Cargo.toml`)
- `docs/howto/wasm.md` package name corrected to `@iscc/wasm` (was fixed in PR #3, now on main)
- Missing new 7 Tier 1 symbols — depends on Rust core adding them first
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)

## C FFI

**Status**: partially met (23/30 Tier 1 symbols; missing 7 new symbols)

- 25 exported `extern "C"` functions: 23 Tier 1 symbols + `iscc_alloc` / `iscc_dealloc`
- `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle; 62 `#[test]` Rust unit tests
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- Missing new 7 Tier 1 functions — depends on Rust core adding them first

## Java Bindings

**Status**: partially met (JNI bridge functional; Maven Central publishing absent)

- `crates/iscc-jni/` crate: 29 `extern "system"` JNI functions covering 23 Tier 1 symbols
- `IsccLib.java` (331 lines), `NativeLoader.java` (169 lines), `IsccLibTest.java` (51 tests)
- Java CI job passing (CI-verified at HEAD)
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` ✅
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested
- Missing: New 7 Tier 1 symbols — depends on Rust core adding them first

## Go Bindings

**Status**: partially met (23/30 Tier 1 symbols; missing 7 new symbols)

- `packages/go/iscc.go` (1,220 lines): `Runtime` struct + 23 Tier 1 exported symbols
- `DataHasher` / `InstanceHasher` with `UpdateFrom(ctx, io.Reader)` streaming
- `packages/go/iscc_test.go` (1,208 lines): 39 test functions; 93 total subtests; all pass
- `CGO_ENABLED=0 go test ./...` passes (CI-verified at HEAD)
- `docs/howto/go.md` complete; navigation entry in `zensical.toml` ✅
- Missing new 7 Tier 1 symbols — depends on Rust core adding them first

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
- `docs/architecture.md` and `docs/development.md` updated for JNI and Go bindings (iteration 8):
    Mermaid diagrams, workspace layout trees, crate summary tables, conformance test matrices
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
- **Latest CI runs on develop: PASSING** —
    [Run 22479950631](https://github.com/iscc/iscc-lib/actions/runs/22479950631) — all 7 jobs
    SUCCESS — triggered at HEAD `4e213a5`
- **Release workflow fixed** (commit `2f4b8f5`): crates.io OIDC token passed via
    `CARGO_REGISTRY_TOKEN`, npm provenance via `repository` field in WASM `package.json`, deprecated
    `macos-13` replaced with `macos-14` for x86_64-apple-darwin builds
- **PR #3 merged** (develop → main) — contains wasm-opt fix + `docs/howto/wasm.md` package name fix
    \+ ecosystem docs page
- **Version bumped to 0.0.2** (commit `306c5e9`): Cargo.toml, `package.json`, `pom.xml` all updated
- **`pyproject.toml` metadata** enriched (commit `53ecc6c`): description, license, authors,
    keywords, classifiers, project URLs added for PyPI presentation
- **`scripts/test_install.py`** added (461 lines): install verification protocol for PyPI,
    crates.io, npm, Go proxy; `mise run test:install` task registered
- Idempotency checks on all 4 publish jobs; version:sync and version:check tasks registered
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `build-jni` / `assemble-jar` untested end-to-end (no release tag triggered since adding)

## Next Milestone

**Implement 7 new Rust core Tier 1 symbols** (highest priority — unblocks all bindings):

1. Add `pub const` values in `crates/iscc-lib/src/lib.rs` or a `constants` module:
    `META_TRIM_NAME = 128usize`, `META_TRIM_DESCRIPTION = 4096usize`,
    `IO_READ_SIZE = 4_194_304usize`, `TEXT_NGRAM_SIZE = 13usize`
2. Promote `encode_component` from `codec.rs` to Tier 1: re-export at crate root with `u8` enum
    parameters (wrap the existing Rust-enum version)
3. Implement `iscc_decode(iscc_unit: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>` using existing
    `codec.rs` helpers
4. Implement `json_to_data_url(json: &str) -> String` using `data_encoding::BASE64`; detect
    `@context` key for `application/ld+json` media type

After Rust core: propagate to Python (`__init__.py`) with iscc-core drop-in extensions (PIL pixels,
dict meta, `MT`/`ST`/`VS` enums, `core_opts`), then to Node.js, WASM, C FFI binding crates.
