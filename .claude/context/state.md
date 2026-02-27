<!-- assessed-at: ace583940417575c7dd505e8b77571b654506f81 -->

# Project State

## Status: IN_PROGRESS

## Phase: Documentation Polish & Publishing Prep

All 6 language bindings now have 30/30 Tier 1 symbols (Go completed in iteration 13). CI is fully
green across all 7 jobs. Remaining work: update Go docs to reflect 7 newly added symbols, clean up
resolved issues from issues.md, advance benchmarks and publishing pipeline.

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
- 124 tests (103 existing + 21 new covering all 7 symbols) — CI-verified at HEAD
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
- 69 total `#[wasm_bindgen_test]` tests; CI-verified passing
- WASM release build fix in place (`wasm-opt` flags in `Cargo.toml`)
- `docs/howto/wasm.md` package name corrected to `@iscc/wasm`
- `@iscc/wasm 0.0.2` not yet published to npm (awaiting release trigger)
- **Nothing missing** in WASM bindings

## C FFI

**Status**: met (30/30 Tier 1 symbols)

- 44 exported `extern "C"` functions covering all 30 Tier 1 symbols + memory management helpers
    (`iscc_alloc`, `iscc_dealloc`, `iscc_free_string`, `iscc_free_string_array`,
    `iscc_free_byte_buffer`, `iscc_free_byte_buffer_array`, `iscc_free_decode_result`,
    `iscc_last_error`)
- 4 constants exported as getter functions: `iscc_meta_trim_name()→128`,
    `iscc_meta_trim_description()→4096`, `iscc_io_read_size()→4194304`, `iscc_text_ngram_size()→13`
- 3 newly propagated functions: `iscc_json_to_data_url`, `iscc_encode_component`, `iscc_decode`
    (returns `IsccDecodeResult` struct with `ok`, `maintype`, `subtype`, `version`, `length`,
    `digest` fields); `iscc_free_decode_result` for lifecycle management
- `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle
- 77 `#[test]` Rust unit tests; C test program covers all 23 test cases including 3 new symbol tests
    and 4 constant assertions — CI-verified passing
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- **Nothing missing** in C FFI bindings

## Java Bindings

**Status**: met (30/30 Tier 1 symbols)

- `crates/iscc-jni/` crate: 32 `extern "system"` JNI functions covering all 30 Tier 1 symbols
- `IsccLib.java` (382 lines): all 30 Tier 1 symbols as `public static native` methods
- 4 algorithm constants as `public static final int` fields: `META_TRIM_NAME` (128),
    `META_TRIM_DESCRIPTION` (4096), `IO_READ_SIZE` (4_194_304), `TEXT_NGRAM_SIZE` (13)
- `IsccDecodeResult.java` (42 lines): new class with `maintype`, `subtype`, `version`, `length`,
    `digest` fields — returned by `isccDecode`
- 3 newly propagated functions: `encodeComponent`, `isccDecode` (returns `IsccDecodeResult`),
    `jsonToDataUrl` — JNI Rust wrappers at lines 514, 540, 588 of `lib.rs`
