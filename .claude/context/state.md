<!-- assessed-at: 81f4e65f9b291be84b53760209c5182bf02c1eac -->

# Project State

## Status: IN_PROGRESS

## Phase: normal-priority issue cleanup (logos only)

v0.2.0 released across all 8 registries. The vcpkg portfile SHA512 issue is now resolved: all 5
platform checksums have been added to `packages/cpp/portfile.cmake` and `SKIP_SHA512` removed. One
`normal`-priority issue remains: language logos absent from README/docs.

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
- `packages/dotnet/Iscc.Lib/IsccLib.cs` — 32 public symbols accessible (all 10 gen functions, 5
    constants, 4 text utilities, 2 encoding, 3 codec, 1 utility, 4 algorithm primitives, 1
    diagnostic) ✅
- `packages/dotnet/Iscc.Lib/Results.cs` — 11 `sealed record` types ✅
- `IsccDataHasher.Finalize()` → `DataCodeResult`; `IsccInstanceHasher.Finalize()` →
    `InstanceCodeResult` ✅
- 41 xUnit `[Fact]` smoke tests + 9 `[Theory]` conformance methods (50 vectors) = 91 total ✅
- xUnit1026 warnings fixed — `vectorName` renamed to `_` in all 9 `[Theory]` methods ✅
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in latest CI run ✅
- `pack-nuget` + `test-nuget` + `publish-nuget` pipeline in `release.yml` ✅
- `docs/howto/dotnet.md` — 417 lines; stale NuGet unavailability note removed ✅
- `packages/dotnet/README.md` — 82 lines ✅
- **Known limitation**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` carry only
    `(string Iscc)` — extra fields require C FFI struct changes first; not blocking
- `NUGET_API_KEY` GitHub Actions secret configured; NuGet publishing pipeline ready

## C++ Bindings

**Status**: met

- `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper with all 32 Tier 1
    symbols, RAII resource management (`UniqueString`, `UniqueStringArray`, `UniqueByteBuffer`,
    `UniqueByteBufferArray`), `IsccError` exception class, full namespace `iscc` ✅
- `packages/cpp/CMakeLists.txt` — CMake config ✅
- `packages/cpp/tests/CMakeLists.txt` + `test_iscc.cpp` — 54 passing tests, ASAN clean ✅
- `safe_data` int32_t overload; `alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0`, and
    `gen_audio_code_v0` all use `detail::safe_data()` for nested/empty vector null-safety ✅
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
- Ruby + C# / .NET + C++ install + quickstart present ✅
- **Open issue** (`normal`): Language logos/icons missing from README and docs for all supported
    languages — filed as `normal` priority
- **Gap** (`low`): Missing Swift, Kotlin install + quickstart sections

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 10 existing crates/packages: `crates/iscc-lib`, `crates/iscc-py`,
    `crates/iscc-napi`, `crates/iscc-wasm`, `crates/iscc-ffi`, `crates/iscc-jni`, `crates/iscc-rb`,
    `packages/go`, `packages/dotnet`, `packages/cpp` ✅
- **Gap**: `packages/swift/README.md`, `packages/kotlin/README.md` missing (those bindings not
    started; `low` priority)

## Documentation

**Status**: met

- 20 pages deployed to lib.iscc.codes; all navigation sections complete
- 9 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md ✅
- `scripts/gen_llms_full.py` — `ORDERED_PAGES` updated to 20 entries (all pages); `discover_pages()`
    auto-discovery via `rglob("*.md")` with `as_posix()` (cross-platform) ✅
- "View as Markdown" / "Copy Page" 404 issue — **RESOLVED** (iteration 3, CID cycle 2) ✅
- **Gap** (`low`): Swift, Kotlin how-to guides not started

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in latest completed run

## CI/CD and Publishing

**Status**: met (for existing bindings)

- **LATEST COMPLETED RUN** — run 22819696003: all **14 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22819696003
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++ (cmake, ASAN, test) — all SUCCESS ✅
- `release.yml` has 7 registry `workflow_dispatch` inputs including `nuget` ✅
- `pack-nuget` → `test-nuget` → `publish-nuget` pipeline in place ✅
- v0.2.0 released successfully across all 8 registries
- `iscc.hpp` bundled in FFI release tarballs (both Unix `cp` and Windows `Copy-Item` steps) ✅
- `NUGET_API_KEY` GitHub Actions secret configured; NuGet publishing pipeline ready

## Next Milestone

One `normal`-priority issue remains open:

**Language logos in README/docs** — Add programming language logos/icons for all 9+ supported
languages (Rust, Python, Node.js, WASM, Go, Java, Ruby, C#/.NET, C++) to the README and
documentation pages. Visual language indicators help users quickly identify binding availability.

After this is addressed, only `low`-priority items remain (Swift/Kotlin bindings), which the CID
loop skips unless directed by the human. The project will be effectively idle at that point.
