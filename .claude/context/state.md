<!-- assessed-at: 1ba01b1e275707a4439356212c8b3016d193402d -->

# Project State

## Status: IN_PROGRESS

## Phase: gen_sum_code_v0 — META_TRIM_META complete across all 6 bindings; Rust core implementation next

Iteration 4 completed: `META_TRIM_META = 128_000` is now exported in all 6 bindings (Rust, Python,
Node.js, WASM, C FFI, Java, Go) — issue #18 fully resolved. CI remains fully green (11/11 jobs). The
sole remaining gap is `gen_sum_code_v0` + `SumCodeResult`, which requires Rust core implementation
first, then propagation to all 6 bindings.

## Rust Core Crate

**Status**: partially met (31/32 Tier 1 symbols; gen_sum_code_v0 unimplemented)

- 31 Tier 1 symbols present: all 9 existing `gen_*_v0`, 4 text utilities, 4 algo primitives, 1 soft
    hash, 2 encoding utilities, 3 codec operations, **5 constants** (META_TRIM_NAME,
    META_TRIM_DESCRIPTION, META_TRIM_META, IO_READ_SIZE, TEXT_NGRAM_SIZE), 2 streaming types, 1
    diagnostic
- `META_TRIM_META: usize = 128_000` in `crates/iscc-lib/src/lib.rs` (CI-verified)
- `gen_meta_code_v0` pre-decode + post-decode payload validation implemented and tested
- 303 tests passing (249 lib + 31 integration + 22 utils + 1 doctest); `cargo clippy` clean
- **MISSING**: `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` —
    single-pass file I/O for Data+Instance code (issue #15)
- **MISSING**: `SumCodeResult` struct with `iscc`, `datahash`, `filesize` fields

## Python Bindings

**Status**: partially met (missing gen_sum_code_v0, SumCodeResult)

- 31/32 Tier 1 symbols accessible; all conformance vectors pass (CI-verified on 3.10 + 3.14)
- `__all__` has 46 entries; `META_TRIM_META` exported ✅
- `core_opts.meta_trim_meta = META_TRIM_META` attribute added ✅
- `_lowlevel.pyi` type stub updated with `META_TRIM_META` ✅
- 198 Python tests passing; `cargo clippy -p iscc-py` clean
- **MISSING**: `gen_sum_code_v0` wrapper accepting `str | os.PathLike`
- **MISSING**: `SumCodeResult` class with dict + attribute access

## Node.js Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` added in `crates/iscc-napi/src/lib.rs`
    with 2 test assertions (value == 128000 and type == 'number') ✅
- 80 `it()` test cases CI-verified passing
- **MISSING**: `gen_sum_code_v0` napi export
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` getter added in
    `crates/iscc-wasm/src/lib.rs` with unit test (`test_meta_trim_meta_value`) ✅
- 61+ wasm-bindgen tests CI-verified passing
- **MISSING**: `gen_sum_code_v0` wasm_bindgen export (path-based I/O needs design decision for WASM
    context)
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: partially met (missing gen_sum_code_v0)

- 45 `extern "C"` functions now; `iscc_meta_trim_meta()` added in `crates/iscc-ffi/src/lib.rs` with
    Rust unit test + C test assertion (`iscc_meta_trim_meta() == 128000`) ✅
- 78 Rust unit tests + C test program (23+ cases) CI-verified passing
- **MISSING**: `iscc_gen_sum_code_v0` extern "C" function + memory management helpers for result

## Java Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 32 existing `extern "system"` JNI functions in `crates/iscc-jni/src/lib.rs`; all Java tests pass
- `META_TRIM_META = 128_000` added as `public static final int` in `IsccLib.java` ✅
- `assertEquals(128_000, IsccLib.META_TRIM_META)` test assertion added in `IsccLibTest.java` ✅
- CI-verified: `Java (JNI build, mvn test)` job SUCCESS
- **MISSING**: JNI bridge + Java static native method for `genSumCodeV0`
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/32 Tier 1 symbols now in `packages/go/`; `MetaTrimMeta = 128_000` constant added in
    `packages/go/codec.go` ✅
- 147 pure Go tests CI-verified passing (`CGO_ENABLED=0`); `go vet` clean
- **NOTE**: No explicit test asserting `MetaTrimMeta == 128_000` in Go test files; constant is
    declared but only `MetaTrimName` and `MetaTrimDescription` are actually referenced in Go source
    functions. CI still passes.
- **MISSING**: `GenSumCodeV0(path string, bits uint32, wide bool)` function
- **MISSING**: `SumCodeResult` struct in Go

## README

**Status**: met

- Public-facing polyglot README (238 lines); all 6 bindings, all 9 `gen_*_v0` listed, CI badge,
    registry badges
- Will need update for `gen_sum_code_v0` when implemented

## Per-Crate READMEs

**Status**: met (for existing 31 symbols)

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- Will need `gen_sum_code_v0` mention when implemented in remaining bindings

## Documentation

**Status**: met (for existing features)

- 16 pages deployed to lib.iscc.codes; all navigation sections complete
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place
- Getting-started tutorial: 7 sections × 6 languages; all howto guides complete
- Benchmarks page updated; `docs/ecosystem.md` current
- Will need `gen_sum_code_v0` mention when implemented

## Benchmarks

**Status**: met

- Criterion benchmarks for all 9 `gen_*_v0` + `bench_data_hasher_streaming` + `bench_cdc_chunks`
    (4KB/64KB/1MB)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py`
- Speedup factors published in `docs/benchmarks.md`
- `Bench (compile check)` CI job verifies all benchmark targets compile

## CI/CD and Publishing

**Status**: met (for existing features)

- **11/11 CI jobs all SUCCESS** on latest push
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22550003423
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench — all success
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Implement gen_sum_code_v0 + SumCodeResult in Rust core, then propagate to all 6 bindings (issue
#15):**

1. Implement `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` in
    `crates/iscc-lib/src/` — single-pass file I/O composing `DataHasher` + `InstanceHasher` in one
    read loop, then calling `gen_iscc_code_v0` internally
2. Add `SumCodeResult` struct with `iscc: String`, `datahash: String`, `filesize: u64` fields (and
    optional `units: Vec<String>`)
3. Add conformance tests verifying output matches two-pass `gen_data_code_v0` +
    `gen_instance_code_v0`
4. Propagate to all 6 bindings: Python (`str | os.PathLike`), Node.js (napi), WASM (design decision:
    Uint8Array or skip path-based), C FFI (`iscc_gen_sum_code_v0`), Java (JNI + Java wrapper), Go
    (`GenSumCodeV0(path string, bits uint32, wide bool)`)
