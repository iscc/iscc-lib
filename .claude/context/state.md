<!-- assessed-at: e19aeae -->

# Project State

## Status: IN_PROGRESS

## Phase: Near-complete — 1 low-priority issue remaining

v0.3.1 released across all 9 registries. All 16/16 CI jobs pass (run 23402159613). All 12 language
bindings scaffolded, tested, and documented. All previously open normal/critical issues resolved.
Benchmarks documentation with speedup factors is published. Only one low-priority cosmetic issue
remains (language logos in docs), which CID is configured to skip.

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
- **Provenance guard**: `build-xcframework` verifies main HEAD matches tag SHA
- Version sync: `releaseTag` managed by `version_sync.py` (16th target, confirmed OK)
- **Root manifest smoke test**: `swift package dump-package` step in CI validates consumer-facing
    Package.swift parses correctly (added iteration 7)

## Kotlin Bindings

**Status**: met

- Scaffold complete — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- 3214-line UniFFI-generated bindings, conformance tests (9 methods, 50 vectors)
- Version sync, CI job, docs, release workflow all complete
- Release workflow builds **9 targets**: 5 desktop/server + 4 Android ABIs
- JAR smoke test validates runtime JAR contains all 9 native library paths

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

**Status**: met

- 22 pages in gen_llms_full.py ORDERED_PAGES; all navigation sections complete
- 11 language howto guides: c-cpp.md, rust.md, python.md, nodejs.md, wasm.md, go.md, java.md,
    ruby.md, dotnet.md, swift.md, kotlin.md
- docs/index.md: 11 language tabs in Quick Start, Swift+Kotlin in Available Bindings table
- docs/benchmarks.md: full speedup comparison table (1.3x to 158x), Criterion native results,
    methodology, key findings, reproduction commands
- **Gap** (low, CID skips): Language logos in docs howto headers

## Benchmarks

**Status**: met

- Criterion benchmarks for all 10 gen\_\*\_v0 functions + 2 additional (12 total in Rust)
- Bench (compile check) CI job SUCCESS
- pytest-benchmark: 18 functions (9 gen\_\*\_v0 x 2 — iscc-lib vs iscc-core)
- Speedup factors published in docs/benchmarks.md (1.3x to 158x across 10 functions)

## CI/CD and Publishing

**Status**: met

- **LATEST COMPLETED RUN** — run 23402159613: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23402159613
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 9 registries
- Release workflow has 9 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift
- XCFramework cache key expanded to include build script, Swift headers, and all Cargo.toml files
- Swift release provenance guard: verifies main HEAD == tag SHA before XCF build
- Root Package.swift smoke test: `dump-package` validates manifest in CI
- version_sync.py manages 16 sync targets (all OK)

## Open Issues (1 total — 0 critical, 0 normal, 1 low)

1. **Language logos in docs** `low` — CID skips, human-directed only.

## Next Milestone

All 12 bindings are complete, CI is green (16/16), benchmarks are documented, and all
normal/critical issues are resolved. The project is functionally complete per target.md criteria.

The only remaining item is the low-priority cosmetic issue (language logos in docs howto headers),
which CID is configured to skip. This requires human direction to proceed.

**Note:** Status remains IN_PROGRESS because 1 open issue exists in issues.md (even though it's low
priority and CID-skipped). The human may choose to close it as won't-fix, address it interactively,
or leave it — at which point the project can move to DONE.
