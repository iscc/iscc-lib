<!-- assessed-at: 80c59f72b8760b3385ecf023879e838cbfef1fcf -->

# Project State

## Status: IN_PROGRESS

## Phase: Issue #21 partial — Rust core units support done; binding exposure pending

Commit `80c59f7` (review pass) completed the Rust core half of issue #21: `gen_sum_code_v0` now
accepts `add_units: bool` and `SumCodeResult` has `units: Option<Vec<String>>`. All 6 binding crates
compile by hardcoding `false`, but none yet expose the `add_units` parameter or `units` return value
to their public APIs. Issue #21 remains open until all bindings are updated.

## Rust Core Crate

**Status**: met (32/32 Tier 1 symbols; all conformance tests pass)

- All 32 Tier 1 symbols present: 10 `gen_*_v0` functions, 4 text utilities, 4 algo primitives, 1
    soft hash, 2 encoding utilities, 3 codec operations, 5 constants, 2 streaming types, 1
    diagnostic
- `gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>`
    — 4-parameter signature with `add_units` gating the new `units` field ✅
- `SumCodeResult { iscc: String, datahash: String, filesize: u64, units: Option<Vec<String>> }` —
    `units` holds `[Data-Code ISCC, Instance-Code ISCC]` when `add_units: true` ✅
- 312 tests passing (258 unit + 31 streaming + 22 utils + 1 doctest), +2 new vs prior 310
    (`test_gen_sum_code_v0_units_enabled`, `test_gen_sum_code_v0_units_disabled`) ✅
- `cargo clippy -p iscc-lib -- -D warnings` clean; `cargo fmt -p iscc-lib --check` clean ✅
- Benchmarks updated to pass 4-argument call to `gen_sum_code_v0` ✅

## Python Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to Python callers

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `gen_sum_code_v0` hardcodes `add_units: false` — Python callers cannot request units ❌
- `units` field absent from `SumCodeResult` dict returned to Python ❌
- 204 Python tests pass (CI green) ✅

## Node.js Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to JS callers

- All 32 Tier 1 symbols exported ✅
- `gen_sum_code_v0` hardcodes `add_units: false` — JS callers cannot request units ❌
- `NapiSumCodeResult` has no `units` field ❌
- 132 mocha tests pass (CI green) ✅

## WASM Bindings

**Status**: partially met — compiles; `add_units`/`units` not yet exposed to WASM callers

- All 32 Tier 1 symbols exported ✅
- WASM has its own inline `gen_sum_code_v0` implementation (does not call
    `iscc_lib::gen_sum_code_v0`) — this means passing `false` was not needed, but
    `add_units`/`units` support still needs adding to the inline implementation ❌
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
- `docs/rust-api.md` still shows old 3-parameter signature for `gen_sum_code_v0` ❌ (noted in handoff
    — intentionally deferred until bindings are exposed)
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
- URL: https://github.com/iscc/iscc-lib/actions/runs/22592139384
- Jobs: Version consistency, Rust (fmt, clippy, test), Python 3.10 (ruff, pytest), Python 3.14
    (ruff, pytest), Python (ruff, pytest), Node.js (napi build, test), WASM (wasm-pack test), C FFI
    (cbindgen, gcc, test), Java (JNI build, mvn test), Go (go test, go vet), Bench (compile check)
- v0.0.4 released to all registries; OIDC trusted publishing for crates.io; Maven Central GPG
    configured; npm via `NPM_TOKEN` secret ✅
- `release.yml` has `build-ffi`/`publish-ffi` with 5-platform matrix; `workflow_dispatch` `ffi`
    boolean input ✅
- FFI publishing untested end-to-end (structural verification only)
- Open issue #16 (feature flags) blocks DONE status

## Next Milestone

**Issue #21 — Expose `add_units` in all binding APIs (priority: normal)**

The Rust core change is done. The next step is to expose `add_units: bool` and the `units` return
field across all six bindings. Recommended order:

1. **Python** (`iscc-py/src/lib.rs`): add `add_units: bool = False` param; include `units`
    (`Optional[List[str]]`) in `SumCodeResult` dict; update tests.
2. **Node.js** (`iscc-napi/src/lib.rs`): add optional `addUnits?: boolean` param; add
    `units?:  string[]` to `NapiSumCodeResult`; update tests.
3. **C FFI** (`iscc-ffi/src/lib.rs`): add `add_units: u8` param to `iscc_gen_sum_code_v0`; extend
    `IsccSumCodeResult` C struct with a `units` array (requires header regeneration); update tests.
4. **Java/JNI** (`iscc-jni/src/lib.rs`): add `boolean addUnits` param to JNI method; extend
    `SumCodeResult.java` with `units` field; update mvn tests.
5. **WASM** (`iscc-wasm/src/lib.rs`): add `addUnits?: boolean` param to inline implementation;
    extend `WasmSumCodeResult` with `units: Option<Vec<String>>`; update tests.
6. **Go** (`packages/go/code_sum.go`): add `addUnits bool` param; extend `SumCodeResult` with
    `Units []string`; update Go tests.

Also update `docs/rust-api.md` and `docs/architecture.md` to reflect the 4-parameter signature when
the bindings are updated.
