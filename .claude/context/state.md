<!-- assessed-at: a6a241f4a09a7a8203b26cf20ccf354b58950500 -->

# Project State

## Status: IN_PROGRESS

## Phase: Issue #16 — conformance_selftest adaptation done; CI feature matrix remains

Commit `0736cce` (review PASS at `a6a241f`) made `conformance_selftest()` always callable by
removing the module-level `#[cfg(feature = "meta-code")]` gate and adding granular `#[cfg]` guards
on the individual `run_meta_tests`/`run_text_tests` functions. One sub-task remains: the CI workflow
does not yet test `--no-default-features`, `--all-features`, or per-feature combinations. Adding
those steps is a YAML-only change that closes issue #16.

## Rust Core Crate

**Status**: partially met — conformance_selftest adaptation complete; CI feature matrix absent

- All 32 Tier 1 symbols present with correct feature-gating:
    - `gen_meta_code_v0`, `json_to_data_url`, `META_TRIM_*` constants: `#[cfg(feature = "meta-code")]`
    - `gen_text_code_v0`, `text_clean`, `text_collapse`: `#[cfg(feature = "text-processing")]`
    - All other symbols always available (no feature gate required)
- `Cargo.toml` features: `default = ["meta-code"]`,
    `meta-code = ["text-processing", "dep:serde_json_canonicalizer"]`,
    `text-processing = ["dep:unicode-normalization", "dep:unicode-general-category"]` ✅
- Optional deps: `serde_json_canonicalizer`, `unicode-normalization`, `unicode-general-category` ✅
- ✅ `conformance_selftest()` is **always callable** — `pub mod conformance;` and
    `pub use conformance::conformance_selftest;` in `lib.rs` have no `#[cfg]` gate
- ✅ Inside `conformance_selftest()`: meta-code section gated `#[cfg(feature = "meta-code")]`, text
    section gated `#[cfg(feature = "text-processing")]`; remaining 7 sections always run
- 314 tests with default features (258 unit + 31 streaming + 24 utils + 1 doctest) ✅
- `--no-default-features`: 250 tests pass; conformance_selftest runs 7 of 9 sections ✅
- `--no-default-features --features text-processing`: 284 tests pass; 8 of 9 sections ✅
- `cargo clippy -p iscc-lib -- -D warnings` clean (all feature combos) ✅
- ❌ **CI does not test feature combinations** — no `--no-default-features`, `--all-features`, or
    per-feature steps in `.github/workflows/ci.yml`

## Python Bindings

**Status**: met — `add_units`/`units` fully exposed to Python callers

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `gen_sum_code_v0(path, bits=64, wide=False, add_units=False)` — `add_units` properly wired ✅
- `SumCodeResult.units: list[str] | None` annotation; `_lowlevel.pyi` stub updated ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅

## Node.js Bindings

**Status**: met — `add_units`/`units` fully exposed to JS callers

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(path, bits?, wide?, addUnits?)` — `add_units: Option<bool>` properly wired ✅
- Auto-generated `index.d.ts` shows `units?: Array<string>` ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: met — `add_units`/`units` fully exposed to WASM callers

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(data, bits?, wide?, add_units?)` — accepts `Uint8Array`/`&[u8]` (not path-based)
    ✅
- `WasmSumCodeResult.units: Option<Vec<String>>` — set when `add_units=true` ✅
- 79 wasm-bindgen tests pass; `cargo clippy -p iscc-wasm -- -D warnings` clean ✅

## C FFI

**Status**: met — `add_units`/`units` fully exposed to C callers ✅

- `iscc_gen_sum_code_v0(path, bits, wide, add_units: bool)` — 4-parameter signature ✅
- `iscc_IsccSumCodeResult.units: char **` — NULL-terminated array or `NULL` ✅
- 85 Rust tests + 65 C tests pass; `cargo clippy -p iscc-ffi -- -D warnings` clean ✅
- `iscc_sum.c` example compiles; `docs/howto/c-cpp.md` linked in nav ✅

## Java Bindings

**Status**: met — `add_units`/`units` fully exposed to Java callers ✅

- All 32 Tier 1 symbols via JNI ✅
- `genSumCodeV0(String path, int bits, boolean wide, boolean addUnits)` — 4-parameter ✅
- `SumCodeResult.units: String[]` — `null` when `addUnits=false` ✅
- 65 Maven tests pass; `cargo clippy -p iscc-jni -- -D warnings` clean ✅

## Go Bindings

**Status**: met — `add_units`/`units` fully exposed to Go callers ✅

- All 32 Tier 1 symbols via pure Go ✅; 154 Go tests pass; `go vet` clean ✅
- `GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool) (*SumCodeResult, error)` ✅
- `SumCodeResult.Units []string` — `nil` when false; `[]string{dataCode, instanceCode}` when true ✅

## README

**Status**: met

- Public-facing polyglot README; all 6 bindings, CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present; all mention `gen_sum_code_v0` in API overview tables ✅

## Documentation

**Status**: met — all `gen_sum_code_v0` references updated to 4-parameter signature ✅

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- All 5 doc files updated: `docs/rust-api.md`, `docs/architecture.md`, `docs/c-ffi-api.md`,
    `docs/howto/rust.md`, `docs/howto/c-cpp.md` — all show `add_units` parameter ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- All howto guides have Sum-Code subsections ✅
- `uv run zensical build` exits 0 ✅

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions; `bench_sum_code` uses 4-arg call ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- pytest-benchmark comparison files; speedup factors in `docs/benchmarks.md` ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **All 11 CI jobs SUCCESS** on latest push — **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22603382763
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10, Python 3.14, Python (gate),
    Node.js, WASM, C FFI, Java, Go, Bench — all SUCCESS ✅
- v0.0.4 released to all registries; OIDC trusted publishing configured ✅
- ❌ **CI does not test feature combinations**: no `--no-default-features`, `--all-features`, or
    per-feature steps in `.github/workflows/ci.yml` — required by issue #16

## Next Milestone

**Issue #16 — Final sub-task: CI feature matrix**

Add steps to the existing `Rust (fmt, clippy, test)` job in `.github/workflows/ci.yml` to run:

- `cargo test -p iscc-lib --no-default-features`
- `cargo test -p iscc-lib --all-features`
- `cargo test -p iscc-lib --no-default-features --features text-processing`

This is a YAML-only change. No Rust code modifications needed. Completing this closes issue #16 and
unblocks DONE status.
