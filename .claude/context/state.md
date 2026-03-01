<!-- assessed-at: f2eeb6dd134fe5badff3509a7c4d98a428322e57 -->

# Project State

## Status: IN_PROGRESS

## Phase: gen_sum_code_v0 — Rust core complete (32/32 Tier 1); bindings propagation next

Iteration 5 completed: `gen_sum_code_v0` + `SumCodeResult` are fully implemented and CI-verified in
the Rust core crate (all 32 Tier 1 symbols now present). The implementation performs single-pass
file I/O feeding both `DataHasher` and `InstanceHasher`, with 7 equivalence/error/wide-mode tests.
All 11 CI jobs remain green. The sole remaining gap is propagating `gen_sum_code_v0` +
`SumCodeResult` to all 6 binding crates (Python, Node.js, WASM, C FFI, Java, Go).

## Rust Core Crate

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols present: 10 `gen_*_v0` functions (including new `gen_sum_code_v0`), 4 text
    utilities, 4 algo primitives, 1 soft hash, 2 encoding utilities, 3 codec operations, 5 constants
    (META_TRIM_NAME, META_TRIM_DESCRIPTION, META_TRIM_META, IO_READ_SIZE, TEXT_NGRAM_SIZE), 2
    streaming types, 1 diagnostic
- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` implemented at
    line 967 of `crates/iscc-lib/src/lib.rs` as `pub fn`; single-pass file I/O ✅
- `SumCodeResult { iscc: String, datahash: String, filesize: u64 }` in `types.rs` exported via
    `pub use types::*` ✅
- 310 tests passing (256 unit + 31 streaming + 22 utils + 1 doctest) — 7 new gen_sum_code_v0 tests
- `cargo clippy -p iscc-lib -- -D warnings` clean; `cargo fmt -p iscc-lib --check` clean
- Module docstring updated from "9 `gen_*_v0`" to "10 `gen_*_v0`" ✅
- `units: Vec<String>` field deferred (not in scope per next.md)

## Python Bindings

**Status**: partially met (missing gen_sum_code_v0, SumCodeResult)

- 31/32 Tier 1 symbols accessible; all conformance vectors pass (CI-verified on 3.10 + 3.14)
- `__all__` has 46 entries; `META_TRIM_META` exported ✅
- `core_opts.meta_trim_meta = META_TRIM_META` attribute added ✅
- `_lowlevel.pyi` type stub updated with `META_TRIM_META` ✅
- 198 Python tests passing; `cargo clippy -p iscc-py` clean
- **MISSING**: `gen_sum_code_v0` PyO3 wrapper accepting `str | os.PathLike`
- **MISSING**: `SumCodeResult` Python class with dict + attribute access

## Node.js Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` added in `crates/iscc-napi/src/lib.rs`
    with 2 test assertions (value == 128000 and type == 'number') ✅
- 80 `it()` test cases CI-verified passing
- **MISSING**: `gen_sum_code_v0` napi export + `SumCodeResult` JS class
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` getter added in
    `crates/iscc-wasm/src/lib.rs` with unit test (`test_meta_trim_meta_value`) ✅
- 61+ wasm-bindgen tests CI-verified passing
- **MISSING**: `gen_sum_code_v0` wasm_bindgen export (path-based I/O needs design decision: likely
    accept `Uint8Array` bytes since WASM has no filesystem access)
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: partially met (missing gen_sum_code_v0)

- 45 `extern "C"` functions; `iscc_meta_trim_meta()` added with Rust unit test + C test assertion ✅
- 78 Rust unit tests + C test program (23+ cases) CI-verified passing
- **MISSING**: `iscc_gen_sum_code_v0` extern "C" function + memory management helpers for result
    struct

## Java Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 32 existing `extern "system"` JNI functions in `crates/iscc-jni/src/lib.rs`; all Java tests pass
- `META_TRIM_META = 128_000` added as `public static final int` in `IsccLib.java` ✅
- `assertEquals(128_000, IsccLib.META_TRIM_META)` test assertion in `IsccLibTest.java` ✅
- CI-verified: `Java (JNI build, mvn test)` job SUCCESS
- **MISSING**: JNI bridge + Java static native method for `genSumCodeV0`
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/32 Tier 1 symbols in `packages/go/`; `MetaTrimMeta = 128_000` constant in `codec.go` ✅
- 147 pure Go tests CI-verified passing (`CGO_ENABLED=0`); `go vet` clean
- **NOTE**: No explicit test asserting `MetaTrimMeta == 128_000`; constant declared but only
    `MetaTrimName` and `MetaTrimDescription` are referenced in Go source functions
- **MISSING**: `GenSumCodeV0(path string, bits uint32, wide bool)` function + `SumCodeResult` struct

## README

**Status**: met

- Public-facing polyglot README (238 lines); all 6 bindings, all 9 `gen_*_v0` listed, CI badge,
    registry badges
- Will need update for `gen_sum_code_v0` when bindings are implemented

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
- Will need `gen_sum_code_v0` mention when bindings are implemented

## Benchmarks

**Status**: met

- Criterion benchmarks for all 9 `gen_*_v0` + `bench_data_hasher_streaming` + `bench_cdc_chunks`
    (4KB/64KB/1MB)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py`
- Speedup factors published in `docs/benchmarks.md`
- `Bench (compile check)` CI job verifies all benchmark targets compile

## CI/CD and Publishing

**Status**: met (for existing features)

- **11/11 CI jobs all SUCCESS** on latest push; all 5 recent runs green
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22550764467
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest), Node.js, WASM, C
    FFI, Java, Go, Bench — all success
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Propagate gen_sum_code_v0 + SumCodeResult to all 6 binding crates (issue #15):**

1. **Python** (`crates/iscc-py/`): PyO3 wrapper
    `gen_sum_code_v0(path: str | PathLike) ->  SumCodeResult`; `SumCodeResult` class with dict +
    attribute access; update `__all__`, type stubs; add pytest tests
2. **Node.js** (`crates/iscc-napi/`): napi export `genSumCodeV0(path: string)` returning
    `SumCodeResult`-shaped object; update TS types; add mocha tests
3. **WASM** (`crates/iscc-wasm/`): design decision — accept `Uint8Array` bytes (WASM has no FS
    access); add wasm-bindgen export + tests
4. **C FFI** (`crates/iscc-ffi/`): `iscc_gen_sum_code_v0(path, bits, wide)` + opaque result struct
    with string/int accessors; update C test program
5. **Java** (`crates/iscc-jni/`): JNI bridge +
    `static native String[] genSumCodeV0(String path,  int bits, boolean wide)` in `IsccLib.java`;
    add `SumCodeResult` record; mvn tests
6. **Go** (`packages/go/`):
    `GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult,  error)` + `SumCodeResult`
    struct; pure Go implementation (CGO_ENABLED=0); tests
