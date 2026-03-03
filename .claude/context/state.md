<!-- assessed-at: 38d0eaf2964535f3354d70a42c54f0471afaf440 -->

# Project State

## Status: IN_PROGRESS

## Phase: Issue #16 partially resolved — feature flag gating done; conformance adaptation + CI matrix remain

Commits `3f3606f`/`38d0eaf` (review PASS) completed the feature flag definitions and code gating for
issue #16. Three deps made optional, all code and tests properly gated behind
`#[cfg(feature = "...")]`. Two sub-tasks remain: (1) `conformance_selftest()` must work without
`meta-code` by skipping disabled code types rather than being completely absent; (2) CI must test
`--no-default-features` and per-feature combinations.

## Rust Core Crate

**Status**: partially met — feature flags done; conformance selftest adaptation incomplete

- All 32 Tier 1 symbols present with correct feature-gating:
    - `gen_meta_code_v0`, `json_to_data_url`, `META_TRIM_*` constants: `#[cfg(feature = "meta-code")]`
    - `gen_text_code_v0`, `text_clean`, `text_collapse`: `#[cfg(feature = "text-processing")]`
    - All other symbols always available (no feature gate required)
- `Cargo.toml` features: `default = ["meta-code"]`,
    `meta-code = ["text-processing",   "dep:serde_json_canonicalizer"]`,
    `text-processing = ["dep:unicode-normalization",   "dep:unicode-general-category"]` ✅
- Optional deps: `serde_json_canonicalizer`, `unicode-normalization`, `unicode-general-category` ✅
- 314 tests with default features (258 unit + 31 streaming + 24 utils + 1 doctest) ✅
- `--no-default-features`: 249 tests pass ✅; `--features text-processing` w/ no-defaults: 283 ✅
- `cargo clippy -p iscc-lib -- -D warnings` clean (all feature combos) ✅
- ❌ **`conformance_selftest()` is entirely absent when `meta-code` is disabled** — the whole
    `conformance` module is gated behind `#[cfg(feature = "meta-code")]`; it does not gracefully
    skip disabled code types as issue #16 requires

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
- URL: https://github.com/iscc/iscc-lib/actions/runs/22602425198
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10, Python 3.14, Python (gate),
    Node.js, WASM, C FFI, Java, Go, Bench — all SUCCESS ✅
- v0.0.4 released to all registries; OIDC trusted publishing configured ✅
- ❌ **CI does not test feature combinations**: no `--no-default-features`, `--all-features`, or
    per-feature jobs — required by issue #16
- Open issue #16 (partially resolved) blocks DONE status

## Next Milestone

**Issue #16 — Remaining two sub-tasks:**

1. **`conformance_selftest()` adaptation**: Currently the entire `conformance` module is gated
    behind `#[cfg(feature = "meta-code")]`, making `conformance_selftest()` completely absent
    without that feature. The function must always be callable but skip code types whose feature is
    disabled (e.g., skip `gen_meta_code_v0` tests when `meta-code` is off, skip `gen_text_code_v0`
    when `text-processing` is off).

2. **CI feature matrix**: Add jobs (or steps within the Rust job) to run:
    `cargo test -p iscc-lib --no-default-features`, `cargo test -p iscc-lib --all-features`,
    `cargo test -p iscc-lib --no-default-features --features text-processing`

Completing these two items closes issue #16 and unblocks DONE status.
