<!-- assessed-at: 2ca9901c13a1fe1d60e8f16db7c1ef03d534a18e -->

# Project State

## Status: IN_PROGRESS

## Phase: .NET CI job added; csbindgen P/Invoke layer is next

v0.2.0 released across all 8 registries. The CID loop completed its second C#/.NET iteration, adding
a `C# / .NET (dotnet build, test)` CI job that passes on every push. The scaffold (1 P/Invoke
function, 1 smoke test) is now validated in CI. The next step is expanding to the full P/Invoke
surface via csbindgen plus idiomatic C# wrappers.

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
- `WASM (wasm-pack test)` = SUCCESS in CI run 22711616491 ✅

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

**Status**: partially met (scaffold + CI job committed; full P/Invoke surface and wrappers missing)

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — .NET 8 class library project ✅
- `packages/dotnet/Iscc.Lib/IsccLib.cs` — `public static partial class IsccLib` with one P/Invoke:
    `ConformanceSelftest()` → `iscc_conformance_selftest` ✅
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — 1 xUnit smoke test (passes) ✅
- `packages/dotnet/.gitignore` — excludes `bin/` and `obj/` artifacts ✅
- `.devcontainer/Dockerfile` — .NET SDK 8 installation via Microsoft install script ✅
- CI job `C# / .NET (dotnet build, test)` — added to `ci.yml`; passes in run 22711616491 ✅
- **Missing**: csbindgen auto-generation of `NativeMethods.g.cs` from `iscc.h` — the full P/Invoke
    surface (31 remaining functions) is not yet wired up
- **Missing**: Idiomatic C# wrappers for all 32 Tier 1 symbols (PascalCase, exceptions, Stream
    support, result record types)
- **Missing**: Conformance tests against `data.json` (xUnit)
- **Missing**: Release pipeline (`release.yml` `nuget` input, multi-platform NuGet pack + publish)
- **Missing**: Version sync integration for .NET project version
- **Missing**: Documentation (`docs/howto/dotnet.md`, README C# install/quickstart section)
- **Note**: NuGet package versions in test `.csproj` use floating (`17.*`, `2.*`) — advisory to pin
    when adding full conformance tests

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
    `packages/kotlin` — none of these directories are complete yet (C# now `normal`, rest `low`)

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

**Status**: met (for existing bindings; NuGet + C++/Swift/Kotlin publish not yet added)

- **ALL PASSING** — latest CI run 22711616491: all **13 jobs** SUCCESS ✅
- URL: https://github.com/iscc/iscc-lib/actions/runs/22711616491
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, **C# / .NET** — all SUCCESS ✅
- `release.yml` has 6 registry `workflow_dispatch` checkboxes: crates.io, PyPI, npm, Maven, FFI,
    RubyGems ✅
- **6 smoke test jobs implemented** — each gates its publish job ✅
- `build-gem` job: 5 platforms via `oxidize-rb/actions/cross-gem@v1` ✅
- **RubyGems publish switched to OIDC** trusted publishing ✅
- v0.2.0 released successfully across all 8 registries ✅
- **Gap**: No `nuget` publish job in `release.yml` yet (`normal` priority)
- **Gap**: No C++/Swift/Kotlin CI or publish jobs (`low` priority)

## Next Milestone

**C#/.NET full P/Invoke layer** — the scaffold (1 function + 1 smoke test) and CI job are committed
and green. The next CID step should expand the P/Invoke surface.

Recommended next work package:

1. **csbindgen P/Invoke layer**: Add `csbindgen` build step in `crates/iscc-ffi/build.rs` (or a
    dedicated tool crate) to auto-generate `NativeMethods.g.cs` from `iscc.h` covering all 10
    `gen_*_v0` functions plus supporting types (`IsccDecodeResult`, `IsccSumCodeResult`, etc.)
2. **Idiomatic C# wrappers**: `IsccLib` PascalCase static methods wrapping the P/Invoke layer, with
    exceptions instead of error codes, `Stream` support for streaming calls, and result record
    types (`IsccResult`, `IsccDecodeResult`)
3. **Conformance tests**: xUnit tests reading `data.json` and verifying all 10 `gen_*_v0` vectors
