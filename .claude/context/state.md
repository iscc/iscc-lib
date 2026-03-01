<!-- assessed-at: cb9615e78a6e3221c56601ce67070390b1d32997 -->

# Project State

## Status: IN_PROGRESS

## Phase: gen_sum_code_v0 propagation — Node.js done (2/6 bindings); WASM next

Iteration 8 completed: `gen_sum_code_v0` + `NapiSumCodeResult` fully propagated to Node.js bindings
(32/32 Tier 1 symbols in Node.js). The napi struct, function, TypeScript declarations, and 6 mocha
tests are all present and CI-verified (132 total tests pass). Four binding crates still lack
`gen_sum_code_v0`: WASM, C FFI, Java, Go.

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
- `gen_sum_code_v0(path: str | os.PathLike, bits: int = 64, wide: bool = False) -> SumCodeResult` in
    `crates/iscc-py/python/iscc_lib/__init__.py` at line 274 ✅
- `SumCodeResult(IsccResult)` class with `iscc`, `datahash`, `filesize` attributes in `__init__.py`
    at line 185 ✅
- PyO3 `#[pyfunction] fn gen_sum_code_v0(path: &str, bits: u32, wide: bool)` in
    `crates/iscc-py/src/lib.rs` at line 334; registered in `iscc_lowlevel` module at line 612 ✅
- `gen_sum_code_v0` type stub in `_lowlevel.pyi` at line 326 ✅
- 6 pytest tests for `gen_sum_code_v0` in `tests/test_smoke.py` (equivalence, PathLike, error,
    result type, attribute access, wide mode) ✅
- 204 Python tests passing (25 smoke + 179 conformance/other); `cargo clippy -p iscc-py` clean;
    `ruff check` clean ✅

## Node.js Bindings

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols exported including `gen_sum_code_v0` ✅
- `NapiSumCodeResult` struct (`#[napi(object)]`) with `iscc: String`, `datahash: String`,
    `filesize: i64` in `crates/iscc-napi/src/lib.rs` ✅
- `#[napi(js_name = "gen_sum_code_v0")] fn gen_sum_code_v0(path, bits?, wide?)` with
    `Option<u32>`/`Option<bool>` params ✅
- `NapiSumCodeResult` interface + `gen_sum_code_v0` declaration in auto-generated `index.d.ts` ✅
- 6 mocha tests for `gen_sum_code_v0` in `functions.test.mjs` (equivalence, shape, error, defaults,
    wide mode, filesize) ✅; 132 total tests pass (95 `it()` blocks across 2 test files)
- Review verdict: PASS; `cargo clippy -p iscc-napi -- -D warnings` clean
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` getter added with unit test ✅
- 61+ wasm-bindgen tests CI-verified passing
- **MISSING**: `gen_sum_code_v0` wasm_bindgen export (design: accept `Uint8Array` bytes since WASM
    has no filesystem access)
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: partially met (missing gen_sum_code_v0)

- 45 `extern "C"` functions; `iscc_meta_trim_meta()` added with unit test ✅
- 78 Rust unit tests + C test program (23+ cases) CI-verified passing
- **MISSING**: `iscc_gen_sum_code_v0` extern "C" function + memory management helpers for result
    struct

## Java Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 32 existing `extern "system"` JNI functions; `META_TRIM_META = 128_000` added in `IsccLib.java`
    with test assertion ✅
- CI-verified: `Java (JNI build, mvn test)` job SUCCESS
- **MISSING**: JNI bridge + `genSumCodeV0(String path, int bits, boolean wide)` native method +
    `SumCodeResult` record in Java
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: partially met (missing gen_sum_code_v0)

- 31/32 Tier 1 symbols in `packages/go/`; `MetaTrimMeta = 128_000` constant in `codec.go` ✅
- 147 pure Go tests CI-verified passing (`CGO_ENABLED=0`); `go vet` clean
- **MISSING**: `GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult, error)` +
    `SumCodeResult` struct + Go tests

## README

**Status**: met

- Public-facing polyglot README (238 lines); all 6 bindings, all 9 `gen_*_v0` listed, CI badge,
    registry badges
- Will need update for `gen_sum_code_v0` when remaining bindings are implemented

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
- Will need `gen_sum_code_v0` mention when remaining bindings are implemented

## Benchmarks

**Status**: met

- Criterion benchmarks for all 9 `gen_*_v0` + `bench_data_hasher_streaming` + `bench_cdc_chunks`
    (4KB/64KB/1MB)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py`
- Speedup factors published in `docs/benchmarks.md`
- `Bench (compile check)` CI job verifies all benchmark targets compile

## CI/CD and Publishing

**Status**: met (for existing features)

- **All 11 CI job records SUCCESS** on latest push; 3 consecutive green runs
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22552124782
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest), Node.js, WASM, C
    FFI, Java, Go, Bench — all success
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Propagate gen_sum_code_v0 to WASM bindings (`crates/iscc-wasm/`) — issue #15:**

1. **WASM** (next): WASM has no filesystem access — accept `Uint8Array` bytes and compose Data-Code
    \+ Instance-Code internally. Add `#[wasm_bindgen]` export + `WasmSumCodeResult` struct; add
    wasm-bindgen tests
2. **C FFI** (`crates/iscc-ffi/`): `iscc_gen_sum_code_v0(path, bits, wide)` + opaque result struct;
    update C test program
3. **Java** (`crates/iscc-jni/`): JNI bridge + `SumCodeResult` record + `genSumCodeV0` native
    method; mvn tests
4. **Go** (`packages/go/`): `GenSumCodeV0` + `SumCodeResult` struct; pure Go file I/O; tests
