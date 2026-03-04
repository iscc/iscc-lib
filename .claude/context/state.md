<!-- assessed-at: b5b46880c0a02ba8f2d08ab9ffa0f9a20a87bf91 -->

# Project State

## Status: IN_PROGRESS

## Phase: Go validation fixed; release smoke tests and README badges remain

The Go `AlgCdcChunks` validation gap is now closed: `packages/go/cdc.go` validates
`avgChunkSize < 2` with a returned `error`, mirroring the Rust core pattern, with an internal
`algCdcChunksUnchecked` for callers that guarantee valid input. All 12 CI jobs pass (run
22662906194). Two `normal`-priority gaps remain: release workflow smoke tests and missing registry
badges in the global README.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating ✅
- `alg_cdc_chunks` public API returns `IsccResult<Vec<&[u8]>>` — validates `avg_chunk_size < 2` with
    `IsccError::InvalidInput` ✅
- `alg_cdc_chunks_unchecked` added as `pub(crate)` for internal callers (`gen_data_code_v0`,
    `DataHasher::update`) where size is the compile-time constant `DATA_AVG_CHUNK_SIZE` ✅
- `docs/howto/rust.md` updated to reflect `IsccResult<Vec<&[u8]>>` return type ✅
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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22662906194 ✅

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
- `AlgCdcChunks` now validates `avgChunkSize < 2` — returns `error` when `< 2`, else delegates to
    `algCdcChunksUnchecked` for internal callers that guarantee valid input ✅
- `TestCdcChunksInvalidAvgChunkSize` test added covering avgChunkSize=0, 1 (error), 2 (OK) ✅
- `docs/howto/go.md` updated to reflect `([][]byte, error)` return type ✅
- 155 Go tests pass; `go vet` clean ✅

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat) ✅
- `crates/iscc-rb/LICENSE` (Apache 2.0) added ✅
- `alg_cdc_chunks` propagates `IsccResult` via `map_err(to_magnus_err)` ✅
- **All 32 of 32 Tier 1 symbols** exposed (10 gen, 4 text, 6 codec/diagnostic, 5 algo, 5 constants,
    2 streaming types) ✅
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

- Public-facing polyglot README exists; CI badge, registry badges for Rust, Python, npm, Go ✅
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- Ruby install instructions and quickstart present ✅
- **Gap**: Missing registry badges for RubyGems (`iscc-rb`), Maven Central (`iscc-jni`), and npm
    `@iscc/wasm` — filed as `normal` priority issue
- **Gap**: C#, C++, Swift, Kotlin sections not present (target requires all 4; all `low` priority)

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
- `docs/howto/go.md` updated: `AlgCdcChunks` return type corrected to `([][]byte, error)` ✅
- `docs/ruby-api.md` API reference page (781 lines) ✅
- **Gap**: Target requires C#, C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: partially met

- **ALL PASSING** — latest CI run 22662906194: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22662906194
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (gate), Node.js, WASM, C FFI,
    Java, Go, Bench, Ruby — all SUCCESS ✅
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- `release.yml` fixed: `cargo test --workspace --exclude iscc-rb` ✅
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1` ✅
- RubyGems account setup complete: account created, `GEM_HOST_API_KEY` secret added to GitHub ✅
- **Gap**: Target requires release workflow smoke-tests (install built artifact, run conformance
    tests) before each publish step — none implemented yet (`normal` priority)
- **Gap**: Target requires CI jobs for C#, C++, Swift, Kotlin (all `low` priority; none started)

## Next Milestone

Two `normal`-priority issues remain actionable by the CID loop (CI is green — no CI fix required):

1. **Add registry badges to global README** (`README.md`): add `img.shields.io/gem/v/iscc-rb`
    (RubyGems), Maven Central version badge for `iscc-jni`, and `img.shields.io/npm/v/@iscc/wasm`
    badge — simple targeted edits to `README.md`.

2. **Add release smoke tests** (`release.yml`): add `test-<artifact>` jobs between each `build-*`
    and `publish-*` step (6 pipelines: PyPI, npm-lib, npm-wasm, RubyGems, Maven, FFI). Each job
    installs the built artifact and runs the binding's conformance suite before gating publish.
