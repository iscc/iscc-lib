<!-- assessed-at: ad7400df7d5aa908450cfbb9fae851459434bb1e -->

# Project State

## Status: IN_PROGRESS

## Phase: Registry badges resolved; release smoke tests remain as sole normal-priority gap

All 7 registry badges (Rust, Python, Ruby, Maven, Go, npm @iscc/lib, npm @iscc/wasm) are now in the
global README. All 12 CI jobs pass (run 22664053215). One `normal`-priority gap remains: release
smoke tests before publish steps in `release.yml`. C#, C++, Swift, and Kotlin bindings are not
started and tracked as `low`-priority issues (CID loop skips them).

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `alg_cdc_chunks` public API returns `IsccResult<Vec<&[u8]>>` — validates `avg_chunk_size < 2` ✅
- `alg_cdc_chunks_unchecked` as `pub(crate)` for internal callers ✅
- `data.json` at iscc-core v1.3.0 (50 total vectors) ✅
- Rust conformance assertion: `assert_eq!(tested, 20, ...)` ✅
- 316 tests pass with default features ✅
- Feature matrix CI (5 steps) passed in latest green run ✅

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries) ✅
- `alg_cdc_chunks` propagates `IsccResult` from Rust core via `PyResult` ✅
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean ✅

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported ✅
- `alg_cdc_chunks` propagates `IsccResult` error from Rust core ✅
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean ✅

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]` ✅
- `alg_cdc_chunks` maps `IsccResult` to `JsError` ✅
- `wasm-opt` upgraded from `-O` to `-O3` for max runtime performance ✅
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20` ✅
- `WASM (wasm-pack test)` = SUCCESS in CI run 22664053215 ✅

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass (per last green CI run) ✅
- `iscc_alg_cdc_chunks` propagates `IsccResult` error via null return ✅
- `cbindgen` header freshness check in CI passed ✅

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI ✅
- `AlgCdcChunks` JNI validates `avgChunkSize < 2` with `IllegalArgumentException` ✅
- 65 Maven tests pass (per last green CI run) ✅

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go ✅
- `AlgCdcChunks` validates `avgChunkSize < 2` — returns `error`, delegates to
    `algCdcChunksUnchecked` for internal callers ✅
- `TestCdcChunksInvalidAvgChunkSize` test covers avgChunkSize=0, 1 (error), 2 (OK) ✅
- `docs/howto/go.md` updated to reflect `([][]byte, error)` return type ✅
- 155 Go tests pass; `go vet` clean ✅

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
- All 32 of 32 Tier 1 symbols exposed (10 gen, 4 text, 6 codec/diagnostic, 5 algo, 5 constants, 2
    streaming types) ✅
- 111 Minitest tests (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- Dedicated `ruby` CI job — runs standardrb, clippy, compile, and test ✅
- `docs/howto/ruby.md` (422 lines) ✅; `docs/ruby-api.md` (781 lines — all 32 symbols) ✅
- `zensical.toml` Reference section: "Ruby API" nav entry ✅
- Root `README.md` Ruby section (install tab + quickstart) ✅
- Standard Ruby linting fully configured ✅
- RubyGems.org account created, `GEM_HOST_API_KEY` secret configured in GitHub ✅

## C# / .NET Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/dotnet/` does not exist; no `csbindgen` integration; no CI job

## C++ Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/cpp/` does not exist; no `iscc.hpp`; no vcpkg/Conan manifests

## Swift Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips) ✅
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (not
    started)

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, all 7 registry badges present ✅
    - Rust (crates.io), Python (PyPI), Ruby (RubyGems), Java (Maven Central), Go (pkg.go.dev), Node.js
        (npm @iscc/lib), WASM (npm @iscc/wasm)
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- Ruby install instructions and quickstart present ✅
- **Gap**: Missing C#, C++, Swift, Kotlin install + quickstart sections (target requires all 4; all
    `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages including `crates/iscc-rb/README.md` (93 lines)
    ✅
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet (all `low` priority)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 8 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, ruby.md
    ✅
- `docs/ruby-api.md` API reference page (781 lines) ✅; `docs/c-ffi-api.md` ✅
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22664053215: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22664053215
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench, Ruby — all SUCCESS ✅
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1` ✅
- RubyGems account setup complete: account created, `GEM_HOST_API_KEY` secret added to GitHub ✅
- **Gap**: Target requires release workflow smoke-tests (install built artifact, run conformance
    tests) before each publish step — none implemented yet (`normal` priority)
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)

## Next Milestone

One `normal`-priority issue remains actionable by the CID loop (CI is green — no CI fix required):

**Add release smoke tests** (`release.yml`): add `test-<artifact>` jobs between each `build-*` and
`publish-*` step (6 pipelines: PyPI, npm-lib, npm-wasm, RubyGems, Maven, FFI). Each job installs the
built artifact and runs the binding's conformance suite before gating publish. The
`publish-crates-io` pipeline already runs `cargo test` and does not need a smoke test.

After smoke tests are implemented, the only remaining gaps are `low`-priority bindings (C#, C++,
Swift, Kotlin). At that point, consider creating a PR from `develop` → `main` for a stable release.
