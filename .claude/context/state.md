<!-- assessed-at: 58ab8d1 -->

# Project State

## Status: IN_PROGRESS

## Phase: Issue #21 fully resolved (docs updated); issue #16 (feature flags) is the only remaining work

Commit `58ab8d1` (review PASS) completed the documentation update for the 4-parameter
`gen_sum_code_v0` signature. All 5 affected doc files now show `add_units` in function signatures,
parameter tables, and code examples. Issue #21 is completely closed. The only remaining open issue
is #16 (feature flags for embedded/minimal builds), which is the sole blocker for DONE status.

## Rust Core Crate

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols present: 10 `gen_*_v0` functions, 4 text utilities, 4 algo primitives, 1
    soft hash, 2 encoding utilities, 3 codec operations, 5 constants, 2 streaming types, 1
    diagnostic
- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>`
    — 4-parameter signature with `add_units` gating the new `units` field ✅
- `SumCodeResult { iscc: String, datahash: String, filesize: u64, units: Option<Vec<String>> }` —
    `units` holds `[Data-Code ISCC, Instance-Code ISCC]` when `add_units: true` ✅
- 312 tests passing (258 unit + 31 streaming + 22 utils + 1 doctest) ✅
- `cargo clippy -p iscc-lib -- -D warnings` clean; `cargo fmt -p iscc-lib --check` clean ✅
- Benchmarks updated to pass 4-argument call to `gen_sum_code_v0` ✅

## Python Bindings

**Status**: met — `add_units`/`units` fully exposed to Python callers

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `gen_sum_code_v0(path, bits=64, wide=False, add_units=False)` — `add_units` properly wired through
    PyO3 wrapper and Python wrapper function ✅
- `SumCodeResult.units: list[str] | None` annotation in `__init__.py` ✅
- `_lowlevel.pyi` stub updated: `gen_sum_code_v0(..., add_units: bool = False)` with `units` in
    return-type docstring ✅
- 207 Python tests pass: 3 smoke tests cover `add_units=True`, default (no units), and
    attribute-access of `result.units` ✅
- `ty check` passes ✅; `cargo clippy -p iscc-py` clean ✅

## Node.js Bindings

**Status**: met — `add_units`/`units` fully exposed to JS callers

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(path, bits?, wide?, addUnits?)` — `add_units: Option<bool>` properly wired in
    NAPI layer; defaults to `false` when omitted ✅
- `NapiSumCodeResult.units: Option<Vec<String>>` — auto-generated `index.d.ts` shows
    `units?: Array<string>` ✅
- 135 mocha tests pass (132 existing + 3 new for add_units=true, default, content verification) ✅
- `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: met — `add_units`/`units` fully exposed to WASM callers

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(data, bits?, wide?, add_units?)` — `add_units: Option<bool>` parameter wired in
    inline WASM implementation (accepts `Uint8Array`/`&[u8]`, not path-based); defaults to `false`
    when `None` ✅
- `WasmSumCodeResult.units: Option<Vec<String>>` — set to `Some(vec![data_code, instance_code])`
    when `add_units=true`, `None` otherwise ✅
- 3 wasm-bindgen tests: `test_gen_sum_code_v0_units_enabled`, `_units_disabled`, `_units_content` ✅
- Total: 79 wasm-bindgen tests pass ✅
- `cargo clippy -p iscc-wasm -- -D warnings` clean ✅

## C FFI

**Status**: met — `add_units`/`units` fully exposed to C callers ✅

- `iscc_gen_sum_code_v0(path, bits, wide, add_units: bool)` — 4-parameter signature in both
    `iscc-ffi/src/lib.rs` and regenerated `crates/iscc-ffi/include/iscc.h` ✅
- `iscc_IsccSumCodeResult.units: char **` — NULL-terminated array of Data-Code + Instance-Code
    strings, or `NULL` when `add_units=false` ✅
- `iscc_free_sum_code_result()` frees `units` via existing `iscc_free_string_array()` ✅
- 85 Rust tests (82 existing + 3 new: units_enabled, units_disabled, units_content) ✅
- C test `test_iscc.c`: 65 passed, 0 failed — tests 27 and 28 cover add_units=true/false ✅
- `cargo clippy -p iscc-ffi -- -D warnings` clean; header freshness check passes ✅
- `iscc_sum.c` example still compiles ✅
- `docs/howto/c-cpp.md` linked in navigation ✅

## Java Bindings

**Status**: met — `add_units`/`units` fully exposed to Java callers ✅

