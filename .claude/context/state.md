<!-- assessed-at: 820683158771ad4d7cca3d32f7e8cacf0c2bccec -->

# Project State

## Status: IN_PROGRESS

## Phase: Kotlin docs complete; release workflow is last Kotlin gap

v0.3.1 released across all 8 registries. All 16/16 CI jobs pass (run 23386397907). Kotlin
documentation is now **complete** — howto guide, package README/CLAUDE.md, root README integration,
zensical.toml nav, and gen_llms_full.py entry all verified. The sole remaining Kotlin sub-task is
the release workflow (maven-kotlin in release.yml). Four open issues remain.

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

**Status**: partially met (scaffold + tests + CI + docs complete; release workflow missing)

- **Scaffold complete** — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- **3214-line UniFFI-generated bindings** in src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt
- **Conformance tests complete** — 9 @Test methods covering all gen\_\*\_v0 functions (50 vectors)
- **Version sync** — gradle.properties added as 15th target in version_sync.py
- **CI job green** — passes in run 23386397907
- **Documentation complete**:
    - docs/howto/kotlin.md — 451-line howto guide
    - packages/kotlin/README.md — 89-line package README
    - packages/kotlin/CLAUDE.md — 101-line agent guidance
    - Root README: 4 Kotlin mentions (install + quickstart sections)
    - zensical.toml nav entry + gen_llms_full.py entry (now 22 total pages)
- **Missing — release workflow**: No `maven-kotlin` in release.yml

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

**Status**: met

- Criterion benchmarks for all 10 gen\_\*\_v0 functions + 2 additional
- Bench (compile check) CI job SUCCESS

## CI/CD and Publishing

**Status**: partially met (Kotlin release workflow missing)

- **LATEST COMPLETED RUN** — run 23386397907: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23386397907
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 8 registries
- version_sync.py manages 15 sync targets
- **Gap**: No `maven-kotlin` in release.yml

## Open Issues (4 total)

1. **Kotlin bindings** `normal` — scaffold + tests + CI + docs all done; release workflow only
    remaining sub-task
2. **Swift SPM install instructions incorrect** `normal` — Package.swift in subdirectory, SPM URL
    won't resolve from repo root
3. **Swift package does not vend native library** `normal` — linkedLibrary declared but no dylib
    bundled; users get link failures
4. **Language logos in docs** `low` — CID skips

## Next Milestone

**Add `maven-kotlin` to release.yml** — the last remaining Kotlin sub-task:

1. Add `maven-kotlin` boolean input to `workflow_dispatch` in release.yml
2. Add Kotlin/Maven Central publish job (GPG signing + Sonatype credentials, same pattern as Java)
3. After release workflow is added, the Kotlin issue can be fully resolved

Secondary (after Kotlin is closed):

- Address Swift SPM issues (install instructions + native library vending)
- pytest-benchmark comparisons vs iscc-core (target spec mentions these)
