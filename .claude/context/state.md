<!-- assessed-at: 3d59788fd59057b02a6e7815fb3ecf786eb317f3 -->

# Project State

## Status: IN_PROGRESS

## Phase: Go Bindings Rewrite (Pure Go)

All bindings except Go are at 30/30 Tier 1 symbols against the target. The target.md was updated in
HEAD to require a **pure Go implementation** (no WASM/wazero bridge, no committed binary artifacts).
The current Go implementation still uses the wazero bridge with a 683KB `.wasm` binary committed to
git — this is now a **[critical]** target mismatch. CI is green across all 7 jobs.

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

**Status**: not met (target rewritten; current implementation is WASM/wazero bridge)

- **Target changed**: `target.md` was updated in HEAD to require a **pure Go implementation** — no
    WASM, no wazero, no committed binary artifacts
- Current implementation: WASM/wazero bridge (`packages/go/iscc.go`, 1,357 lines) with 683KB
    `iscc_ffi.wasm` committed to git via `//go:embed`
- `go.mod` still depends on `github.com/tetratelabs/wazero v1.11.0`
- 46 test functions in `iscc_test.go` (1,353 lines) pass via the WASM bridge — tests exist and are
    valid, but the bridge approach must be replaced
- `.pre-commit-config.yaml` threshold raised to 1024KB globally to accommodate the binary — must be
    restored to 256KB after removing the binary
- `[critical]` issue filed in `issues.md`: rewrite as pure Go with Go-native algorithm
    implementations (CDC, MinHash, SimHash, DCT, WTA-Hash), using `zeebo/blake3`, `cespare/xxhash`,
    `golang.org/x/text` — estimated ~6,300 lines of Rust to port

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
    [Run 22493418952](https://github.com/iscc/iscc-lib/actions/runs/22493418952) — all 7 jobs
    SUCCESS — triggered at commit `e225748`
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)
- Missing: `issues.md` entries #5–#8 are resolved but never deleted (stale noise for agents)

## Next Milestone

**[critical] Rewrite Go bindings as pure Go — top priority:**

The `target.md` now requires a pure Go implementation with no WASM/wazero bridge and no binary
artifacts in git. The current implementation is entirely WASM-based. The rewrite must:

1. **Remove WASM infrastructure**: delete `packages/go/iscc_ffi.wasm`, re-add `packages/go/*.wasm`
    to `.gitignore`, restore `check-added-large-files` to `--maxkb=256`, remove `wazero` from
    `go.mod`
2. **Implement pure Go algorithms**: port CDC, MinHash, SimHash, DCT, WTA-Hash, codec, and text
    utilities from Rust into Go using `zeebo/blake3`, `cespare/xxhash/v2`, `golang.org/x/text`
3. **Preserve 30/30 Tier 1 API surface**: same function signatures and Go naming conventions;
    existing `iscc_test.go` (46 test functions) should pass against the new implementation
4. **Clean up issues.md**: delete the 4 stale resolved entries (#5–#8: gen_meta_code_v0 dict,
    encode_component, iscc_decode, constants) — all implementations confirmed complete
