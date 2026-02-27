<!-- assessed-at: c7c3807 -->

# Project State

## Status: IN_PROGRESS

## Phase: Symbol Propagation — 7 Tier 1 symbols to 3 remaining bindings (C FFI, Java JNI, Go)

CID iteration 10 completed WASM symbol propagation: all 30/30 Tier 1 symbols are now accessible from
WASM. Python, Node.js, and WASM bindings are fully met. C FFI, Java JNI, and Go remain at 23/30 Tier
1 symbols — the 7 symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants)
must be propagated to these 3 remaining bindings.

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
- `MT` (`IntEnum`, 8 values: META=0..FLAKE=7), `ST` (`IntEnum`, 8 values with TEXT=0 alias for
    NONE=0), `VS` (`IntEnum`, V0=0) — all exported in `__all__`
- `core_opts` `SimpleNamespace` with `meta_trim_name=128`, `meta_trim_description=4096`,
    `io_read_size=4_194_304`, `text_ngram_size=13` — exported in `__all__`
- `iscc_decode` Python wrapper returns `(MT, ST, VS, length, bytes)` with IntEnum-typed values
- `gen_meta_code_v0` accepts `meta: str | dict | None` — dict serialized to compact JSON then
    converted to data URL via `json_to_data_url`
- `gen_image_code_v0` accepts `bytes | bytearray | memoryview | Sequence[int]` — non-bytes input
    converted via `bytes()`
- `DataHasher` and `InstanceHasher` as `#[pyclass]` with file-like object support
- 198 tests passing across 6 files (CI-verified at HEAD)
- `ruff check` and `ruff format --check` pass (CI-verified at HEAD)
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered since
    v0.0.2 bump)

## Node.js Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported via napi-rs; 39 `#[napi]` annotations (includes struct methods
    and `IsccDecodeResult` object struct)
- 4 algorithm constants exported: `META_TRIM_NAME` (128), `META_TRIM_DESCRIPTION` (4096),
    `IO_READ_SIZE` (4194304), `TEXT_NGRAM_SIZE` (13) — verified by tests
- 3 newly propagated functions: `encode_component`, `iscc_decode` (returns `IsccDecodeResult` object
    with `maintype`/`subtype`/`version`/`length`/`digest` fields), `json_to_data_url`
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass
- 124 tests (103 existing + 21 new covering all 7 symbols) — CI-verified at HEAD `0d2f60c`
- `cargo clippy -p iscc-napi --all-targets -- -D warnings` clean (CI-verified)
- `repository` field in `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)
- **Nothing missing** in Node.js bindings

## WASM Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported; 35 `#[wasm_bindgen]` annotations
- 4 constants exposed as getter functions: `meta_trim_name()→128`, `meta_trim_description()→4096`,
    `io_read_size()→4194304`, `text_ngram_size()→13`
- 3 newly propagated functions: `encode_component`, `iscc_decode` (returns `IsccDecodeResult` struct
    with `getter_with_clone` for `Vec<u8>` digest), `json_to_data_url`
- `DataHasher` and `InstanceHasher` as `#[wasm_bindgen]` structs with `new`/`update`/`finalize`
- `conformance_selftest` gated behind `#[cfg(feature = "conformance")]`
- 69 total `#[wasm_bindgen_test]` tests: 60 in `tests/unit.rs` (40 pre-existing + 19 new covering
    all 7 symbols) + 9 in `tests/conformance.rs` — CI-verified passing (run 22486077314)
- WASM release build fix in place (`wasm-opt` flags in `Cargo.toml`)
- `docs/howto/wasm.md` package name corrected to `@iscc/wasm`
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)
- **Nothing missing** in WASM bindings

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
    [Run 22486077314](https://github.com/iscc/iscc-lib/actions/runs/22486077314) — all 7 jobs
    SUCCESS — triggered at HEAD `c7c3807`
- Release workflow fixed: crates.io OIDC token, npm provenance, `macos-14` for x86_64-apple-darwin
- PR #3 merged (develop → main); version bumped to 0.0.2 across all manifests
- `pyproject.toml` metadata enriched; `scripts/test_install.py` present; idempotency checks in place
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `build-jni` / `assemble-jar` untested end-to-end (no release tag triggered since adding)

## Next Milestone

**Propagate 7 new Tier 1 symbols to C FFI (highest priority — well-established pattern):**

1. **C FFI** (`crates/iscc-ffi`): Add `extern "C"` wrappers for `encode_component`, `iscc_decode`
    (returning an out-param struct or allocated string), `json_to_data_url`; expose 4 constants as
    `#define` entries in cbindgen header via `const`; add C tests — pattern mirrors Node.js
    additions in commit `caf87ef` and WASM additions in commit `ec436a2`
2. **Java** (`crates/iscc-jni`): Add JNI functions + Java method declarations; add tests
3. **Go** (`packages/go`): Extend `iscc.go` with 3 functions + 4 constants; update `.wasm` embed;
    add tests
