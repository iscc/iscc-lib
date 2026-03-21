<!-- assessed-at: 105ed4d7e12fc5c878f2abfb4453eda742320e17 -->

# Project State

## Status: IN_PROGRESS

## Phase: Kotlin fully complete; Swift packaging issues remain

v0.3.1 released across all 9 registries (8 previous + maven-kotlin added). All 16/16 CI jobs pass
(run 23387467886). Kotlin bindings are **fully complete** — scaffold, tests, CI, docs, AND release
workflow all verified. Three open issues remain (2 Swift packaging, 1 cosmetic).

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating
- alg_cdc_chunks public API returns IsccResult\<Vec\<&[u8]>> — validates avg_chunk_size < 2
- data.json at iscc-core v1.3.0 (50 total vectors)
- 316 tests pass with default features
- Feature matrix CI (5 steps) passed in latest green run

## Python Bindings

**Status**: met

- All 32 Tier 1 symbols accessible via __all__ (48 entries)
- 207 Python tests pass; ty check passes; cargo clippy -p iscc-py clean

## Node.js Bindings

**Status**: met

- All 32 Tier 1 symbols exported
- 135 mocha tests pass; cargo clippy -p iscc-napi -- -D warnings clean

## WASM Bindings

**Status**: met

- All 32 Tier 1 symbols exported via #[wasm_bindgen]
- wasm-opt -O3; conformance.rs asserts tested == 20

## C FFI

**Status**: met

- 85 Rust tests + 65 C tests pass
- cbindgen header freshness check in CI passed
- build.rs runs csbindgen to generate NativeMethods.g.cs

## Java Bindings

**Status**: met

- All 32 Tier 1 symbols via JNI
- 65 Maven tests pass

## Go Bindings

**Status**: met

- All 32 Tier 1 symbols via pure Go (no CGO)
- 155 Go tests pass; go vet clean

## Ruby Bindings

**Status**: met

- 32 of 32 Tier 1 symbols exposed via Magnus bridge
- 111 Minitest tests (295 assertions, 0 failures)

## C# / .NET Bindings

**Status**: met

- 32 public symbols; 11 sealed record types
- 91 total tests (41 smoke + 50 conformance vectors)
- CI job SUCCESS

## C++ Bindings

**Status**: met

- 681-line C++17 header-only wrapper with all 32 Tier 1 symbols
- 54 passing tests, ASAN clean
- vcpkg manifest + Conan 2.x recipe

## UniFFI Scaffolding Crate

**Status**: complete (internal, not published)

- 32 `#[uniffi::export]` annotations, 21 `#[test]` functions pass
- Proc macro approach — no uniffi.toml or build.rs needed
- Dependencies: iscc-lib (with meta-code feature), uniffi 0.31, thiserror

## Swift Bindings

**Status**: met

- SPM package with 2400-line UniFFI-generated Swift bindings, all 32 Tier 1 symbols
- 9 conformance test methods covering 50 vectors
- CI job SUCCESS on macos-14
- docs/howto/swift.md (425 lines), README, CLAUDE.md

## Kotlin Multiplatform Bindings

**Status**: met

- **Scaffold complete** — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- **3214-line UniFFI-generated bindings** in src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt
- **Conformance tests complete** — 9 @Test methods covering all gen\_\*\_v0 functions (50 vectors)
- **Version sync** — gradle.properties added as 15th target in version_sync.py
- **CI job green** — passes in latest run
- **Documentation complete** — howto guide (451 lines), package README (89 lines), CLAUDE.md (101
    lines), root README integration, zensical.toml nav, gen_llms_full.py (22 pages)
- **Release workflow complete** — `maven-kotlin` input + build-kotlin-native + smoke-test-kotlin +
    publish-maven-kotlin jobs in release.yml. GPG signing via `useInMemoryPgpKeys`, Central Portal
    upload via curl REST API

## README

**Status**: met

- Public-facing polyglot README with CI badge and 8 registry badges
- Language logos: 18 inline img tags from cdn.simpleicons.org
- Installation and Quick Start sections for all 12 languages (Rust, Python, Ruby, Java, Go, Node.js,
    WASM, C#, C++, Swift, Kotlin, C/C++)
- ISCC Architecture section, MainTypes table, Implementors Guide

## Per-Crate READMEs

**Status**: met

- READMEs present for all 12 crates/packages (7 crates + 5 packages)
- CLAUDE.md files present for all 12 crates/packages

## Documentation

**Status**: partially met

- 22 pages in gen_llms_full.py ORDERED_PAGES; all navigation sections complete
- 11 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md, swift.md, kotlin.md
- **Gap** (low, CID skips): Language logos in docs howto headers

## Benchmarks

**Status**: partially met

- Criterion benchmarks for all 10 gen\_\*\_v0 functions + 2 additional (12 total)
- Bench (compile check) CI job SUCCESS
- **Missing**: No pytest-benchmark setup comparing Python bindings vs iscc-core
- **Missing**: No speedup factors published in documentation

## CI/CD and Publishing

**Status**: met

- **LATEST COMPLETED RUN** — run 23387467886: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23387467886
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 8 registries (maven-kotlin to be exercised on next release)
- Release workflow has 8 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin
- version_sync.py manages 15 sync targets

## Open Issues (3 total)

1. **Swift SPM install instructions incorrect** `normal` — Package.swift in subdirectory, SPM URL
    won't resolve from repo root
2. **Swift package does not vend native library** `normal` — linkedLibrary declared but no dylib
    bundled; users get link failures
3. **Language logos in docs** `low` — CID skips

## Next Milestone

**Address Swift packaging issues** — the two `normal` Swift issues are interconnected:

1. Fix native library vending (XCFramework binary target, build plugin, or documented
    build-from-source)
2. Fix SPM install instructions to match the chosen distribution approach
3. These should be tackled together since the library distribution approach determines the install
    instructions

Secondary (lower priority):

- pytest-benchmark comparisons vs iscc-core (target spec mentions these)
- Speedup factors in documentation
