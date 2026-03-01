<!-- assessed-at: 21823e962ce016e5435732a5f4f3aeb203b30e71 -->

# Project State

## Status: IN_PROGRESS

## Phase: gen_sum_code_v0 propagation — Python done (1/6 bindings); Node.js next

Iteration 8 completed: `gen_sum_code_v0` + `SumCodeResult` fully propagated to Python bindings
(32/32 Tier 1 symbols in Python). The PyO3 wrapper, public API wrapper, type stub, `SumCodeResult`
class, and 6 pytest tests are all present and CI-verified. Five binding crates still lack
`gen_sum_code_v0`: Node.js, WASM, C FFI, Java, Go.

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

**Status**: partially met (missing gen_sum_code_v0)

- 31/31 existing Tier 1 symbols exported; `META_TRIM_META` added with 2 test assertions ✅
- 80 `it()` test cases CI-verified passing
- **MISSING**: `genSumCodeV0(path: string)` napi export + `SumCodeResult`-shaped object + TS types
    - mocha tests
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

- **11/11 CI jobs all SUCCESS** on latest push; all 3 recent runs green
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22551403022
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest), Node.js, WASM, C
    FFI, Java, Go, Bench — all success
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Propagate gen_sum_code_v0 to Node.js bindings (`crates/iscc-napi/`) — issue #15:**

1. **Node.js** (next): Add napi export `genSumCodeV0(path: string)` returning object with `iscc`,
    `datahash`, `filesize` keys — follow `genInstanceCodeV0` pattern; update TS type declarations;
    add mocha tests
2. **WASM** (`crates/iscc-wasm/`): Accept `Uint8Array` bytes (no filesystem); wasm-bindgen export +
    tests
3. **C FFI** (`crates/iscc-ffi/`): `iscc_gen_sum_code_v0(path, bits, wide)` + opaque result struct;
    update C test program
4. **Java** (`crates/iscc-jni/`): JNI bridge + `SumCodeResult` record + `genSumCodeV0` native
    method; mvn tests
5. **Go** (`packages/go/`): `GenSumCodeV0` + `SumCodeResult` struct; pure Go file I/O; tests