- All 32 Tier 1 symbols via JNI ✅
- `genSumCodeV0(String path, int bits, boolean wide, boolean addUnits)` — 4-parameter signature in
    `IsccLib.java` (native declaration) ✅
- `SumCodeResult.java` has `String[] units` field — `null` when `addUnits=false`, otherwise
    `[Data-Code ISCC, Instance-Code ISCC]` ✅
- Rust JNI bridge (`crates/iscc-jni/src/lib.rs`) reuses `build_string_array` helper to convert
    `Some(Vec<String>)` → `jobjectArray`; `None` → `JObject::null()` ✅
- Constructor signature updated to `(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V` ✅
- 65 Maven tests pass (62 existing + 3 new: `genSumCodeV0UnitsEnabled`, `genSumCodeV0UnitsDisabled`,
    `genSumCodeV0UnitsContent`) ✅
- `cargo clippy -p iscc-jni -- -D warnings` clean ✅

## Go Bindings

**Status**: met — `add_units`/`units` fully exposed to Go callers ✅

- All 32 Tier 1 symbols via pure Go ✅; 154 Go tests pass; `go vet` clean ✅
- `GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool) (*SumCodeResult, error)` —
    4-parameter signature in `packages/go/code_sum.go` ✅
- `SumCodeResult.Units []string` — `nil` when `addUnits=false`; `[]string{dataCode, instanceCode}`
    when `addUnits=true` ✅
- 7 tests in `code_sum_test.go` (4 existing updated + 3 new: `TestGenSumCodeV0UnitsEnabled`,
    `TestGenSumCodeV0UnitsDisabled`, `TestGenSumCodeV0UnitsContent`) ✅
- `TestGenSumCodeV0UnitsContent` verifies units match separate `GenDataCodeV0` / `GenInstanceCodeV0`
    calls ✅

## README

**Status**: met

- Public-facing polyglot README; all 6 bindings, CI badge, registry badges ✅
- All 10 `gen_*_v0` functions listed in Implementors Guide (including `gen_sum_code_v0`) ✅
- Per-language installation instructions and quick-start code examples ✅
- ISCC architecture diagram and MainTypes table ✅

## Per-Crate READMEs

**Status**: met

- All 7 per-crate READMEs present with registry-specific install commands and quick-start examples ✅
- All 7 mention `gen_sum_code_v0` in their API overview tables ✅

## Documentation

**Status**: met — all `gen_sum_code_v0` references updated to 4-parameter signature ✅

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- Getting-started tutorial: 7 sections × 6 languages ✅
- `docs/rust-api.md` line 275:
    `pub fn gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool)` ✅
- `docs/architecture.md` line 131: `pub fn gen_sum_code_v0(path, bits, wide, add_units)` ✅
- `docs/c-ffi-api.md`: `add_units` parameter in table + `char **units` field in struct docs ✅
- `docs/howto/rust.md`: code example uses 4-argument call ✅
- `docs/howto/c-cpp.md`: both code examples use 4-argument call ✅
- Verified: `grep -n 'gen_sum_code_v0.*bits.*wide' docs/*.md docs/howto/*.md | grep -v add_units` —
    no matches ✅
- `docs/llms.txt` and `scripts/gen_llms_full.py` in place ✅
- All howto guides have Sum-Code subsections ✅; `docs/howto/c-cpp.md` linked in nav ✅
- `uv run zensical build` exits 0 ✅

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅; `bench_sum_code` updated to 4-arg call ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- pytest-benchmark comparison files ✅; speedup factors in `docs/benchmarks.md` ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **All 11 CI jobs SUCCESS** on latest push — **PASSING** ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22600934621
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check) —
    all SUCCESS ✅
- v0.0.4 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via `NPM_TOKEN` secret ✅
- `release.yml` has `build-ffi`/`publish-ffi` with 5-platform matrix; `workflow_dispatch` `ffi`
    boolean input ✅
- FFI publishing untested end-to-end (structural verification only)
- Open issue #16 (feature flags) blocks DONE status

## Next Milestone

**Issue #16 — Feature flags for embedded/minimal builds**

Issue #21 is fully resolved across all 7 bindings and docs. The only remaining open issue is #16:

Add Cargo feature flags (`meta-code`, `text-processing`) so embedded consumers can opt out of heavy
dependencies (~82K source lines for serde/unicode). Default behavior unchanged (all features on).
`conformance_selftest()` must adapt to available features (skip disabled code types). CI must test
`--all-features`, `--no-default-features`, and each feature individually.

Completing issue #16 is the final requirement before the project can reach DONE status.
