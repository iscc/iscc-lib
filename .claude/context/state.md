<!-- assessed-at: bdbc92f -->

# Project State

## Status: IN_PROGRESS

## Phase: Go Bindings Rewrite — Step 4 of ~5 Complete (All 7 Algorithm Modules Done)

All non-Go bindings are at 30/30 Tier 1 symbols. The Go pure rewrite has progressed to step 4: all 7
algorithm modules are now complete — codec (`codec.go`), text utilities (`utils.go`), CDC
(`cdc.go`), MinHash (`minhash.go`), SimHash (`simhash.go`), DCT (`dct.go`), and WTA-Hash
(`wtahash.go`) — all reviewed PASS. Step 5 remains: 9 `gen_*_v0` functions, `DataHasher` /
`InstanceHasher` streaming, and conformance selftest, followed by WASM bridge removal. CI is green
across all 7 jobs.

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

**Status**: not met — pure Go rewrite in progress (~4/5 steps complete)

- **Target requires**: pure Go, no WASM/wazero, no binary artifacts
- **Step 1 COMPLETE**: `packages/go/codec.go` (570 lines) — type enums, varnibble header
    encode/decode, base32/base64, `EncodeComponent`, `IsccDecompose`, `IsccDecode`, `EncodeBase64`;
    47 tests in `codec_test.go`
- **Step 2 COMPLETE**: `packages/go/utils.go` (130 lines) — 4 pure Go text utilities using
    `golang.org/x/text`; 21 tests in `utils_test.go`
- **Step 3 COMPLETE**: Three algorithm modules — review verdict: PASS
    - `packages/go/cdc.go` (129 lines): `AlgCdcChunks` + 2 private helpers; 15 tests in `cdc_test.go`
    - `packages/go/minhash.go` (205 lines): `AlgMinhash256` + 2 private helpers; 8 tests in
        `minhash_test.go`
    - `packages/go/simhash.go` (86 lines): `AlgSimhash`, `SlidingWindow`; 14 tests in
        `simhash_test.go`
- **Step 4 COMPLETE**: Two remaining algorithm modules — review verdict: PASS
    - `packages/go/dct.go` (52 lines): `algDct` (unexported, mirrors Rust `pub(crate)`), recursive
        DCT; 10 tests in `dct_test.go` (including permutation table verification)
    - `packages/go/wtahash.go` (92 lines): `AlgWtahash` (exported); 9 tests in `wtahash_test.go`
    - All 7 algorithm modules now complete; 124 total pure Go tests (excl. WASM bridge tests)
- **Remaining (Step 5)**:
    - 9 `gen_*_v0` functions (Gen prefix, idiomatic Go naming)
    - `DataHasher` / `InstanceHasher` streaming with `io.Reader` support
    - `conformance_selftest` — validates against `data.json` vectors
    - `github.com/zeebo/blake3` dependency not yet added (needed for gen_data_code_v0,
        gen_instance_code_v0)
- `iscc.go` (1,357 lines) WASM/wazero bridge still present — coexists during transition
- `iscc_ffi.wasm` (683KB) still committed to git; `go.mod` still has `wazero` dependency
- `.pre-commit-config.yaml` large-file threshold still raised to 1024KB (must restore to 256KB after
    binary removal)
- 46 test functions in `iscc_test.go` valid as regression suite (currently pass via WASM bridge)

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
    [Run 22499935084](https://github.com/iscc/iscc-lib/actions/runs/22499935084) — all 7 jobs
    SUCCESS
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (0.0.2 not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Continue pure Go rewrite — implement gen functions layer (Step 5 of ~5):**

All 7 algorithm modules are complete. The gen functions layer is the final step before WASM bridge
removal. Recommended sequencing within step 5:

1. **`gen_meta_code_v0` + `gen_text_code_v0`** — only need SimHash, MinHash, and already-ported text
    utils. No new dependencies. Enables immediate conformance validation against data.json vectors
    (16 meta + 5 text = 21 vectors)
2. **`gen_data_code_v0` + `gen_instance_code_v0`** — need CDC + MinHash + BLAKE3. Add
    `github.com/zeebo/blake3` dependency first
3. **`gen_image_code_v0` + `gen_video_code_v0`** — need DCT + WTA-Hash (now done)
4. **`gen_audio_code_v0` + `gen_mixed_code_v0` + `gen_iscc_code_v0`** — complete the set
5. **`DataHasher` / `InstanceHasher` streaming** — `io.Reader` interface
6. **`conformance_selftest`** — final validation against data.json
7. **Cleanup** — remove `iscc.go`, `iscc_ffi.wasm`, wazero dep; restore large-file threshold to
    256KB; simplify Go CI job
