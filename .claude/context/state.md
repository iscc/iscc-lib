<!-- assessed-at: 637722d -->

# Project State

## Status: IN_PROGRESS

## Phase: gen_sum_code_v0 complete (7/7 bindings); README/docs cleanup pending

Iteration 13 completed: `GenSumCodeV0` propagated to Go bindings — issue #15 is fully resolved. All
7 language bindings (Rust core, Python, Node.js, WASM, C FFI, Java, Go) now expose
`gen_sum_code_v0`. CI is all-green (11/11 jobs SUCCESS). Remaining work: update README, per-crate
READMEs, docs, and FFI module docstring to mention `gen_sum_code_v0`; then address issue #16
(feature flags for minimal builds).

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
- PyO3 `#[pyfunction] fn gen_sum_code_v0` in `crates/iscc-py/src/lib.rs`; registered in
    `iscc_lowlevel` module; type stub in `_lowlevel.pyi` ✅
- 6 pytest tests for `gen_sum_code_v0`; 204 Python tests passing ✅
- `cargo clippy -p iscc-py` clean; `ruff check` clean ✅

## Node.js Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `NapiSumCodeResult` struct (`#[napi(object)]`) with `iscc: String`, `datahash: String`,
    `filesize: i64` in `crates/iscc-napi/src/lib.rs` ✅
- `NapiSumCodeResult` interface + `gen_sum_code_v0` declaration in auto-generated `index.d.ts` ✅
- 6 mocha tests for `gen_sum_code_v0`; 132 total tests pass ✅
- Review verdict: PASS; `cargo clippy -p iscc-napi -- -D warnings` clean
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `WasmSumCodeResult` struct (`#[wasm_bindgen(getter_with_clone)]`) with `iscc: String`,
    `datahash: String`, `filesize: f64` ✅
- 6 wasm-bindgen tests for `gen_sum_code_v0`; 75 total wasm-bindgen tests ✅
- Review verdict: PASS; `cargo clippy -p iscc-wasm` clean
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: met (32/32 Tier 1 symbols; gen_sum_code_v0 added — review PASS)

- `IsccSumCodeResult` repr(C) struct + `iscc_gen_sum_code_v0` + `iscc_free_sum_code_result` in
    `crates/iscc-ffi/src/lib.rs` ✅
- 82 Rust unit tests; 57 C assertions ✅
- Review verdict: PASS; `cargo clippy -p iscc-ffi -- -D warnings` clean
- **Minor gap**: module docstring still says "9 `gen_*_v0` functions" (should be 10) — cosmetic only

## Java Bindings

**Status**: met (32/32 Tier 1 symbols; gen_sum_code_v0 added — review PASS)

- `SumCodeResult.java` immutable class with `iscc: String`, `datahash: String`, `filesize: long` ✅
- `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` JNI bridge function in `crates/iscc-jni/src/lib.rs`
    ✅
- `public static native SumCodeResult genSumCodeV0(String path, int bits, boolean wide)` in
    `IsccLib.java` ✅
- 62 total mvn tests (58 existing + 4 new); CI "Java (JNI build, mvn test)" SUCCESS ✅
- Review verdict: PASS; `cargo clippy -p iscc-jni -- -D warnings` clean
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: met (32/32 Tier 1 symbols; gen_sum_code_v0 added — review PASS)

- `code_sum.go`: `SumCodeResult` struct (`Iscc string`, `Datahash string`, `Filesize uint64`) +
    `GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult, error)` ✅
- Single-pass file I/O: feeds `DataHasher` and `InstanceHasher` from shared 4MB buffer, composes via
    `GenIsccCodeV0`; idiomatic error wrapping, `defer f.Close()`, `io.EOF` handling ✅
- `code_sum_test.go`: 4 tests (equivalence, result fields, non-existent-file error, wide mode) ✅
- 151 total Go tests (147 existing + 4 new); `go vet ./...` clean ✅
- CI "Go (go test, go vet)" SUCCESS; review verdict: PASS
- **Issue #15 fully resolved** — `gen_sum_code_v0` present in all 7 bindings

## README

**Status**: partially met

- Public-facing polyglot README (238 lines); all 6 bindings, CI badge, registry badges ✅
- **MISSING**: `gen_sum_code_v0` not listed among the 9 gen\_\*\_v0 entry points (README lists only
    9 but doesn't include this 10th function by name) — needs update now that all 7 bindings
    complete
- Go README (`packages/go/README.md`) API table omits `GenSumCodeV0` from code generators table

## Per-Crate READMEs

**Status**: partially met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples ✅
- **MISSING**: None of the 7 per-crate READMEs mention `gen_sum_code_v0` — all need updating now
    that the function is fully implemented across all bindings

## Documentation

**Status**: partially met

- 16 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- Getting-started tutorial: 7 sections × 6 languages; all howto guides complete ✅
- **MISSING**: docs pages don't mention `gen_sum_code_v0` — grep confirms zero hits in `docs/`

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
- URL: https://github.com/iscc/iscc-lib/actions/runs/22558240656
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check) —
    all success ✅
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Update README, per-crate READMEs, docs, and FFI docstring for `gen_sum_code_v0`:**

1. Fix `crates/iscc-ffi/src/lib.rs` module docstring: "9 `gen_*_v0` functions" → "10 `gen_*_v0`
    functions"
2. Update `README.md` to list `gen_sum_code_v0` among the 10 gen\_\*\_v0 entry points
3. Update `packages/go/README.md` API table to include `GenSumCodeV0`
4. Update all 7 per-crate READMEs to mention `gen_sum_code_v0` in their API overview sections
5. Update `docs/` pages that list gen\_\*\_v0 functions to include `gen_sum_code_v0`

After documentation cleanup, address issue #16 (feature flags for minimal builds, low priority).
