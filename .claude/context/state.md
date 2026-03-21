<!-- assessed-at: 9abb15e6edaa27c100ccf80bca8217f40ef0a9bd -->

# Project State

## Status: IN_PROGRESS

## Phase: Swift bindings — UniFFI scaffolding done, Swift package next

v0.3.1 released across all 8 registries. The UniFFI scaffolding crate (`crates/iscc-uniffi/`) is now
complete with all 32 Tier 1 symbols exposed via proc macros and 21 unit tests passing. Next step is
creating the Swift package (`packages/swift/`) with generated bindings and XCTest conformance tests.
Two `normal`-priority issues drive the CID loop: Swift bindings (in progress) and Kotlin bindings
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
- Review verdict: PASS (iteration 1)

## Swift Bindings

**Status**: partially met (UniFFI scaffolding done, Swift package not started)

- UniFFI scaffolding crate complete (see above)
- **Not started**: `packages/swift/` does not exist — needs `Package.swift`, generated Swift
    bindings via `uniffi-bindgen generate`, XCTest conformance tests, `README.md`
- **Not started**: CI job (macOS runner for `swift build` + `swift test`)
- **Not started**: `docs/howto/swift.md`, README Swift install/quickstart sections

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

- READMEs present for all 10 existing crates/packages
- CLAUDE.md files created for all 10 crates/packages
- **Gap**: `packages/swift/README.md`, `packages/kotlin/README.md` missing (those bindings not
    started)

## Documentation

**Status**: met

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

**Status**: met (for existing bindings)

- **LATEST COMPLETED RUN** — run 23378717217: all **14 jobs** SUCCESS
- URL: https://github.com/iscc/iscc-lib/actions/runs/23378717217
- Jobs: Version consistency, Rust, Python 3.10, Python 3.14, Python (ruff/pytest gate), Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++ (cmake, ASAN, test) — all SUCCESS
- v0.3.1 released across all 8 registries (crates.io, PyPI, npm x2, Maven Central, RubyGems, NuGet,
    GitHub Releases)
- `iscc.hpp` bundled in FFI release tarballs
- `NUGET_API_KEY` GitHub Actions secret configured
- **Note**: No CI job yet for `iscc-uniffi` crate — will be added with Swift CI job

## Next Milestone

Continue the Swift bindings issue — the UniFFI scaffolding crate is done, so the next step is
creating the Swift package:

**Swift package (`packages/swift/`)** — Generate Swift bindings via `uniffi-bindgen generate`,
create `Package.swift` (SPM manifest), write XCTest conformance tests against vendored `data.json`,
and add a `swift` CI job (macOS runner). Also needs `docs/howto/swift.md` and README Swift
install/quickstart. Once Swift is complete, Kotlin can follow using the same UniFFI crate.
