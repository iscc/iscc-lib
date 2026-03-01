<!-- assessed-at: 59626a7c352410f3c79ffa7bf432c71c0491f523 -->

# Project State

## Status: IN_PROGRESS

## Phase: Binding Propagation — META_TRIM_META + gen_sum_code_v0 Pending in All 6 Bindings

Iteration 1 completed: `META_TRIM_META: usize = 128_000` and `gen_meta_code_v0` payload validation
(pre-decode + post-decode) are now implemented in the Rust core with 4 tests (303 total, all
passing). CI is fully green (11/11 jobs). The Rust core now has 31/32 Tier 1 symbols — only
`gen_sum_code_v0` + `SumCodeResult` remain unimplemented. All 6 language bindings still need
`META_TRIM_META` exported, and all 6 still need `gen_sum_code_v0` when that is added to the Rust
core.

## Rust Core Crate

**Status**: partially met (31/32 Tier 1 symbols; gen_sum_code_v0 unimplemented)

- 31 Tier 1 symbols present: all 9 existing `gen_*_v0`, 4 text utilities, 4 algo primitives, 1 soft
    hash, 2 encoding utilities, 3 codec operations, **5 constants** (META_TRIM_NAME,
    META_TRIM_DESCRIPTION, **META_TRIM_META now added ✅**), 2 streaming types, 1 diagnostic
- `META_TRIM_META: usize = 128_000` added to `crates/iscc-lib/src/lib.rs` (CI-verified)
- `gen_meta_code_v0` pre-decode + post-decode payload validation implemented and tested
- 303 tests passing (249 lib + 31 integration + 22 utils + 1 doctest); `cargo clippy` clean
- **MISSING**: `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` —
    single-pass file I/O for Data+Instance code (issue #15)
- **MISSING**: `SumCodeResult` struct with `iscc`, `datahash`, `filesize` fields

## Python Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 30/30 existing Tier 1 symbols accessible; all conformance vectors pass (CI-verified on 3.10 +
    3.14)
- `__all__` currently has 45 entries; needs `gen_sum_code_v0`, `SumCodeResult`, `META_TRIM_META`,
    `core_opts.meta_trim_meta`
- Python 3.10 and 3.14 CI passing; `python` gate job passing
- **MISSING**: `META_TRIM_META` constant export (Rust core now has it; binding not updated)
- **MISSING**: `gen_sum_code_v0` wrapper accepting `str | os.PathLike`
- **MISSING**: `SumCodeResult` class with dict + attribute access
- **MISSING**: `core_opts.meta_trim_meta` attribute for iscc-core parity

## Node.js Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 30/30 existing Tier 1 symbols exported; 124 tests CI-verified passing
- **MISSING**: `META_TRIM_META` constant export (Rust core now has it; binding not updated)
- **MISSING**: `gen_sum_code_v0` napi export
- `@iscc/lib 0.0.3` on npm

## WASM Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 30/30 existing Tier 1 symbols exported; 69 wasm-bindgen tests CI-verified passing
- **MISSING**: `META_TRIM_META` constant getter (Rust core now has it; binding not updated)
- **MISSING**: `gen_sum_code_v0` wasm_bindgen export (path-based I/O needs special WASM handling)
- `@iscc/wasm 0.0.3` on npm

## C FFI

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 44 existing `extern "C"` functions; 77 Rust unit tests + C test program (23 cases) CI-verified
    passing
- **MISSING**: `iscc_meta_trim_meta()` getter function (Rust core now has it; binding not updated)
- **MISSING**: `iscc_gen_sum_code_v0` extern "C" function + memory management helpers for result

## Java Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 32 existing `extern "system"` JNI functions; `IsccLibTest.java` CI-verified passing
- **MISSING**: `META_TRIM_META` constant in `IsccLib.java` (Rust core now has it; binding not
    updated)
- **MISSING**: `genSumCodeV0` JNI bridge + Java static native method
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META export)

- 30/30 existing Tier 1 symbols present; 147 pure Go tests CI-verified passing (`CGO_ENABLED=0`)
- **MISSING**: `MetaTrimMeta` constant (128_000) (Rust core now has it; Go binding not updated)
- **MISSING**: `GenSumCodeV0(path string, bits uint32, wide bool)` function
- **MISSING**: `SumCodeResult` struct in Go

## README

**Status**: met

- Public-facing polyglot README (238 lines); all 6 bindings, all 9 `gen_*_v0` listed, CI badge,
    registry badges
- Will need update for `gen_sum_code_v0` and `META_TRIM_META` when bindings are propagated

## Per-Crate READMEs

**Status**: met (for existing 30 symbols)

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- Will need `gen_sum_code_v0` and `META_TRIM_META` mentions when implemented in bindings

## Documentation

**Status**: met (for existing features)

- 16 pages deployed to lib.iscc.codes; all navigation sections complete
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place
- Getting-started tutorial: 7 sections × 6 languages; all howto guides complete
- Benchmarks page updated; `docs/ecosystem.md` current
- Will need `gen_sum_code_v0` and `META_TRIM_META` mentions when bindings are propagated

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
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22548036633
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench — all success
- v0.0.3 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via NPM_TOKEN

## Next Milestone

**Propagate META_TRIM_META to all 6 bindings, then implement gen_sum_code_v0 across all crates:**

1. Export `META_TRIM_META = 128_000` in all 6 binding crates (Python, Node.js, WASM, C FFI, Java,
    Go) — each needs the constant + a test asserting value equals 128,000
2. Python additionally needs `core_opts.meta_trim_meta` attribute for iscc-core parity
3. After META_TRIM_META is propagated: implement `gen_sum_code_v0` + `SumCodeResult` in Rust core
    (issue #15), then propagate to all 6 bindings
