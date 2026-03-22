<!-- assessed-at: aa6037798bcb99c0ce6a11580d3e79ea510502cd -->

# Project State

## Status: IN_PROGRESS

## Phase: Release workflow hardening + benchmark documentation gap

v0.3.1 released across all 9 registries. All 16/16 CI jobs pass (run 23399952416). All 12 language
bindings scaffolded, tested, and documented. The JNA ARM32 resource path mismatch was fixed
(`android-armv7` → `android-arm` in release.yml and kotlin-bindings.md spec). Four normal-priority
release workflow hardening issues remain, plus a benchmarks documentation gap and a low-priority
cosmetic issue.

## Rust Core Crate

**Status**: met

- All 32 Tier 1 symbols present with correct feature-gating
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
- 9 conformance test methods covering 50 vectors; CI job SUCCESS on macos-14
- docs/howto/swift.md updated with SPM install instructions + collapsible "Build from source" tip
- XCFramework build script executable, valid shell, 5 Apple targets
- Root `Package.swift` restructured: Ferrostar-style toggle, `releaseTag = "0.3.1"`
- Release workflow: `swift` checkbox input (9th), `build-xcframework` job integrated
- Version sync: `releaseTag` managed by `version_sync.py` (16th target, confirmed OK)

## Kotlin Bindings

**Status**: met

- Scaffold complete — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- 3214-line UniFFI-generated bindings, conformance tests (9 methods, 50 vectors)
- Version sync, CI job, docs, release workflow all complete
- Release workflow builds **9 targets**: 5 desktop/server + 4 Android ABIs
    - Desktop: linux-x86-64, linux-aarch64, darwin-aarch64, darwin-x86-64, win32-x86-64
    - Android: android-aarch64 (arm64-v8a), android-arm (armeabi-v7a), android-x86-64 (x86_64),
        android-x86 (x86)
- Uses `cargo-ndk` with NDK r27c for Android builds
- JNA ARM32 resource path fix verified: `android-arm` in release.yml, no `android-armv7` remnants

## README

**Status**: met

- Public-facing polyglot README with CI badge and 8 registry badges
- Language logos: 18 inline img tags from cdn.simpleicons.org
- Installation and Quick Start sections for all 12 languages
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
- docs/index.md: 11 language tabs in Quick Start, Swift+Kotlin in Available Bindings table
- **Gap** (low, CID skips): Language logos in docs howto headers

## Benchmarks

**Status**: partially met

- Criterion benchmarks for all 10 gen\_\*\_v0 functions + 2 additional (12 total in Rust)
- Bench (compile check) CI job SUCCESS
- pytest-benchmark: 18 functions (9 gen\_\*\_v0 x 2 — iscc-lib vs iscc-core)
- **Missing**: Speedup factors not yet published in documentation

## CI/CD and Publishing

**Status**: met

- **LATEST COMPLETED RUN** — run 23399952416: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23399952416
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 9 registries
- Release workflow has 9 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift
- version_sync.py manages 16 sync targets (all OK)

## Open Issues (5 total — 0 critical, 4 normal, 1 low)

1. **XCFramework release cache key incomplete** `normal` — Cache key only hashes crate sources and
    Cargo.lock, missing build script and Swift headers.
2. **Swift release job checks out `ref: main` instead of tag SHA** `normal` — race window if main
    moves after tag creation.
3. **Kotlin release smoke test doesn't validate assembled JAR** `normal` — Tests run against raw
    native libs, not the packaged JAR.
4. **CI does not exercise root Package.swift** `normal` — Only packages/swift manifest tested,
    consumer-facing root manifest never validated.
5. **Language logos in docs** `low` — CID skips.

## Next Milestone

All 12 bindings are complete and CI is green. The remaining gaps are:

1. **Publish speedup factors in documentation** (benchmarks gap) — most impactful actionable item
    since all 4 normal issues are release workflow hardening that the CID loop may not prioritize
    over feature completeness.
2. **Release workflow hardening** (4 normal issues) — XCF cache key, Swift ref:main race, Kotlin JAR
    smoke test, root Package.swift CI. The Kotlin JAR smoke test is most impactful since it would
    catch resource-path errors like the one just fixed.
3. **Language logos in docs** (low) — CID skips, human-directed only.
