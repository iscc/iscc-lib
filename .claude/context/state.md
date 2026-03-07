<!-- assessed-at: 109686fcb90d038b4254d8e01ab6ff6d72ce30c7 -->

# Project State

## Status: IN_PROGRESS

## Phase: C# bindings — API surface complete; docs + NuGet publish pipeline pending

v0.2.0 released across all 8 registries. The streaming `Finalize()` return types are now complete:
`IsccDataHasher.Finalize()` returns `DataCodeResult` and `IsccInstanceHasher.Finalize()` returns
`InstanceCodeResult`, matching the structured return type pattern across the entire .NET API
surface. All 91 tests pass and CI is fully green (run 22802326852). Remaining C# gaps: documentation
(`docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C# section) and NuGet publish pipeline.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating
- `alg_cdc_chunks` public API returns `IsccResult<Vec<&[u8]>>` — validates `avg_chunk_size < 2`
- `alg_cdc_chunks_unchecked` as `pub(crate)` for internal callers
- `data.json` at iscc-core v1.3.0 (50 total vectors)
- Rust conformance assertion: `assert_eq!(tested, 20, ...)`
- 316 tests pass with default features
- Feature matrix CI (5 steps) passed in latest green run

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via `__all__` (48 entries)
- `alg_cdc_chunks` propagates `IsccResult` from Rust core via `PyResult`
- 207 Python tests pass; `ty check` passes; `cargo clippy -p iscc-py` clean

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported
- `alg_cdc_chunks` propagates `IsccResult` error from Rust core
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]`
- `alg_cdc_chunks` maps `IsccResult` to `JsError`
- `wasm-opt` upgraded from `-O` to `-O3` for max runtime performance
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20`
- `--features conformance` added to `build-wasm` release job so `conformance_selftest` is exported
- WASM CI job = SUCCESS in run 22802326852

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass (per last green CI run)
- `iscc_alg_cdc_chunks` propagates `IsccResult` error via null return
- `cbindgen` header freshness check in CI passed
- `build.rs` runs `csbindgen` to generate `NativeMethods.g.cs`

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI
- `AlgCdcChunks` JNI validates `avgChunkSize < 2` with `IllegalArgumentException`
- 65 Maven tests pass (per last green CI run)

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go
- `AlgCdcChunks` validates `avgChunkSize < 2` — returns `error`, delegates to
    `algCdcChunksUnchecked` for internal callers
- `TestCdcChunksInvalidAvgChunkSize` test covers avgChunkSize=0, 1 (error), 2 (OK)
- `docs/howto/go.md` updated to reflect `([][]byte, error)` return type
- 155 Go tests pass; `go vet` clean

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat)
- All 32 of 32 Tier 1 symbols exposed
- 111 Minitest tests (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance
- `bundle exec rake compile` builds in release profile
- Dedicated `ruby` CI job — runs standardrb, clippy, compile, and test
- `docs/howto/ruby.md` (422 lines); `docs/ruby-api.md` (781 lines — all 32 symbols)
- `zensical.toml` Reference section: "Ruby API" nav entry
- Root `README.md` Ruby section (install tab + quickstart)
- `crates/iscc-rb/CLAUDE.md` added with detailed cross-compilation guidance
- Cross-compilation fixes for v0.2.0: Rakefile gemspec, native loader path, Gemfile.lock symlink
- RubyGems publish switched to OIDC trusted publishing (no API key needed)

## C# / .NET Bindings

**Status**: partially met (32/32 Tier 1 symbols wrapped; conformance tests complete; all return
types typed including streaming hashers; docs + NuGet publish still missing)

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — .NET 8 class library project
- `packages/dotnet/Iscc.Lib/IsccException.cs` — `IsccException : Exception`
- `packages/dotnet/Iscc.Lib/Results.cs` — 11 `sealed record` types: `MetaCodeResult`,
    `TextCodeResult`, `ImageCodeResult`, `AudioCodeResult`, `VideoCodeResult`, `MixedCodeResult`,
    `DataCodeResult`, `InstanceCodeResult`, `IsccCodeResult`, `SumCodeResult`, `DecodeResult`
- `packages/dotnet/Iscc.Lib/IsccLib.cs` — 32 public symbols accessible (methods + classes):
    - 5 constants: `MetaTrimName`, `MetaTrimDescription`, `MetaTrimMeta`, `IoReadSize`,
        `TextNgramSize`
    - 4 text utilities: `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`
    - 10 gen functions: all return typed records (`MetaCodeResult`, `TextCodeResult`, etc.)
    - 2 encoding utilities: `EncodeBase64`, `JsonToDataUrl`
    - 3 codec operations: `IsccDecode`, `IsccDecompose`, `EncodeComponent`
    - 1 utility: `SlidingWindow`
    - 4 algorithm primitives: `AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`
    - 1 diagnostic: `ConformanceSelftest`
- `packages/dotnet/Iscc.Lib/IsccDataHasher.cs` — `IDisposable` + `SafeHandle` pattern;
    `Update(ReadOnlySpan<byte>)` + `Finalize(uint bits = 64)` → **`DataCodeResult`** ✅
- `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs` — same pattern; `Finalize(uint bits = 64)` →
    **`InstanceCodeResult`** ✅
- All 7 empty-span null pointer locations fixed with stack sentinel pattern
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — 41 xUnit `[Fact]` tests
- `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — 9 `[Theory]` methods, 50 vectors
- `packages/dotnet/Iscc.Lib.Tests/testdata/data.json` — vendored (84KB, 50 vectors)
- **Total tests: 91** (41 smoke + 50 conformance), 0 failed
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in run 22802326852
- `NativeMethods.g.cs` — 929 lines, 47 P/Invoke extern declarations (auto-generated by csbindgen)
- **Simplified record types**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` currently
    only carry `(string Iscc)` — spec requires additional fields (`Name`, `MetaHash`, `Description`,
    `Meta`; `Characters`; `DataHash`, `FileSize`). These require C FFI struct changes first.
- **Missing files** per spec: `Native/SafeHandles.cs`
- **Missing**: Documentation (`docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C#
    section)
