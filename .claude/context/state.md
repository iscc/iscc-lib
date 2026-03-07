<!-- assessed-at: 6a988d94457376eff91dbff6b4286d8d70f2a4aa -->

# Project State

## Status: IN_PROGRESS

## Phase: C# bindings — docs complete; NuGet publish pipeline + version sync pending

v0.2.0 released across all 8 registries. C# / .NET documentation is now complete:
`docs/howto/dotnet.md` (417 lines), `packages/dotnet/README.md` (82 lines), README C# install +
quickstart, and `zensical.toml` nav entry all added (CI run 22803269184, all 13 jobs green).
Remaining C# gaps: NuGet publish pipeline (`release.yml`) and version sync integration. Open
`normal`-priority issues remain (C++ wrapper, NuGet pipeline).

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
- WASM CI job = SUCCESS in run 22803269184

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
types typed including streaming hashers; documentation complete; NuGet publish pipeline + version
sync still missing)

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
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in run 22803269184
- `NativeMethods.g.cs` — 929 lines, 47 P/Invoke extern declarations (auto-generated by csbindgen)
- `docs/howto/dotnet.md` — **417 lines** ✅ (installed, quickstart, all 10 gen functions, streaming,
    build from source, conformance — added in iteration 8)
- `packages/dotnet/README.md` — **82 lines** ✅ (added in iteration 8)
- Root `README.md` — NuGet badge, C# install tab, C# quickstart example ✅ (added in iteration 8)
- `zensical.toml` — "C# / .NET" nav entry in How-To Guides section ✅ (added in iteration 8)
- **Simplified record types**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` currently
    only carry `(string Iscc)` — spec requires additional fields (`Name`, `MetaHash`, `Description`,
    `Meta`; `Characters`; `DataHash`, `FileSize`). These require C FFI struct changes first.
- **Missing files** per spec: `Native/SafeHandles.cs`
- **Missing**: Release pipeline (`nuget` publish job in `release.yml`)
- **Missing**: Version sync integration for .NET project version

## C++ Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `normal` priority in issues.md
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

- Public-facing polyglot README exists; CI badge, all 8 registry badges present (incl. NuGet)
- All 10 `gen_*_v0` functions listed; per-language install + quick-start examples
- Ruby install instructions and quickstart present
- C# / .NET install tab (`dotnet add package Iscc.Lib`) + quickstart present ✅ (added iteration 8)
- **Gap**: Missing C++, Swift, Kotlin install + quickstart sections (`normal`/`low` priority)
- **Gap**: Language logos/icons not added yet (C++/Swift/Kotlin `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all existing 9 crates/packages: `crates/iscc-lib`, `crates/iscc-py`,
    `crates/iscc-napi`, `crates/iscc-wasm`, `crates/iscc-ffi`, `crates/iscc-jni`, `crates/iscc-rb`,
    `packages/go`, `packages/dotnet` ✅ (dotnet added in iteration 8)
- **Gap**: Target requires READMEs for `packages/cpp`, `packages/swift`, `packages/kotlin` — none
    exist (those bindings not started; `normal`/`low` priority)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete
- 9 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, **dotnet.md** ✅ (added in iteration 8, 417 lines)
- `docs/ruby-api.md` API reference page (781 lines); `docs/c-ffi-api.md`
- `zensical.toml` How-To Guides nav includes "C# / .NET" → `howto/dotnet.md` ✅
- **Gap**: Target requires C++ how-to guide update (`normal` priority; not started)
- **Gap**: Target requires Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in run 22803269184

## CI/CD and Publishing

**Status**: met (for existing bindings; NuGet + C++/Swift/Kotlin publish not yet added)

- **ALL PASSING** — latest CI run 22803269184: all **13 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22803269184
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, **C# / .NET** — all SUCCESS
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems
- **6 smoke test jobs implemented** — each gates its publish job
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1`
- RubyGems publish switched to OIDC trusted publishing
- v0.2.0 released successfully across all 8 registries
- **Gap**: No `nuget` publish job in `release.yml` yet (`normal` priority)
- **Gap**: No C++/Swift/Kotlin CI or publish jobs (`normal`/`low` priority)

## Next Milestone

**NuGet publish pipeline** — C# documentation is now complete. Remaining items from the open C#
issue (`normal` priority):

1. **NuGet publish job** in `release.yml`: add `nuget` boolean input, build/pack NuGet package with
    native libraries for all platforms (linux-x64, linux-arm64, win-x64, osx-x64, osx-arm64),
    publish via `dotnet nuget push`.
2. **Version sync**: add `.NET` project to `mise run version:sync` (`scripts/version_sync.py`) so
    the `.csproj` version stays in sync with `Cargo.toml`.
3. **SafeHandles.cs**: extract `SafeHandle` subclasses from `IsccDataHasher.cs` /
    `IsccInstanceHasher.cs` into `Native/SafeHandles.cs` per spec file layout.

After NuGet pipeline is complete, the open C# issue can be closed and CID loop can address the C++
wrapper issue (`normal` priority).