- `NativeLoader.java` (169 lines) handles platform JAR extraction
- `IsccLibTest.java` (472 lines): 9 `@TestFactory` sections + 12 `@Test` unit methods — CI-verified
    passing
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` ✅
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested

## Go Bindings

**Status**: met (30/30 Tier 1 symbols)

- `packages/go/iscc.go` (1,357 lines): `Runtime` struct + all 30 Tier 1 exported symbols
- 4 package-level constants added: `MetaTrimName = 128`, `MetaTrimDescription = 4096`,
    `IoReadSize = 4_194_304`, `TextNgramSize = 13`
- `DecodeResult` struct added with `Maintype`, `Subtype`, `Version`, `Length`, `Digest` fields
- 3 new `Runtime` methods: `JsonToDataUrl`, `EncodeComponent`, `IsccDecode` (returns
    `*DecodeResult`); WASM sret ABI handled correctly with proper cleanup on all error paths
- `DataHasher` / `InstanceHasher` with `UpdateFrom(ctx, io.Reader)` streaming
- `packages/go/iscc_test.go` (1,353 lines): 46 test functions (39 existing + 7 new for constants,
    `JsonToDataUrl` x2, `EncodeComponent`, `IsccDecode` valid/invalid, encode/decode roundtrip); all
    pass CI-verified
- `CGO_ENABLED=0 go test ./...` passes (CI-verified at HEAD `ace5839`)
- **Documentation gap**: `docs/howto/go.md` (388 lines) and `packages/go/README.md` (104 lines) do
    not cover the 7 newly added symbols; README still says "Additional utilities…are planned"
    (stale)

## README

**Status**: met

- Rewritten public-facing polyglot developer README (238 lines)
- All 6 language bindings mentioned; per-language install + Quick Start; all 9 `gen_*_v0` listed
- CI badge, DeepWiki badge, version badges for all registries

## Per-Crate READMEs

**Status**: partially met

- All 7 per-crate READMEs present: `crates/iscc-lib/README.md`, `crates/iscc-py/README.md`,
    `crates/iscc-napi/README.md`, `crates/iscc-wasm/README.md`, `crates/iscc-jni/README.md`,
    `packages/go/README.md`, `crates/iscc-ffi/README.md`
- **Gap**: `packages/go/README.md` API overview is stale — Utilities section says "Additional
    utilities (text processing, algorithm primitives, streaming hashers) are planned"; the full 30
    Tier 1 symbols (including codec functions, constants, streaming hashers, all text utilities, all
    primitives) are implemented but the README only lists the 9 `gen_*_v0` functions in the API
    table

## Documentation

**Status**: partially met

- **14 pages** deployed to lib.iscc.codes; all navigation sections complete (Tutorials, How-to
    Guides, Explanation, Reference, Benchmarks, Development) plus Ecosystem top-level page
- `docs/ecosystem.md` (100 lines): Official + Community Implementations; Contributing guide
- `docs/architecture.md` and `docs/development.md` updated for JNI and Go bindings
- ISCC branding, copy-page split-button, Open Graph meta tags, `gen_llms_full.py` in place
- All pages have `icon: lucide/...` and `description:` YAML front matter
- Site builds and deploys via GitHub Pages; latest Docs run on main: **PASSING**
- **Gap**: `docs/howto/go.md` (388 lines) was written when Go had 23/30 symbols; it has no sections
    for codec operations (`EncodeComponent`, `IsccDecode`, `IsccDecompose`, `JsonToDataUrl`,
    `EncodeBase64`) or algorithm constants (`MetaTrimName`, `MetaTrimDescription`, `IoReadSize`,
    `TextNgramSize`)

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
    [Run 22488601158](https://github.com/iscc/iscc-lib/actions/runs/22488601158) — all 7 jobs
    SUCCESS — triggered at HEAD `ace5839`
- Release workflow fixed: crates.io OIDC token, npm provenance, `macos-14` for x86_64-apple-darwin
- PR #3 merged (develop → main); version bumped to 0.0.2 across all manifests
- `pyproject.toml` metadata enriched; `scripts/test_install.py` present; idempotency checks in place
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `build-jni` / `assemble-jar` untested end-to-end (no release tag triggered since adding)

## Next Milestone

**Update Go documentation to reflect completed 30/30 symbol set:**

1. **`docs/howto/go.md`**: Add sections for codec operations (`EncodeComponent`, `IsccDecode`,
    `IsccDecompose`, `JsonToDataUrl`, `EncodeBase64`), algorithm constants block, complete text
    utilities coverage (`TextRemoveNewlines`, `TextTrim`, `TextCollapse`), and algorithm primitives
    table
2. **`packages/go/README.md`**: Replace stale "planned" text; update API Overview Utilities table to
    list the full set of available functions beyond `gen_*_v0` generators
3. **`issues.md` cleanup**: Issues #5–#8 have been implemented and GitHub issues filed — delete
    local entries (they are resolved)
