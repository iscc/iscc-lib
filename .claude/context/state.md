<!-- assessed-at: 6ee0ab1b1cb513df173c5f5a8d62430fa3bba7bc -->

# Project State

## Status: IN_PROGRESS

## Phase: C++ bindings — documentation complete; package manager manifests and test gaps pending

v0.2.0 released across all 8 registries. C# / .NET bindings are complete. The C++17 header-only
wrapper (`iscc.hpp`) has 32 symbols, RAII guards, 52 passing tests, CI job, and FFI tarball
bundling. In iteration 14 all documentation gaps were closed: `packages/cpp/README.md`,
`docs/howto/c-cpp.md` iscc.hpp section, and root README C++ install/quickstart tab. Remaining C++
gaps: package manager manifests (vcpkg, Conan) and `gen_mixed_code_v0` test.

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
- WASM CI job = SUCCESS in run 22808974429

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
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in run 22808974429
- `pack-nuget` + `test-nuget` + `publish-nuget` pipeline in `release.yml` ✅
- `docs/howto/dotnet.md` — 417 lines; `packages/dotnet/README.md` — 82 lines ✅
- **Known limitation**: `MetaCodeResult`, `TextCodeResult`, `InstanceCodeResult` carry only
    `(string Iscc)` — extra fields require C FFI struct changes first; not blocking

## C++ Bindings

**Status**: partially met

- `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper with all 32 Tier 1
    symbols, RAII resource management (`UniqueString`, `UniqueStringArray`, `UniqueByteBuffer`,
    `UniqueByteBufferArray`), `IsccError` exception class, full namespace `iscc` ✅
- `packages/cpp/CMakeLists.txt` — CMake config ✅
- `packages/cpp/tests/CMakeLists.txt` + `test_iscc.cpp` — 52 passing tests, ASAN clean ✅
- `conformance_selftest()` passes; `gen_meta_code_v0` exact match verified ✅
- CI job `C++ (cmake, ASAN, test)` — SUCCESS in run 22808974429 ✅
- `iscc.hpp` bundled in FFI release tarballs (Unix `cp` + Windows `Copy-Item`) ✅
- `packages/cpp/README.md` — 105 lines with install, quickstart, API overview, links ✅
- `docs/howto/c-cpp.md` — 497 lines; iscc.hpp section added at line 328 covering include paths,
    quick start, gen functions, streaming, error handling, codec, conformance ✅
- Root `README.md` — C++ install tab (line 127) + quickstart snippet (line 216) ✅
- **Missing**: `packages/cpp/vcpkg.json`, `portfile.cmake`, `conanfile.py`, `iscc-config.cmake.in`,
    `pkg-config/iscc.pc.in` — no package manager manifests ❌
- **Missing**: `gen_mixed_code_v0` has no test coverage in `test_iscc.cpp` ❌
- **Known edge case**: nested vector `safe_data()` not applied for inner elements in `alg_simhash`,
    `soft_hash_video_v0`, `gen_video_code_v0` — not blocking but should be hardened

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
- `docs/howto/c-cpp.md` — 497 lines; includes full C++ wrapper section with both include conventions
    (tarball flat `#include "iscc.hpp"` vs CMake/vcpkg `<iscc/iscc.hpp>`), gen functions, streaming
    RAII, error handling, codec utilities ✅
- `zensical.toml` "C / C++" nav entry present ✅
- **Gap**: Swift, Kotlin how-to guides (all `low` priority; none started)

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in run 22808974429

## CI/CD and Publishing

**Status**: met (for existing bindings)

- **ALL PASSING** — latest CI run 22808974429: all **14 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/22808974429
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++ (cmake, ASAN, test) — all SUCCESS
- `release.yml` has **7 registry** `workflow_dispatch` inputs including `nuget` ✅
- `pack-nuget` → `test-nuget` → `publish-nuget` pipeline in place ✅
- v0.2.0 released successfully across all 8 registries
- `iscc.hpp` bundled in FFI release tarballs (both Unix `cp` and Windows `Copy-Item` steps) ✅
- **Manual action still needed**: NuGet.org account setup (NUGET_API_KEY secret, package ID
    reservation) before NuGet publish job can be triggered

## Next Milestone

**Complete remaining C++ issue items** — still `normal` priority in issues.md. Documentation gaps
are now closed. Remaining implementation tasks:

1. **`gen_mixed_code_v0` test**: Add missing test in `packages/cpp/tests/test_iscc.cpp` — currently
    the only gen function without test coverage in the C++ test suite
2. **Package manager manifests**: `packages/cpp/vcpkg.json` + `portfile.cmake` (vcpkg port),
    `conanfile.py` (Conan recipe), `iscc-config.cmake.in` (CMake find-package support),
    `pkg-config/iscc.pc.in`
3. **Nested vector null safety**: Harden `alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0` in
    `iscc.hpp` to apply `safe_data()` on inner vector elements (edge case, not blocking)
4. After all C++ items done: only `low`-priority issues remain (Swift, Kotlin, logos) — these are
    CID-loop-skipped until explicitly promoted
