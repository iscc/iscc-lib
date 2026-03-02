<!-- assessed-at: 000c35d -->

# Project State

## Status: IN_PROGRESS

## Phase: Documentation sweep complete; howto code examples for gen_sum_code_v0 pending

Iteration 14 completed the documentation sweep: `gen_sum_code_v0` propagated to all 9 READMEs (root
\+ 7 per-crate + 1 Go package) and key docs pages (architecture, c-ffi-api, index, rust-api). The C
FFI module docstring was corrected to "10 gen\_\*\_v0 functions". CI remains all-green (11/11 jobs
SUCCESS). Remaining gap: the 6 per-language howto guides (rust.md, python.md, nodejs.md, wasm.md,
java.md, go.md) each have a Code Generation section with per-function examples for the original 9
functions but no `gen_sum_code_v0` subsection with example code.

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
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `WasmSumCodeResult` struct with `iscc: String`, `datahash: String`, `filesize: f64` ✅
- 6 wasm-bindgen tests for `gen_sum_code_v0`; 75 total wasm-bindgen tests ✅
- `cargo clippy -p iscc-wasm` clean
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: met (32/32 Tier 1 symbols; all quality gates pass)

- `IsccSumCodeResult` repr(C) struct + `iscc_gen_sum_code_v0` + `iscc_free_sum_code_result` in
    `crates/iscc-ffi/src/lib.rs` ✅
- Module docstring now says "10 `gen_*_v0` functions" (was "9" — fixed in iteration 14) ✅
- 82 Rust unit tests; 57 C assertions ✅
- `cargo clippy -p iscc-ffi -- -D warnings` clean

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
- All 10 `gen_*_v0` functions listed in Implementors Guide section (including `gen_sum_code_v0` at
    line 213) ✅
- Per-language installation instructions: Rust, Python, Java, Go, Node.js, WASM ✅
- Per-language quick-start code examples ✅
- ISCC architecture diagram (`iscc-codec-light.png`) and MainTypes table ✅
- Links to `lib.iscc.codes` ✅
- No development workflow content ✅

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples ✅
- All 7 READMEs now mention `gen_sum_code_v0` (or language-specific equivalent) in their API
    overview tables:
    - `crates/iscc-lib/README.md` → `gen_sum_code_v0` ✅
    - `crates/iscc-py/README.md` → `gen_sum_code_v0` ✅
    - `crates/iscc-napi/README.md` → `gen_sum_code_v0` ✅
    - `crates/iscc-wasm/README.md` → `gen_sum_code_v0` ✅
    - `crates/iscc-ffi/README.md` → `iscc_gen_sum_code_v0` ✅
    - `crates/iscc-jni/README.md` → `genSumCodeV0` ✅
    - `packages/go/README.md` → `GenSumCodeV0` ✅

## Documentation

**Status**: partially met

- 16 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- Getting-started tutorial: 7 sections × 6 languages ✅
- `docs/rust-api.md` has full `gen_sum_code_v0` section with code example ✅
- `docs/c-ffi-api.md` has `iscc_gen_sum_code_v0` + `IsccSumCodeResult` struct documented ✅
- `docs/index.md` lists `gen_sum_code_v0` in function table ✅
- `docs/architecture.md` references `gen_sum_code_v0` ✅
- **MISSING**: 6 howto guides (`docs/howto/rust.md`, `python.md`, `nodejs.md`, `wasm.md`, `java.md`,
    `go.md`) each have a Code Generation section with per-function subsections for the original 9
    `gen_*_v0` functions but none have a `gen_sum_code_v0` code example subsection. Of these,
    nodejs.md, wasm.md, java.md, go.md have zero mentions; rust.md and python.md only mention
    `SumCodeResult` in a result types table

## Benchmarks

**Status**: met

- Criterion benchmarks for all 9 `gen_*_v0` + `bench_data_hasher_streaming` + `bench_cdc_chunks`
    (4KB/64KB/1MB)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py`
- Speedup factors published in `docs/benchmarks.md`
- `Bench (compile check)` CI job verifies all benchmark targets compile

## CI/CD and Publishing

**Status**: met

- **All 11 CI jobs SUCCESS** on latest push; latest CI run: **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22559228662
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check) —
    all success ✅
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Add `gen_sum_code_v0` code examples to all 6 per-language howto guides:**

Each of `docs/howto/rust.md`, `python.md`, `nodejs.md`, `wasm.md`, `java.md`, `go.md` has a Code
Generation section with per-function subsections for the 9 conformance `gen_*_v0` functions but no
`gen_sum_code_v0` subsection. Add a "Sum Code" subsection to each howto guide's Code Generation
section with a minimal working code example.

Note: `gen_sum_code_v0` in WASM takes `Uint8Array`/`&[u8]` (no path-based I/O in browser context)
vs. file path in all other bindings.

After howto guide completion, address issue #16 (feature flags for minimal builds, low priority).
