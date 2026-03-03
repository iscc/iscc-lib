<!-- assessed-at: a49ac7d19dbf8c65b4023fd153c4bf69863929c8 -->

# Project State

## Status: DONE

## Phase: All target criteria met — CI green, no open issues

Issue #16 (CI feature matrix) was the last remaining gap. Iteration 15 added 5 steps to the `Rust`
CI job: 2 clippy checks (`--no-default-features`, `--all-features`) and 3 test runs
(`--no-default-features`, `--all-features`, `--no-default-features --features text-processing`). All
11 CI jobs pass on HEAD `a49ac7d`. `issues.md` is empty.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
    - `gen_meta_code_v0`, `json_to_data_url`, `META_TRIM_*` constants: `#[cfg(feature = "meta-code")]`
    - `gen_text_code_v0`, `text_clean`, `text_collapse`: `#[cfg(feature = "text-processing")]`
    - All other symbols always available
- `Cargo.toml` features: `default = ["meta-code"]`,
    `meta-code = ["text-processing", "dep:serde_json_canonicalizer"]`,
    `text-processing = ["dep:unicode-normalization", "dep:unicode-general-category"]` ✅
- `conformance_selftest()` always callable; meta/text sections gated internally ✅
- 314 tests with default features (258 unit + 31 streaming + 24 utils + 1 doctest) ✅
- `--no-default-features`: 250 tests pass ✅
- `--all-features`: passes ✅
- `--no-default-features --features text-processing`: 284 tests pass ✅
- `cargo clippy -p iscc-lib -- -D warnings` clean (all feature combos) ✅
- **CI feature matrix**: all 5 steps in `.github/workflows/ci.yml` verified and passing ✅

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `gen_sum_code_v0(path, bits=64, wide=False, add_units=False)` — `add_units` wired ✅
- `SumCodeResult.units: list[str] | None` annotation; `_lowlevel.pyi` stub updated ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(path, bits?, wide?, addUnits?)` wired ✅
- Auto-generated `index.d.ts` shows `units?: Array<string>` ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(data, bits?, wide?, add_units?)` accepts `Uint8Array` ✅
- `WasmSumCodeResult.units: Option<Vec<String>>` set when `add_units=true` ✅
- 79 wasm-bindgen tests pass; `cargo clippy -p iscc-wasm -- -D warnings` clean ✅

## C FFI

**Status**: met

- `iscc_gen_sum_code_v0(path, bits, wide, add_units: bool)` — 4-parameter signature ✅
- `iscc_IsccSumCodeResult.units: char **` — NULL-terminated array or `NULL` ✅
- 85 Rust tests + 65 C tests pass; `cargo clippy -p iscc-ffi -- -D warnings` clean ✅
- `iscc_sum.c` example compiles; `docs/howto/c-cpp.md` linked in nav ✅
- `cbindgen` header freshness checked in CI ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- `genSumCodeV0(String path, int bits, boolean wide, boolean addUnits)` — 4-parameter ✅
- `SumCodeResult.units: String[]` — `null` when `addUnits=false` ✅
- 65 Maven tests pass; `cargo clippy -p iscc-jni -- -D warnings` clean ✅

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go ✅; 154 Go tests pass; `go vet` clean ✅
- `GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool) (*SumCodeResult, error)` ✅
- `SumCodeResult.Units []string` — `nil` when false; `[]string{dataCode, instanceCode}` when true ✅
- Pure Go (no cgo); CGO_ENABLED=0 works ✅

## README

**Status**: met

- Public-facing polyglot README; all 6 bindings, CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present; all mention `gen_sum_code_v0` in API overview tables ✅

## Documentation

**Status**: met

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- All 5 doc files updated to 4-parameter `gen_sum_code_v0` signature ✅
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

**Status**: met

- **All 11 CI jobs SUCCESS** on HEAD `a49ac7d` — **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22604187637
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10, Python 3.14, Python (gate),
    Node.js, WASM, C FFI, Java, Go, Bench — all SUCCESS ✅
- `Rust` job now includes 5 feature matrix steps (2 clippy + 3 test combos) ✅
- `release.yml` has `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm, Maven,
    FFI) ✅
- Tag push `v*.*.*` triggers all publish jobs ✅
- v0.0.4 released to all registries; OIDC trusted publishing configured ✅
- `mise run version:sync` / `version:check` in CI ✅

## Next Milestone

**All target criteria met.** No open issues. CI fully green.

The project has reached its DONE state. Possible next directions (if the target is extended):

- Bump to v0.1.0 for a stable experimental release
- Add more howto guide content or interactive documentation pages
- Expand CI to test on macOS/Windows runners
- Performance regression tracking in CI