- **Missing**: Release pipeline (`nuget` publish job in `release.yml`)
- **Missing**: Version sync integration for .NET project version

## C++ Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips)
- **No code exists**: `packages/cpp/` does not exist; no `iscc.hpp`; no vcpkg/Conan manifests

## Swift Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips)
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `low` priority (CID loop skips)
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (not
    started)

## README

**Status**: partially met

- Public-facing polyglot README exists; CI badge, all 7 registry badges present
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples
- Ruby install instructions and quickstart present
- **Gap**: Missing C#, C++, Swift, Kotlin install + quickstart sections (C# now `normal` priority)
- **Gap**: Language logos/icons not added yet (C++/Swift/Kotlin `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 8 crates/packages including `crates/iscc-rb/README.md`
- **Gap**: Target requires READMEs for `packages/dotnet`, `packages/cpp`, `packages/swift`,
    `packages/kotlin` — none complete yet (C# now `normal`, rest `low`)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete
- 8 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, ruby.md
- `docs/ruby-api.md` API reference page (781 lines); `docs/c-ffi-api.md`
- **Gap**: Target requires C# how-to guide (`normal` priority; not started)
- **Gap**: Target requires C++, Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in run 22802326852

## CI/CD and Publishing

**Status**: met (for existing bindings; NuGet + C++/Swift/Kotlin publish not yet added)

- **ALL PASSING** — latest CI run 22802326852: all **13 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22802326852
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, **C# / .NET** — all SUCCESS
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems
- **6 smoke test jobs implemented** — each gates its publish job
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1`
- RubyGems publish switched to OIDC trusted publishing
- v0.2.0 released successfully across all 8 registries
- **Gap**: No `nuget` publish job in `release.yml` yet (`normal` priority)
- **Gap**: No C++/Swift/Kotlin CI or publish jobs (`low` priority)

## Next Milestone

**C#/.NET documentation + NuGet publish pipeline** — the entire .NET API surface is now
feature-complete (32/32 symbols, all typed returns including streaming hashers, 91 tests passing, CI
green). Priority order:

1. **Documentation**: `docs/howto/dotnet.md` how-to guide, `packages/dotnet/README.md`, and README
    C# install + quickstart section. The API is stable and these are self-contained.
2. **NuGet publish job** in `release.yml` — add `nuget` boolean input and build/pack/publish
    workflow step.
3. **Version sync**: add `.NET` project to `mise run version:sync` so the `.csproj` version stays in
    sync with `Cargo.toml`.
4. **SafeHandles.cs**: `Native/SafeHandles.cs` with `SafeHandle` subclasses extracted from
    `IsccDataHasher.cs` / `IsccInstanceHasher.cs` (spec file layout).
