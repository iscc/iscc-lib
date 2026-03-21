<!-- assessed-at: 94ce7f6d3f6ca0088c213a70ac7dfc4c84570bd9 -->

# Project State

## Status: IN_PROGRESS

## Phase: Swift / Kotlin bindings (normal-priority issues)

v0.3.1 released across all 8 registries. All previously `normal`-priority issues are resolved:
language logos added to README (18 inline SVG icons), docs Available Bindings table updated, NuGet
smoke test extended to 3 platforms, and C#/C++ minor correctness fixes applied. Two
`normal`-priority issues now drive the CID loop: Swift bindings (not started) and Kotlin bindings
(not started, depends on Swift).

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
- `pyproject.toml` excludes `packages/cpp/conanfile.py` from `ty` type-check scope

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported
- `alg_cdc_chunks` propagates `IsccResult` error from Rust core
- 135 mocha tests pass; `cargo clippy -p iscc-napi -- -D warnings` clean
- `release.yml` NAPI upload now includes `index.js` + `index.d.ts` alongside `*.node`

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via `#[wasm_bindgen]`
- `alg_cdc_chunks` maps `IsccResult` to `JsError`
- `wasm-opt` upgraded from `-O` to `-O3` for max runtime performance
- `crates/iscc-wasm/tests/conformance.rs` asserts `tested == 20`
- `--features conformance` added to `build-wasm` release job so `conformance_selftest` is exported
- WASM CI job = SUCCESS in latest completed run

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
- 155 Go tests pass; `go vet` clean

## Ruby Bindings

**Status**: met

- `crates/iscc-rb/` with Magnus bridge (magnus 0.7.1, Ruby 3.1.2 compat)
- All 32 of 32 Tier 1 symbols exposed
- 111 Minitest tests (295 assertions, 0 failures): 46 smoke + 15 streaming + 50 conformance
- `bundle exec rake compile` builds in release profile
- Dedicated `ruby` CI job — runs standardrb, clippy, compile, and test

## C# / .NET Bindings

**Status**: met

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — .NET 8 class library; full NuGet metadata ✅
- `packages/dotnet/Iscc.Lib/IsccLib.cs` — 32 public symbols; sentinel variables initialized to `0`
    (correctness fix for empty-span fast paths) ✅
- `packages/dotnet/Iscc.Lib/Results.cs` — 11 `sealed record` types ✅
- `IsccDataHasher.Finalize()` → `DataCodeResult`; `IsccInstanceHasher.Finalize()` →
    `InstanceCodeResult` ✅
- 41 xUnit `[Fact]` smoke tests + 9 `[Theory]` conformance methods (50 vectors) = 91 total ✅
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in latest CI run ✅
- `pack-nuget` + `test-nuget` (3 platforms: ubuntu, macos, windows) + `publish-nuget` in
    `release.yml` ✅
- `docs/howto/dotnet.md` — 417 lines ✅
- `packages/dotnet/README.md` — 82 lines ✅
- **Known limitation**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` carry only
    `(string Iscc)` — extra fields require C FFI struct changes first; not blocking
- `NUGET_API_KEY` GitHub Actions secret configured; NuGet publishing pipeline ready

## C++ Bindings

**Status**: met

- `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper with all 32 Tier 1
    symbols, RAII resource management, `IsccError` exception class, full namespace `iscc` ✅
- `iscc_decode` and `gen_sum_code_v0` use exception-safe try/catch to guarantee `iscc_free_*` called
    even if `std::string` or `vector::emplace_back` throws ✅
- `packages/cpp/tests/test_iscc.cpp` — cross-platform temp paths via `std::filesystem` ✅
- `packages/cpp/CMakeLists.txt` — CMake config ✅
- `packages/cpp/tests/CMakeLists.txt` + `test_iscc.cpp` — 54 passing tests, ASAN clean ✅
- `conformance_selftest()` passes; all 10 gen functions tested ✅
- CI job `C++ (cmake, ASAN, test)` — SUCCESS in latest CI run ✅
- `iscc.hpp` bundled in FFI release tarballs ✅
- `packages/cpp/README.md` — 105 lines ✅
- `docs/howto/c-cpp.md` — 497 lines with full C++ wrapper section ✅
- Root `README.md` — C++ install tab + quickstart snippet ✅
- `packages/cpp/vcpkg.json` — vcpkg manifest (version synced by `version_sync.py`) ✅
- `packages/cpp/portfile.cmake` — SHA512 checksums present for all 5 platforms; `SKIP_SHA512`
    removed ✅
