<!-- assessed-at: a518c5dc24dd67855df6b6b6c8a3be665484eab4 -->

# Project State

## Status: IN_PROGRESS

## Phase: Swift bindings — CI green, docs/README/version-sync remaining

v0.3.1 released across all 8 registries. The Swift package (`packages/swift/`) is functional with
UniFFI-generated bindings, SPM manifest, and XCTest conformance tests — and the CI job now
**passes** (all 15/15 jobs green, run 23380398819). The SPM module name mismatch was fixed by
renaming `IsccLibFFI` to `iscc_uniffiFFI`. Remaining Swift work: `docs/howto/swift.md`, README Swift
install/quickstart sections, version sync, and `packages/swift/CLAUDE.md`. Two `normal`-priority
issues remain: Swift bindings (in progress) and Kotlin bindings (not started, depends on Swift).

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

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — .NET 8 class library; full NuGet metadata
- `packages/dotnet/Iscc.Lib/IsccLib.cs` — 32 public symbols
- `packages/dotnet/Iscc.Lib/Results.cs` — 11 `sealed record` types
- `IsccDataHasher.Finalize()` -> `DataCodeResult`; `IsccInstanceHasher.Finalize()` ->
    `InstanceCodeResult`
- 41 xUnit `[Fact]` smoke tests + 9 `[Theory]` conformance methods (50 vectors) = 91 total
- CI job `C# / .NET (dotnet build, test)` — SUCCESS in latest CI run
- `pack-nuget` + `test-nuget` (3 platforms) + `publish-nuget` in `release.yml`

## C++ Bindings

**Status**: met

- `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper with all 32 Tier 1
    symbols, RAII resource management, `IsccError` exception class, full namespace `iscc`
- 54 passing tests, ASAN clean; `conformance_selftest()` passes
- CI job `C++ (cmake, ASAN, test)` — SUCCESS in latest CI run
- `iscc.hpp` bundled in FFI release tarballs
- vcpkg manifest + Conan 2.x recipe; version synced by `version_sync.py`

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- `crates/iscc-uniffi/` — 704-line `lib.rs` with `publish = false`
- 32 `#[uniffi::export]` annotations: 30 free functions + 2 `impl` blocks (DataHasher,
    InstanceHasher)
- 11 `uniffi::Record` types for all result structs + `DecodeResult`
- 2 `uniffi::Object` types with `Mutex<Option<Inner>>` for thread-safe streaming
- 5 constant getter functions (UniFFI doesn't support const exports)
- Error mapping via `#[derive(uniffi::Error)]` enum `IsccUniError`
- 21 `#[test]` functions pass
- `cargo clippy -p iscc-uniffi -- -D warnings` clean
- Uses proc macro approach — no `uniffi.toml` or `build.rs` needed
- Dependencies: `iscc-lib` (with `meta-code` feature), `uniffi` 0.31, `thiserror`
- `bindgen` feature added with `[[bin]]` section for `uniffi-bindgen` CLI

## Swift Bindings

**Status**: partially met (CI passing, docs/README/version-sync not done)

- **Exists**: `packages/swift/` with complete SPM package structure:
    - `Package.swift` — SPM manifest (swift-tools-version: 5.9), 3 targets (iscc_uniffiFFI, IsccLib,
        IsccLibTests)
    - `Sources/IsccLib/iscc_uniffi.swift` — 2400-line UniFFI-generated Swift bindings with all 32 Tier
        1 symbols (camelCase, `throws`, Swift `Data` types)
    - `Sources/iscc_uniffiFFI/iscc_uniffiFFI.h` — 935-line generated C header
    - `Sources/iscc_uniffiFFI/module.modulemap` — `module iscc_uniffiFFI` (matches generated code)
    - `Tests/IsccLibTests/ConformanceTests.swift` — 9 test methods covering 50 vectors across all 9
        `gen_*_v0` functions (correct per-function counts: 20+5+3+5+3+2+4+3+5)
    - `Tests/IsccLibTests/data.json` — vendored conformance vectors (matches iscc-lib copy)
    - `README.md` — installation, usage examples, build-from-source instructions
- **CI job PASSING** (run 23380398819): `Swift (swift build, swift test)` on `macos-14` — SUCCESS
- **Not done**: `docs/howto/swift.md` how-to guide
- **Not done**: README Swift install/quickstart sections
- **Not done**: Version sync integration (`Constants.swift` with version string)
- **Not done**: `packages/swift/CLAUDE.md`

## Kotlin Multiplatform Bindings

**Status**: not started

- Target defined in `target.md`; issue filed as `normal` priority — depends on Swift
- **No code exists**: `packages/kotlin/` does not exist
- UniFFI scaffolding crate (shared dependency) is now complete
- Requires: KMP Gradle project, Maven Central publishing, `docs/howto/kotlin.md`, README update

## README

**Status**: partially met

- Public-facing polyglot README exists with CI badge and 8 registry badges including NuGet
- Language logos: 18 inline img tags from cdn.simpleicons.org
- Installation and Quick Start sections for 9 implemented languages
- ISCC Architecture section, ISCC MainTypes table, Implementors Guide
- All 10 `gen_*_v0` functions listed
- **Gap** (downstream of Swift/Kotlin `normal` issues): Missing Swift and Kotlin installation +
    quickstart sections

## Per-Crate READMEs

**Status**: partially met

- READMEs present for all 10 existing crates/packages + Swift package
- CLAUDE.md files created for all 10 crates/packages (not yet for Swift)
- **Gap**: `packages/kotlin/README.md` missing (Kotlin bindings not started)

## Documentation

**Status**: partially met

- 20 pages deployed to lib.iscc.codes; all navigation sections complete
- 9 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md
- `scripts/gen_llms_full.py` — `ORDERED_PAGES` has 20 entries
- **Gap** (`low`, CID skips): Language logos in docs howto headers
- **Gap** (`normal`, downstream): Swift, Kotlin how-to guides not yet written

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 `gen_*_v0` functions
- `bench_data_hasher_streaming` + `bench_cdc_chunks` additional benchmarks
- `Bench (compile check)` CI job SUCCESS in latest completed run

## CI/CD and Publishing

**Status**: partially met (no Kotlin CI job)

- **LATEST COMPLETED RUN** — run 23380398819: **15/15 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23380398819
- All jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js, WASM,
    C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, **Swift** — all SUCCESS
- v0.3.1 released across all 8 registries (crates.io, PyPI, npm x2, Maven Central, RubyGems, NuGet,
    GitHub Releases)
- `iscc.hpp` bundled in FFI release tarballs
- `NUGET_API_KEY` GitHub Actions secret configured
- **Gap**: No Kotlin CI job (Kotlin bindings not started)

## Next Milestone

**Complete Swift integration** (CI is green, finish remaining documentation and tooling):

1. **`docs/howto/swift.md`** — write Swift how-to guide covering SPM installation, all 9 `gen_*_v0`
    usage examples, streaming API, error handling
2. **README Swift sections** — add Swift install/quickstart to the repository README
3. **`packages/swift/CLAUDE.md`** — per-package agent guidance file
4. **Version sync** — add Swift version target to `scripts/version_sync.py` (e.g., `Constants.swift`
    with version string)

After Swift issue is closed, begin Kotlin bindings.
