<!-- assessed-at: e894b6ca69b17ab1d1e0abda4ad19b2faee9d1c5 -->

# Project State

## Status: IN_PROGRESS

## Phase: Issue #21 partial — Python + Node.js bindings done; 4 bindings still need add_units/units exposure

Commit `e894b6c` (review PASS) completed the Node.js binding half of issue #21: `gen_sum_code_v0`
now accepts `add_units?: boolean` in the NAPI layer; `NapiSumCodeResult` has
`units?: Array<string>`; 3 new mocha tests verify enabled/disabled/content. Python was done the
previous iteration. Remaining bindings (WASM, C FFI, Java/JNI, Go) still hardcode `false` and do not
expose the parameter or field to their callers.

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

**Status**: met — `add_units`/`units` fully exposed to JS callers ✅ (completed this iteration)

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0(path, bits?, wide?, addUnits?)` — `add_units: Option<bool>` properly wired in
    NAPI layer; defaults to `false` when omitted ✅
- `NapiSumCodeResult.units: Option<Vec<String>>` — auto-generated `index.d.ts` shows
    `units?:   Array<string>` ✅
- 135 mocha tests pass (132 existing + 3 new for add_units=true, default, content verification) ✅
- `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to WASM callers

- All 32 Tier 1 symbols exported ✅
- WASM has its own inline `gen_sum_code_v0` implementation (does not call
    `iscc_lib::gen_sum_code_v0`) — `add_units`/`units` support still needs adding to the inline
    implementation ❌
- `WasmSumCodeResult` has no `units` field ❌
- 75 wasm-bindgen tests pass (CI green) ✅

## C FFI

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to C callers

- cbindgen generates valid C headers ✅; `iscc.h` committed ✅; CI freshness check ✅
- C example `crates/iscc-ffi/examples/iscc_sum.c` + `CMakeLists.txt` ✅
- `docs/howto/c-cpp.md` (433 lines) linked in navigation ✅
- `build-ffi`/`publish-ffi` jobs in `release.yml` with 5-platform matrix ✅
- `iscc_gen_sum_code_v0` hardcodes `add_units: false` — C callers cannot request units ❌
- `IsccSumCodeResult` C struct has no `units` field ❌
- CI "C FFI (cbindgen, gcc, test)" SUCCESS ✅

## Java Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to Java callers

- All 32 Tier 1 symbols via JNI ✅
- `genSumCodeV0` hardcodes `add_units: false` — Java callers cannot request units ❌
- `SumCodeResult.java` has no `units` field ❌
- 62 mvn tests pass (CI green) ✅

## Go Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to Go callers

- All 32 Tier 1 symbols via pure Go ✅; 151 Go tests pass; `go vet` clean ✅
- `SumCodeResult` struct has no `Units` field ❌
- `GenSumCodeV0` still has 3 parameters (path, bits, wide) — no `addUnits` param ❌
- Go uses its own pure Go implementation, not Rust FFI, so needs its own change

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

**Status**: partially met — docs reference old 3-parameter `gen_sum_code_v0` signature

- 17 pages deployed to lib.iscc.codes; all navigation sections complete ✅
- Getting-started tutorial: 7 sections × 6 languages ✅
- `docs/rust-api.md` still shows old 3-parameter signature for `gen_sum_code_v0` ❌ (deferred until
    all bindings are updated)
- `docs/architecture.md` still references old 3-parameter signature ❌ (same deferral)
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
- URL: https://github.com/iscc/iscc-lib/actions/runs/22594770894
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check)
- v0.0.4 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via `NPM_TOKEN` secret ✅
- `release.yml` has `build-ffi`/`publish-ffi` with 5-platform matrix; `workflow_dispatch` `ffi`
    boolean input ✅
- FFI publishing untested end-to-end (structural verification only)
- Open issues #16 (feature flags) and #21 (units in remaining 4 bindings) block DONE status

## Next Milestone

**Issue #21 — Expose `add_units` in remaining 4 binding APIs (priority: normal)**

Python and Node.js bindings are complete. Next step is WASM, then the remaining bindings.
Recommended order:

1. **WASM** (`iscc-wasm/src/lib.rs`): add `addUnits?: boolean` param to inline implementation;
    extend `WasmSumCodeResult` with `units: Option<Vec<String>>`; update wasm tests. Note: WASM has
    its own inline implementation not delegating to `iscc_lib::gen_sum_code_v0`.
2. **C FFI** (`iscc-ffi/src/lib.rs`): add `add_units: u8` param to `iscc_gen_sum_code_v0`; extend
    `IsccSumCodeResult` C struct with a `units` array (requires header regeneration); update tests.
3. **Java/JNI** (`iscc-jni/src/lib.rs`): add `boolean addUnits` param to JNI method; extend
    `SumCodeResult.java` with `units` field; update mvn tests.
4. **Go** (`packages/go/code_sum.go`): add `addUnits bool` param; extend `SumCodeResult` with
    `Units []string`; update Go tests.

After all 4 are done, update `docs/rust-api.md` and `docs/architecture.md` to reflect the
4-parameter signature and close issue #21.
