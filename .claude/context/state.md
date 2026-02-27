<!-- assessed-at: da22141 -->

# Project State

## Status: IN_PROGRESS

## Phase: Go Bindings Rewrite — Step 1 of ~5 Complete (Codec Module)

All non-Go bindings are at 30/30 Tier 1 symbols. The Go bindings rewrite is underway: the pure Go
codec module (`codec.go`) was implemented and reviewed PASS. The WASM bridge (`iscc.go`,
`iscc_ffi.wasm`, wazero dep) still exists — removal happens only after all pure Go modules are
complete. CI is green across all 7 jobs.

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
- `crates/iscc-lib/CLAUDE.md` accurate: 30-symbol Tier 1 list, correct Tier 2 list
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
- 198 tests passing across 6 files (CI-verified)
- `ruff check` and `ruff format --check` pass (CI-verified)
- `iscc-lib 0.0.2` not yet published to PyPI (0.0.1 was published; release not re-triggered)

## Node.js Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported via napi-rs; 39 `#[napi]` annotations
- 4 algorithm constants exported: `META_TRIM_NAME` (128), `META_TRIM_DESCRIPTION` (4096),
    `IO_READ_SIZE` (4194304), `TEXT_NGRAM_SIZE` (13) — verified by tests
- `iscc_decode` returns `IsccDecodeResult` object with
    `maintype`/`subtype`/`version`/`length`/`digest` fields
- `DataHasher` and `InstanceHasher` implemented; conformance vectors pass
- 124 tests (CI-verified)
- `cargo clippy -p iscc-napi --all-targets -- -D warnings` clean (CI-verified)
- `repository` field in `package.json` for npm provenance verification
- `@iscc/lib 0.0.2` not yet published to npm (awaiting release trigger)
- **Nothing missing** in Node.js bindings

## WASM Bindings

**Status**: met (30/30 Tier 1 symbols)

- All 30/30 Tier 1 symbols exported; 35 `#[wasm_bindgen]` annotations
- 4 constants exposed as getter functions with uppercase names via `js_name`:
    `META_TRIM_NAME()→128`, `META_TRIM_DESCRIPTION()→4096`, `IO_READ_SIZE()→4194304`,
    `TEXT_NGRAM_SIZE()→13`
- `iscc_decode` returns `IsccDecodeResult` struct with `getter_with_clone` for `Vec<u8>` digest
- `DataHasher` and `InstanceHasher` as `#[wasm_bindgen]` structs with `new`/`update`/`finalize`
- `conformance_selftest` gated behind `#[cfg(feature = "conformance")]`
- 69 total `#[wasm_bindgen_test]` tests; CI-verified passing
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
- `FfiDataHasher` and `FfiInstanceHasher` with complete lifecycle
- 77 `#[test]` Rust unit tests; C test program covers 23 test cases — CI-verified passing
- cbindgen generates valid C headers; C test program compiles and runs (CI-verified)
- **Nothing missing** in C FFI bindings

## Java Bindings

**Status**: met (30/30 Tier 1 symbols)

- `crates/iscc-jni/` crate: 32 `extern "system"` JNI functions covering all 30 Tier 1 symbols
- `IsccLib.java` (382 lines): all 30 Tier 1 symbols as `public static native` methods
- 4 algorithm constants as `public static final int` fields
- `IsccDecodeResult.java` (42 lines): returned by `isccDecode`
- `NativeLoader.java` (169 lines) handles platform JAR extraction
- `IsccLibTest.java` (472 lines): 9 `@TestFactory` sections + 12 `@Test` unit methods — CI-verified
- `docs/howto/java.md` complete; navigation entry in `zensical.toml` present
- `build-jni` + `assemble-jar` release jobs in `release.yml`; 5-platform matrix
- Version: `pom.xml` at `0.0.2` (synced)
- Missing: Maven Central publishing (GPG signing, Sonatype); end-to-end release untested

## Go Bindings

**Status**: not met — pure Go rewrite in progress (~1/5 modules complete)

- **Target requires**: pure Go, no WASM/wazero, no binary artifacts
- **Step 1 COMPLETE**: `packages/go/codec.go` (570 lines) — type enums (`MainType`, `SubType`,
    `Version`), varnibble header encoding/decoding, base32/base64, length/unit encode/decode,
    `EncodeComponent`, `IsccDecompose`, `IsccDecode`, `EncodeBase64`; zero external deps; 48 tests
    pass in `codec_test.go` (929 lines) — review verdict: PASS
- **Remaining**: `utils.go` (text normalization: `TextClean`, `TextCollapse`, `TextTrim`,
    `TextRemoveNewlines`), algorithms (`AlgCdcChunks`, `AlgMinhash256`, `AlgSimhash`, DCT,
    WTA-Hash), gen functions (9 `Gen*V0`), streaming hashers (`DataHasher`, `InstanceHasher`),
    conformance selftest
- `iscc.go` (1,357 lines) WASM/wazero bridge still present — coexists with pure Go modules during
    transition; will be deleted once pure Go modules cover all 30 Tier 1 symbols
- `iscc_ffi.wasm` (683KB) still committed to git; `go.mod` still has `wazero` dependency
- `.pre-commit-config.yaml` large-file threshold still raised to 1024KB (must restore to 256KB after
    binary removal)
- 46 test functions in `iscc_test.go` (1,353 lines) valid as regression suite for new implementation
    — all currently pass via WASM bridge

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
- `docs/howto/python.md` (441 lines), `docs/howto/java.md` (384 lines), `docs/howto/nodejs.md` (360
    lines), `docs/howto/wasm.md` (419 lines), `docs/howto/go.md` (462 lines), `docs/howto/rust.md`
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
    [Run 22495010193](https://github.com/iscc/iscc-lib/actions/runs/22495010193) — all 7 jobs
    SUCCESS
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Continue pure Go rewrite — implement `packages/go/utils.go` (text utilities):**

Step 2 of the Go rewrite dependency chain (codec → **text utils** → algorithms → gen functions).
Implement idiomatic Go text normalization functions:

1. `TextClean(text string) string` — Unicode NFKC normalization + whitespace collapse
2. `TextCollapse(text string) string` — collapse whitespace runs to single space
3. `TextTrim(text string, nbytes uint) string` — trim to nbytes at character boundary
4. `TextRemoveNewlines(text string) string` — replace newlines with spaces

Requires adding `golang.org/x/text/unicode/norm` to `go.mod` (first external dependency in the pure
Go rewrite). Existing `iscc_test.go` test functions for these utilities will validate against
conformance vectors. After utils, proceed with algorithms module.
