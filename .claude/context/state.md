<!-- assessed-at: db921b97452170c799544c90880fbaf73321a593 -->

# Project State

## Status: IN_PROGRESS

## Phase: C#/.NET bindings elevated to normal priority; v0.2.0 released

v0.2.0 released successfully across all 8 registries including RubyGems (now via OIDC trusted
publishing). C#/.NET bindings priority was elevated from `low` to `normal` by human directive — this
is now the CID loop's next implementation target. All other normal-priority gaps remain resolved
with CI green (12/12 jobs, latest run 22708331786).

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
- `--features conformance` added to `build-wasm` release job so `conformance_selftest` is exported ✅
- `WASM (wasm-pack test)` = SUCCESS in CI run 22708331786 ✅

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
- All 32 of 32 Tier 1 symbols exposed ✅
- 111 Minitest tests (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance ✅
- `bundle exec rake compile` builds in release profile ✅
- Dedicated `ruby` CI job — runs standardrb, clippy, compile, and test ✅
- `docs/howto/ruby.md` (422 lines) ✅; `docs/ruby-api.md` (781 lines — all 32 symbols) ✅
- `zensical.toml` Reference section: "Ruby API" nav entry ✅
- Root `README.md` Ruby section (install tab + quickstart) ✅
- `crates/iscc-rb/CLAUDE.md` added with detailed cross-compilation guidance ✅
- Cross-compilation fixes for v0.2.0: Rakefile gemspec, native loader path, Gemfile.lock symlink ✅
- RubyGems publish switched to OIDC trusted publishing (no API key needed) ✅

## C# / .NET Bindings

**Status**: not started — **NORMAL PRIORITY** (CID loop must address)

- Priority elevated from `low` to `normal` by human directive (commit `db921b9`) ✅
- **No code exists**: `packages/dotnet/` does not exist; no `csbindgen` integration; no CI job
- Issue in `issues.md` with full implementation scope:
    - `packages/dotnet/` — .csproj targeting .NET 8+, csbindgen P/Invoke, xUnit conformance tests
    - DevContainer: .NET SDK 8+ needed in Dockerfile
    - CI: `dotnet` job (`dotnet build` + `dotnet test`)
    - Release: `nuget` boolean input, NuGet publish via OIDC or API key
    - Docs: `docs/howto/dotnet.md`, README C# section
    - Version sync target needed for .NET project

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
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples ✅
- Ruby install instructions and quickstart present ✅
- **Gap**: Missing C#, C++, Swift, Kotlin install + quickstart sections (C# now `normal` priority)
- **Gap**: Language logos/icons not added yet (C++/Swift/Kotlin `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages including `crates/iscc-rb/README.md` ✅
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none of these directories exist yet (C# now `normal`, rest `low`)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete ✅
- 8 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, ruby.md
    ✅
- `docs/ruby-api.md` API reference page (781 lines) ✅; `docs/c-ffi-api.md` ✅
- **Gap**: Target requires C# how-to guide (`normal` priority; not started)
- **Gap**: Target requires C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions ✅
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks ✅
- `Bench (compile check)` CI job SUCCESS ✅

## CI/CD and Publishing

**Status**: met

- **ALL PASSING** — latest CI run 22708331786: all **12 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22708331786
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest), Node.js, WASM, C
    FFI, Java, Go, Bench, Ruby — all SUCCESS ✅
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- **6 smoke test jobs implemented** — each gates its publish job ✅
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1` ✅
- **RubyGems publish switched to OIDC** trusted publishing via
    `rubygems/configure-rubygems-credentials@main` ✅ (replaces `GEM_HOST_API_KEY` — no long-lived
    API key needed, consistent with crates.io + PyPI)
- v0.2.0 released successfully across all 8 registries ✅
- **Gap**: Target requires CI jobs for C# (now `normal` priority), C++, Swift, Kotlin (rest `low`)

## Next Milestone

**C#/.NET bindings** — priority elevated to `normal` by human directive; CID loop must now address
this gap.

Full implementation scope (from `issues.md`):

1. `packages/dotnet/` package: .csproj (.NET 8+), `csbindgen`-generated P/Invoke wrappers from
    `iscc.h`, idiomatic C# wrapper (PascalCase, exceptions, Stream support), xUnit conformance
    tests against `data.json`, NuGet packaging spec
2. DevContainer: add .NET SDK 8+ to Dockerfile
3. CI (`ci.yml`): add `dotnet` job — `dotnet build` + `dotnet test`
4. Release (`release.yml`): `nuget` boolean input, build/pack NuGet with native libs for 5
    platforms, publish via `dotnet nuget push` (OIDC or API key), idempotency check
5. Version sync: add .NET project version to sync targets
6. Documentation: `docs/howto/dotnet.md`, README C# install/quickstart section

Account setup for NuGet is a manual human action (register nuget.org, reserve `Iscc.Lib`, configure
OIDC trusted publisher).
