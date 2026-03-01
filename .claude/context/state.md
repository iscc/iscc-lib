<!-- assessed-at: 5eb6ba133c2cda81471a1176047c78d8ed01f4ec -->

# Project State

## Status: IN_PROGRESS

## Phase: New Tier 1 Symbols Required — gen_sum_code_v0 + META_TRIM_META Unimplemented

v0.0.3 was released (all registries) and CI is green on all jobs. However, the target was updated to
require **32** public Tier 1 symbols (up from 30): a new `gen_sum_code_v0` function and a new
`META_TRIM_META` constant are now required but not yet implemented in any crate or binding. Issues
#15 and #18 track these additions.

## Rust Core Crate

**Status**: partially met (30/32 Tier 1 symbols; 2 new symbols unimplemented)

- 30 existing Tier 1 symbols present and passing conformance (CI-verified)
- **MISSING**: `gen_sum_code_v0(path: &Path, bits: u32, wide: bool) -> IsccResult<SumCodeResult>` —
    single-pass file I/O for Data+Instance code (issue #15)
- **MISSING**: `META_TRIM_META: usize = 128_000` constant — payload size guard for
    `gen_meta_code_v0` (issue #18)
- **MISSING**: `SumCodeResult` struct with `iscc`, `datahash`, `filesize` fields
- **MISSING**: `gen_meta_code_v0` payload validation against `META_TRIM_META`
- CDC optimization merged (PR #17): unchecked indexing + 4x loop unrolling in `cdc.rs`;
    `bench_cdc_chunks` benchmark added
- `cargo test -p iscc-lib` CI-verified passing; `cargo clippy --workspace` clean

## Python Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META, SumCodeResult)

- 30/30 existing Tier 1 symbols accessible; all conformance vectors pass (CI-verified on 3.10 +
    3.14)
- `__all__` currently has 45 entries; needs `gen_sum_code_v0`, `SumCodeResult`, `META_TRIM_META`,
    `core_opts.meta_trim_meta`
- Python 3.14 CI added (matrix 3.10 + 3.14 both passing); `python` gate job added for stable branch
    protection
- **MISSING**: `gen_sum_code_v0` wrapper accepting `str | os.PathLike`
- **MISSING**: `SumCodeResult` class with dict + attribute access
- **MISSING**: `META_TRIM_META` constant and `core_opts.meta_trim_meta` attribute
- `iscc-lib 0.0.3` published to PyPI

## Node.js Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META)

- 30/30 existing Tier 1 symbols exported; 124 tests CI-verified passing
- **MISSING**: `gen_sum_code_v0` napi export
- **MISSING**: `META_TRIM_META` constant export
- `@iscc/lib 0.0.3` published to npm

## WASM Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META)

- 30/30 existing Tier 1 symbols exported; 69 wasm-bindgen tests CI-verified passing
- **MISSING**: `gen_sum_code_v0` wasm_bindgen export (note: path-based I/O may need special handling
    in WASM context)
- **MISSING**: `META_TRIM_META` constant getter
- `@iscc/wasm 0.0.3` published to npm

## C FFI

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META)

- 44 existing `extern "C"` functions; 77 Rust unit tests + C test program (23 cases) CI-verified
    passing
- **MISSING**: `iscc_gen_sum_code_v0` extern "C" function + memory management helpers for result
- **MISSING**: `iscc_meta_trim_meta()` getter function

## Java Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META)

- 32 existing `extern "system"` JNI functions; `IsccLibTest.java` (472 lines) CI-verified passing
- **MISSING**: `genSumCodeV0` JNI bridge + Java static native method
- **MISSING**: `META_TRIM_META` constant in `IsccLib.java`
- Maven Central external setup complete; end-to-end release untested

## Go Bindings

**Status**: partially met (missing gen_sum_code_v0, META_TRIM_META)

- 30/30 existing Tier 1 symbols present; 147 pure Go tests CI-verified passing (`CGO_ENABLED=0`)
- **MISSING**: `GenSumCodeV0(path string, bits uint32, wide bool)` function
- **MISSING**: `MetaTrimMeta` constant (128_000)
- **MISSING**: `SumCodeResult` struct

## README

**Status**: met

- Public-facing polyglot README (238 lines); all 6 bindings, all 9 `gen_*_v0` listed, CI badge,
    registry badges
- Will need update for `gen_sum_code_v0` when implemented (low priority — new function)

## Per-Crate READMEs

**Status**: met (for existing 30 symbols)

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples
- Will need `gen_sum_code_v0` mention when implemented

## Documentation

**Status**: met (for existing features)

- 16 pages deployed to lib.iscc.codes; all navigation sections complete
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place
- Getting-started tutorial: 7 sections × 6 languages; all howto guides complete
- Benchmarks page updated; `docs/ecosystem.md` updated for iscc-core-ts v1.0.0 stable
- Will need `gen_sum_code_v0` and `META_TRIM_META` mentions when implemented

## Benchmarks

**Status**: met

- Criterion benchmarks for all 9 `gen_*_v0` + `bench_data_hasher_streaming` + `bench_cdc_chunks`
    (4KB/64KB/1MB)
- pytest-benchmark comparison: `benchmarks/python/bench_iscc_core.py` and `bench_iscc_lib.py`
- Speedup factors published in `docs/benchmarks.md`
- `Bench (compile check)` CI job verifies all benchmark targets compile

## CI/CD and Publishing

**Status**: met (for existing features)

- CI workflow upgraded: `python-test` matrix (3.10, 3.14), `python` gate job for stable branch
    protection
- **11 CI run records** per push: version-check, Rust, Python 3.10, Python 3.14, Python (gate),
    Node.js, WASM, C FFI, Java, Go, Bench — **all SUCCESS**
- Latest CI run: **PASSING** — https://github.com/iscc/iscc-lib/actions/runs/22545587495
- v0.0.3 released: tags `v0.0.3` and `packages/go/v0.0.3`; all registries published
- OIDC trusted publishing for crates.io; Maven Central GPG/Sonatype configured; npm via NPM_TOKEN

## Next Milestone

**Implement the two new Tier 1 symbols across all 7 bindings:**

1. Add `META_TRIM_META: usize = 128_000` constant to Rust core (`crates/iscc-lib/src/lib.rs`)
2. Add `gen_meta_code_v0` payload validation in Rust core (pre-decode + post-decode checks)
3. Add `gen_sum_code_v0` + `SumCodeResult` to Rust core with single-pass file I/O
4. Propagate both new symbols to all 6 language bindings (Python, Node.js, WASM, C FFI, Java, Go)
5. Add tests for `META_TRIM_META` boundary cases and `gen_sum_code_v0` correctness
