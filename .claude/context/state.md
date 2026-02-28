<!-- assessed-at: cc70146dc8af829f6b74c7288796b72acad3f809 -->

# Project State

## Status: IN_PROGRESS

## Phase: All 7 Bindings + Docs Complete — Tab Order + Publishing Remain

All seven language bindings (Rust, Python, Node.js, WASM, C FFI, Java, Go) export the full 30/30
Tier 1 symbols and pass conformance. CI is green on all 8 jobs. PR #10 (develop → main) is open. The
landing page Go example was fixed this iteration (stale WASM-bridge API → pure Go API). One
low-priority doc issue remains: tab order inconsistency across pages (needs human decision on
canonical order). Non-doc gaps: benchmark speedup documentation, Maven Central publishing, and
npm/crates.io release triggers.

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

**Status**: met — 30/30 Tier 1 symbols

- **Architecture**: Pure Go, no CGO, no WASM, no binary artifacts — target fully met
- **30/30 Tier 1 symbols**: All 9 `gen_*_v0` functions, `ConformanceSelftest`, `DataHasher`,
    `InstanceHasher`, 4 text utilities, `SlidingWindow`, `AlgMinhash256`, `AlgCdcChunks`,
    `AlgSimhash`, `SoftHashVideoV0`, `EncodeBase64`, `EncodeComponent`, `IsccDecode`,
    `IsccDecompose`, `JsonToDataUrl`, 4 constants (`MetaTrimName`, `MetaTrimDescription`,
    `IoReadSize`, `TextNgramSize`)
- 147 pure Go test functions across 18+ test files; `go test ./...` and `go vet ./...` pass
    (CI-verified)
- `go.mod` has minimal deps: `zeebo/blake3`, `golang.org/x/text`, `klauspost/cpuid` (indirect)
- **Nothing missing** in Go bindings
- Minor: 5 test files retain vestigial "do NOT require the WASM binary" comments (cosmetic only)

## README

**Status**: met

- Rewritten public-facing polyglot developer README (238 lines)
- All 6 language bindings mentioned; per-language install + Quick Start; all 9 `gen_*_v0` listed
- CI badge, DeepWiki badge, version badges for all registries

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- `packages/go/README.md` updated to reflect pure Go: no wazero references, package-level functions,
    `Push` → `Finalize` streaming API, no binary artifact description
- **Nothing missing** in Per-Crate READMEs

## Documentation

**Status**: partially met

- **14 pages** deployed to lib.iscc.codes; all navigation sections complete
- `docs/llms.txt` references all 14 doc pages; `scripts/gen_llms_full.py` generates
    `site/llms-full.txt`
- Getting-started tutorial in tabbed multi-language format: 7 sections × 6 languages (Python, Rust,
    Node.js, Java, Go, WASM)
- Landing page Go tab fixed: stale WASM-bridge API (`NewRuntime`/`ctx`) replaced with pure Go API
    (`result, _ := iscc.GenTextCodeV0(...)`) — verified this iteration
- All 6 binding howto guides have "Codec operations" and "Constants" sections
- Site builds and deploys via GitHub Pages; latest Docs run on main: PASSING
- ISCC branding, copy-page split-button, Open Graph meta tags in place
- **Known issue (low priority, filed, needs human decision):**
    - Tab order inconsistency across pages: spec says "Python, Rust, Java, Node.js, WASM" (no Go),
        landing page uses "Rust, Python, ...", tutorial uses "Python, Rust, Node.js, Java, Go, WASM" —
        spec update needed to add Go; `HUMAN REVIEW REQUESTED` for canonical tab order

## Benchmarks

**Status**: partially met

- Criterion benchmarks exist for all 9 `gen_*_v0` functions + `bench_data_hasher_streaming`
- pytest-benchmark comparison files present
- `Bench (compile check)` job in CI (`cargo bench --no-run`) — all 7 benchmark targets compile
    (CI-verified, run 22513202460)
- Missing: CI does not run benchmarks and collect results; speedup factors not published in
    documentation

## CI/CD and Publishing

**Status**: partially met

- 3 workflows: `ci.yml`, `docs.yml`, `release.yml`
- `ci.yml` covers **8 jobs**: Rust (fmt, clippy, test), Python (ruff, pytest), Node.js (napi build,
    test), WASM (wasm-pack test), Java (JNI build, mvn test), Go (go test, go vet), C FFI (cbindgen,
    gcc, test), **Bench (compile check)** — all 8 SUCCESS
- Go CI job is clean: checkout → setup-go → `CGO_ENABLED=0 go test` → `go vet` (no Rust toolchain)
- **Latest CI run on develop: PASSING** —
    [Run 22513202460](https://github.com/iscc/iscc-lib/actions/runs/22513202460) — all 8 jobs
    SUCCESS
- **PR #10 open**: develop → main "Pure Go rewrite & polyglot bindings progress"
    ([#10](https://github.com/iscc/iscc-lib/pull/10))
- Missing: OIDC trusted publishing for crates.io not configured (registry-side; human task)
- Missing: npm publishing awaiting new release trigger (`0.0.2` not yet published)
- Missing: Maven Central publishing configuration (GPG signing, Sonatype)

## Next Milestone

**Tab order standardization requires human decision; next automatable step is benchmark speedup
documentation.**

The one remaining low-priority doc issue (tab order) is blocked on a human decision about the
canonical order (whether Go is included and where). That's not an automated CID task without the
decision.

The next high-value automated step is **benchmark speedup documentation**:

1. Run `cargo bench` to capture Rust benchmark results
2. Run `pytest --benchmark-only` to capture Python baseline
3. Document speedup factors in the documentation site (e.g., a `docs/benchmarks.md` page or within
    the existing `docs/howto/` pages)

After benchmarks, the remaining non-automated gaps are publishing (PyPI 0.0.2, npm 0.0.2, Maven
Central) — these require registry-side configuration or human-triggered releases.

Merging PR #10 (develop → main) remains a human task requiring approval.
