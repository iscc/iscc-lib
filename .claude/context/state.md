<!-- assessed-at: ba1a0302595036bab3e2f6e5f949f45266147e4d -->

# Project State

## Status: IN_PROGRESS

## Phase: C FFI DX — 1 target criterion unmet; 3 open issues

Commit `ba1a030` resolved issue #22: `docs/howto/c-cpp.md` (433 lines) now exists with all required
sections (CMake, streaming, RAII wrapper, error handling, memory management, static vs dynamic
linking, cross-compilation, conformance verification) and is linked in site navigation.
Documentation section is now fully met. One C FFI target criterion remains unmet (pre-built FFI
tarballs, issue #25), plus issues #21 (units support) and #16 (feature flags). All 11 CI jobs pass.

## Rust Core Crate

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols present: 10 `gen_*_v0` functions (including `gen_sum_code_v0`), 4 text
    utilities, 4 algo primitives, 1 soft hash, 2 encoding utilities, 3 codec operations, 5 constants
    (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE`), 2 streaming types, 1 diagnostic
- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` — single-pass
    file I/O ✅
- `SumCodeResult { iscc: String, datahash: String, filesize: u64 }` in `types.rs` ✅
- 310 tests passing (256 unit + 31 streaming + 22 utils + 1 doctest)
- `cargo clippy -p iscc-lib -- -D warnings` clean; `cargo fmt -p iscc-lib --check` clean

## Python Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols accessible; `__all__` has 48 entries (32 API + 11 result types +
    `__version__` + `MT`, `ST`, `VS`, `core_opts`) ✅
- `gen_sum_code_v0(path: str | os.PathLike, bits: int = 64, wide: bool = False) -> SumCodeResult` ✅
- `SumCodeResult(IsccResult)` class with `iscc`, `datahash`, `filesize` attributes ✅
- 6 pytest tests for `gen_sum_code_v0`; 204 Python tests passing ✅
- `cargo clippy -p iscc-py` clean; `ruff check` clean ✅

## Node.js Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `NapiSumCodeResult` struct with `iscc: String`, `datahash: String`, `filesize: i64` ✅
- 6 mocha tests for `gen_sum_code_v0`; 132 total tests pass ✅
- `cargo clippy -p iscc-napi -- -D warnings` clean
- `@iscc/lib 0.0.4` on npm

## WASM Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `WasmSumCodeResult` struct with `iscc: String`, `datahash: String`, `filesize: f64` ✅
- 6 wasm-bindgen tests for `gen_sum_code_v0`; 75 total wasm-bindgen tests ✅
- `cargo clippy -p iscc-wasm` clean
- `@iscc/wasm 0.0.4` on npm

## C FFI

**Status**: partially met (5/6 criteria met; pre-built tarballs outstanding)

- cbindgen generates valid C headers ✅
- C test program calls entrypoints and gets correct results ✅
- `crates/iscc-ffi/include/iscc.h` committed ✅ (resolved issue #24)
- CI freshness check regenerates `iscc.h` and asserts `git diff --exit-code` ✅
- `crates/iscc-ffi/examples/iscc_sum.c` (147 lines, dual-hasher streaming, C89/C99 compatible, full
    error handling) + `CMakeLists.txt` exist and compile correctly ✅ (resolved issue #23)
- `docs/howto/c-cpp.md` exists (433 lines) with all required sections — linked in navigation ✅
    (resolved issue #22)
- Pre-built FFI tarballs not set up — `release.yml` has no `build-ffi`/`publish-ffi` jobs ❌ (issue
    #25)

## Java Bindings

**Status**: met (32/32 Tier 1 symbols; all quality gates pass)

- `SumCodeResult.java` immutable class with `iscc: String`, `datahash: String`, `filesize: long` ✅
- `public static native SumCodeResult genSumCodeV0(String path, int bits, boolean wide)` in
    `IsccLib.java` ✅
- 62 total mvn tests; CI "Java (JNI build, mvn test)" SUCCESS ✅
- `cargo clippy -p iscc-jni -- -D warnings` clean
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: met (32/32 Tier 1 symbols; all quality gates pass)

- `code_sum.go`: `SumCodeResult` struct (`Iscc string`, `Datahash string`, `Filesize uint64`) +
    `GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult, error)` ✅
- 151 total Go tests; `go vet ./...` clean ✅
- CI "Go (go test, go vet)" SUCCESS

## README

**Status**: met

- Public-facing polyglot README; all 6 bindings, CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed in Implementors Guide section (including `gen_sum_code_v0`) ✅
- Per-language installation instructions: Rust, Python, Java, Go, Node.js, WASM ✅
- Per-language quick-start code examples ✅
- ISCC architecture diagram (`iscc-codec-light.png`) and MainTypes table ✅
- Links to `lib.iscc.codes` ✅
- No development workflow content ✅

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples ✅
- All 7 READMEs mention `gen_sum_code_v0` (or language-specific equivalent) in their API overview
    tables ✅

## Documentation

**Status**: met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- Getting-started tutorial: 7 sections × 6 languages ✅
- `docs/rust-api.md` has full `gen_sum_code_v0` section with code example ✅
- `docs/c-ffi-api.md` has `iscc_gen_sum_code_v0` + `IsccSumCodeResult` struct documented ✅
- `docs/index.md` lists `gen_sum_code_v0` in function table ✅
- `docs/architecture.md` references `gen_sum_code_v0` ✅
- All existing 6 howto guides have `### Sum-Code` subsections with working code examples ✅
- `docs/howto/c-cpp.md` (433 lines) exists and is linked in navigation ✅ (resolved issue #22)
- `uv run zensical build` exits 0 ✅

## Benchmarks

**Status**: met

- Criterion benchmarks exist for all 10 `gen_*_v0` functions:
    - `bench_meta_code`, `bench_text_code`, `bench_image_code`, `bench_audio_code`,
        `bench_video_code`, `bench_mixed_code`, `bench_data_code`, `bench_instance_code`,
        `bench_iscc_code`, `bench_sum_code` (64KB + 1MB throughput using `NamedTempFile`) ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py` ✅
- Speedup factors published in `docs/benchmarks.md` ✅
- `Bench (compile check)` CI job passes ✅

## CI/CD and Publishing

**Status**: partially met

- **All 11 CI jobs SUCCESS** on latest push — **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22589681430
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check)
- v0.0.4 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via `NPM_TOKEN` secret
- `release.yml` is missing `build-ffi` and `publish-ffi` jobs ❌ (issue #25)
- `workflow_dispatch` does not have `ffi` boolean input ❌ (issue #25)

## Next Milestone

**Remaining open issues in priority order:**

1. **#25** — Add `build-ffi` and `publish-ffi` jobs to `release.yml` with 5-platform matrix
    (`x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `aarch64-apple-darwin`,
    `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`). Each platform produces a tarball containing
    shared lib + static lib + `iscc.h` + LICENSE. Add `ffi` boolean input to `workflow_dispatch`.
    Tarballs uploaded as GitHub Release assets with naming `iscc-ffi-v{version}-{target}.tar.gz`.
2. **#21** — Units support for `gen_sum_code_v0`: add `add_units: bool` parameter and
    `units: Option<Vec<String>>` field to `SumCodeResult`; update all bindings.
3. **#16** — Feature flags for embedded/minimal builds (low priority).
