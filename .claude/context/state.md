<!-- assessed-at: ab8a4c3876afa2b32e41d89102becfe969d1b54d -->

# Project State

## Status: IN_PROGRESS

## Phase: normal-priority bug fixes (Conan recipe, C++ audio, docs)

v0.2.0 released across all 8 registries. The C++ package manager manifests (`vcpkg.json`,
`portfile.cmake`, `conanfile.py`) were added in iteration 16. Four `normal`-priority issues remain
open: a broken Conan recipe (missing shared library), `gen_audio_code_v0` null-pointer crash on
empty vector in `iscc.hpp`, stale `.NET docs` mentioning NuGet is unavailable, and a broken "View as
Markdown" feature on the docs site.

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
- `pyproject.toml` now excludes `packages/cpp/conanfile.py` from `ty` type-check scope

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
- `docs/howto/dotnet.md` — 417 lines; `packages/dotnet/README.md` — 82 lines ✅
- **Known limitation**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` carry only
    `(string Iscc)` — extra fields require C FFI struct changes first; not blocking
- **Open issue** (`normal`): `docs/howto/dotnet.md:21` still says NuGet publishing unavailable —
    needs update

## C++ Bindings

**Status**: partially met

- `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper with all 32 Tier 1
    symbols, RAII resource management (`UniqueString`, `UniqueStringArray`, `UniqueByteBuffer`,
    `UniqueByteBufferArray`), `IsccError` exception class, full namespace `iscc` ✅
- `packages/cpp/CMakeLists.txt` — CMake config ✅
- `packages/cpp/tests/CMakeLists.txt` + `test_iscc.cpp` — 53 passing tests, ASAN clean ✅
- `safe_data` int32_t overload; `alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0` use
    `detail::safe_data()` for nested vector null-safety ✅
- `conformance_selftest()` passes; all 10 gen functions tested ✅
- CI job `C++ (cmake, ASAN, test)` — SUCCESS in latest CI run ✅
- `iscc.hpp` bundled in FFI release tarballs ✅
- `packages/cpp/README.md` — 105 lines ✅
- `docs/howto/c-cpp.md` — 497 lines with full C++ wrapper section ✅
- Root `README.md` — C++ install tab + quickstart snippet ✅
- `packages/cpp/vcpkg.json` — vcpkg manifest ✅ (added iteration 16)
- `packages/cpp/portfile.cmake` — vcpkg portfile (87 lines, maps triplets to GitHub Releases) ✅
    (added iteration 16)
- `packages/cpp/conanfile.py` — Conan 2.x recipe (76 lines) ✅ (added iteration 16)
- **Open issue** (`normal`): `conanfile.py` declares `package_type = "shared-library"` and
    `self.cpp_info.libs = ["iscc_ffi"]`, but `package()` only copies headers — never packages the
    native `iscc_ffi` binary. Consumers cannot link. Needs fix.
- **Open issue** (`normal`): `gen_audio_code_v0` in `iscc.hpp:472` passes `cv.data()` directly; for
    empty vector this is NULL → FFI rejects it. Fix: use `detail::safe_data(cv)`.
- **Open issue** (`low`): `portfile.cmake` uses `SKIP_SHA512` — no checksum pinning
- **Missing**: `packages/cpp/iscc-config.cmake.in` (CMake find_package template — scoped out)

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
- **Gap**: Missing Swift, Kotlin install + quickstart sections (`low` priority)
- **Gap**: Language logos/icons not added yet (C++/Swift/Kotlin `low` priority)

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 10 existing crates/packages: `crates/iscc-lib`, `crates/iscc-py`,
    `crates/iscc-napi`, `crates/iscc-wasm`, `crates/iscc-ffi`, `crates/iscc-jni`, `crates/iscc-rb`,
    `packages/go`, `packages/dotnet`, `packages/cpp` ✅
- **Gap**: `packages/swift/README.md`, `packages/kotlin/README.md` missing (those bindings not
    started; `low` priority)

## Documentation

**Status**: partially met

- 17+ pages deployed to lib.iscc.codes; all navigation sections complete
- 9 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md ✅
- `docs/howto/c-cpp.md` — 497 lines; includes full C++ wrapper section ✅
- **Open issue** (`normal`): `docs/howto/dotnet.md:21` says "NuGet publishing is not yet available"
    — stale; needs update now that publish pipeline exists
- **Open issue** (`normal`): "View as Markdown" / "Copy Page" button on docs site returns 404 —
    needs Zensical fix (see `iscc-usearch` implementation)
- **Gap**: Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in latest completed run

## CI/CD and Publishing

**Status**: met (for existing bindings)

- **LATEST COMPLETED RUN** — run 22816479556: all **14 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22816479556
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++ (cmake, ASAN, test) — all SUCCESS
- Run 22816480092 in-progress at assessment time (12/14 jobs SUCCESS, Bench + Python gate pending)
- `release.yml` has 7 registry `workflow_dispatch` inputs including `nuget` ✅
- `pack-nuget` → `test-nuget` → `publish-nuget` pipeline in place ✅
- v0.2.0 released successfully across all 8 registries
- `iscc.hpp` bundled in FFI release tarballs (both Unix `cp` and Windows `Copy-Item` steps) ✅
- **Manual action still needed**: NuGet.org account setup (NUGET_API_KEY secret, package ID
    reservation) before NuGet publish job can be triggered

## Next Milestone

Fix the four open `normal`-priority issues, in order of impact:

1. **C++ audio null pointer** — `gen_audio_code_v0` in `iscc.hpp:472` uses `cv.data()` directly;
    empty vector makes this NULL. Fix: use `detail::safe_data(cv)`. Add smoke test.
2. **Conan recipe contract** — `conanfile.py` declares `shared-library` but `package()` never copies
    the `iscc_ffi` binary. Fix: package pre-built binary or reclassify as `header-library`.
3. **Stale .NET docs** — `docs/howto/dotnet.md:21` says NuGet publishing is unavailable; update to
    reflect the `pack-nuget` / `publish-nuget` pipeline now in `release.yml`.
4. **Docs "View as Markdown" 404** — clicking "View as Markdown" on lib.iscc.codes navigates to a
    404\. Investigate how `iscc/iscc-usearch` solves this; apply the same Zensical fix.

After these four, only `low`-priority issues remain (vcpkg SHA512, version sync, Swift, Kotlin,
logos) — the CID loop skips `low` by default.
