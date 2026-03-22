<!-- assessed-at: 366f36a264ce8633b41b347a3f252ce192a25eaf -->

# Project State

## Status: IN_PROGRESS

## Phase: Kotlin Android native libraries — critical gap

v0.3.1 released across all 9 registries. All 16/16 CI jobs pass (run 23398247400). All 12 language
bindings scaffolded, tested, and documented. The GITHUB_REF_NAME bug was fixed (commit d29a1b3).
However, the target was refocused: Kotlin bindings now target Android developers as the primary
audience, requiring native libraries for 4 Android ABIs. This is filed as a `critical` issue — the
published JAR is unusable on Android because no Android native libraries are bundled.

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
- GITHUB_REF_NAME bug fixed (commit d29a1b3) — now derives version from Cargo.toml

## Kotlin Bindings

**Status**: partially met

- Scaffold complete — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- 3214-line UniFFI-generated bindings, conformance tests (9 methods, 50 vectors)
- Version sync, CI job, docs, release workflow all complete
- Release workflow builds 5 desktop/server targets (linux-x86-64, linux-aarch64, darwin-aarch64,
    darwin-x86-64, win32-x86-64) — **no Android ABIs**
- **Critical gap**: Target now requires Android native libraries for 4 ABIs (arm64-v8a, armeabi-v7a,
    x86_64, x86). Published JAR is unusable on Android. Spec updated in
    `.claude/context/specs/kotlin-bindings.md` with Android cross-compilation details.

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

- **LATEST COMPLETED RUN** — run 23398247400: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23398247400
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 9 registries
- Release workflow has 9 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift
- version_sync.py manages 16 sync targets (all OK)

## Open Issues (6 total — 1 critical, 4 normal, 1 low)

1. **Kotlin bindings missing Android native libraries** `critical` — Release workflow only builds 5
    desktop/server targets. No Android ABIs (aarch64-linux-android, armv7-linux-androideabi,
    x86_64-linux-android, i686-linux-android) are cross-compiled. Published JAR unusable on
    Android. Requires: NDK setup, cargo-ndk, release workflow matrix expansion, JNA resource path
    mapping.
2. **XCFramework release cache key incomplete** `normal` — Cache key at release.yml:1192 only hashes
    crate sources and Cargo.lock, missing build script and Swift headers.
3. **Swift release job checks out `ref: main` instead of tag SHA** `normal` — race window if main
    moves after tag creation.
4. **Kotlin release smoke test doesn't validate assembled JAR** `normal` — Tests run against raw
    native libs, not the packaged JAR.
5. **CI does not exercise root Package.swift** `normal` — Only packages/swift manifest tested,
    consumer-facing root manifest never validated.
6. **Language logos in docs** `low` — CID skips.

## Next Milestone

**Critical priority: Kotlin Android native libraries.** The target was refocused to make Android the
primary audience for Kotlin bindings. Required work:

1. Add Android NDK + Rust Android targets + `cargo-ndk` to devcontainer (or CI-only)
2. Add 4 Android ABI targets to `build-kotlin-native` matrix in `release.yml`
3. Map Android Rust targets to JNA resource paths in `assemble-kotlin` job
4. Add Android smoke test or resource-path verification
5. Update `docs/howto/kotlin.md` with Android-specific install instructions

After Android support: address the 4 `normal` release workflow issues (cache key, ref:main race, JAR
smoke test, root Package.swift CI).