- `packages/cpp/conanfile.py` — Conan 2.x recipe (150 lines); downloads and packages pre-built FFI
    binaries for all 5 platforms; `cxxflags = ["-std=c++17"]` removed (MSVC fix) ✅; version synced
    by `version_sync.py` ✅
- **Note**: `conanfile.py` does not pin SHA512 checksums for downloads — no issue filed; acceptable
    for Conan recipes which typically rely on registry integrity

## Swift Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `normal` priority -- CID loop works on this
- **No code exists**: `packages/swift/` does not exist; `crates/iscc-uniffi/` does not exist
- Requires: UniFFI crate, Swift Package, XCTest conformance tests, macOS CI runner,
    `docs/howto/swift.md`, README update

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `normal` priority -- depends on Swift
- **No code exists**: `packages/kotlin/` does not exist; depends on `iscc-uniffi` crate (not
    started)
- Requires: UniFFI crate (shared with Swift), KMP Gradle project, Maven Central publishing,
    `docs/howto/kotlin.md`, README update

## README

**Status**: partially met

- Public-facing polyglot README exists with CI badge and 8 registry badges including NuGet ✅
- Language logos: 18 inline img tags from cdn.simpleicons.org (9 Installation + 9 Quick Start
    headers) ✅
- Installation and Quick Start sections for 9 implemented languages (Rust, Python, Node.js, Java,
    Go, Ruby, C#/.NET, C/C++, WASM) ✅
- ISCC Architecture section, ISCC MainTypes table, Implementors Guide ✅
- All 10 `gen_*_v0` functions listed ✅
- **Gap** (downstream of Swift/Kotlin `normal` issues): Missing Swift and Kotlin installation +
    quickstart sections -- will be added when those bindings are implemented

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 10 existing crates/packages: `crates/iscc-lib`, `crates/iscc-py`,
    `crates/iscc-napi`, `crates/iscc-wasm`, `crates/iscc-ffi`, `crates/iscc-jni`, `crates/iscc-rb`,
    `packages/go`, `packages/dotnet`, `packages/cpp` ✅
- CLAUDE.md files created for all 10 crates/packages (commit a47c934) ✅
- **Gap**: `packages/swift/README.md`, `packages/kotlin/README.md` missing (those bindings not
    started; will be created as part of Swift/Kotlin implementation)

## Documentation

**Status**: met

- 20 pages deployed to lib.iscc.codes; all navigation sections complete
- 9 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md ✅
- `docs/index.md` Available Bindings table updated: Java links to Maven Central, Ruby + C#/.NET +
    C/C++ added, Java install shows dependency snippet (commit b97b0f0) ✅
- `scripts/gen_llms_full.py` — `ORDERED_PAGES` has 20 entries ✅
- View as Markdown / Copy Page 404 issue — RESOLVED ✅
- **Gap** (`low`, CID skips): Language logos in docs howto headers
- **Gap** (`normal`, downstream): Swift, Kotlin how-to guides not yet written

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in latest completed run

## CI/CD and Publishing

**Status**: met (for existing bindings)

- **LATEST COMPLETED RUN** — run 22858622397: all **14 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22858622397
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++ (cmake, ASAN, test) — all SUCCESS ✅
- `release.yml` has `nuget` boolean input; NuGet workflow activates on tag or `inputs.nuget` ✅
- `test-nuget` now runs on 3-OS matrix (ubuntu, macos, windows) ✅
- v0.3.1 released across all 8 registries (crates.io, PyPI, npm x2, Maven Central, RubyGems, NuGet,
    GitHub Releases) ✅
- `iscc.hpp` bundled in FFI release tarballs ✅
- `NUGET_API_KEY` GitHub Actions secret configured ✅

## Next Milestone

Two `normal`-priority issues are open in issues.md — CID loop should begin Swift bindings:

**Swift bindings via UniFFI** — Create `crates/iscc-uniffi/` (shared UniFFI scaffolding crate),
`packages/swift/` (Swift Package Manager package), XCTest conformance tests against vendored
`data.json`, macOS CI runner job in `ci.yml`, and `docs/howto/swift.md`. This also unblocks Kotlin
Multiplatform. Once Swift is complete, Kotlin can follow using the same UniFFI crate.
