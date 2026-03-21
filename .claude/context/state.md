<!-- assessed-at: 4cfb6abfb314c2322c4ca5c3f836a85eea15a8b8 -->

# Project State

## Status: IN_PROGRESS

## Phase: Swift GITHUB_REF_NAME fix — blocked on human review

v0.3.1 released across all 9 registries. All 16/16 CI jobs pass (run 23391076552). All 12 language
bindings scaffolded, tested, documented, and shipping. XCFramework build script, root Package.swift,
release workflow, version sync (16 targets), and Swift install docs all complete. Only remaining
`normal` issue is the GITHUB_REF_NAME bug in the Swift release job — blocked on
`HUMAN REVIEW REQUESTED` (spec prescribes `GITHUB_REF_NAME`, fix requires spec update).

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

**Status**: partially met

- SPM package with 2400-line UniFFI-generated Swift bindings, all 32 Tier 1 symbols
- 9 conformance test methods covering 50 vectors; CI job SUCCESS on macos-14
- docs/howto/swift.md updated with SPM install instructions + collapsible "Build from source" tip
- XCFramework build script executable, valid shell, 5 Apple targets
- Root `Package.swift` restructured: Ferrostar-style toggle, `releaseTag = "0.3.1"`
- Release workflow: `swift` checkbox input (9th), `build-xcframework` job integrated
- Version sync: `releaseTag` now managed by `version_sync.py` (16th target, confirmed OK)
- **Remaining issue**: `GITHUB_REF_NAME` bug at line 1198 of release.yml — derives version from
    `GITHUB_REF_NAME` instead of Cargo.toml, breaking `--ref main` re-trigger. Flagged
    `HUMAN REVIEW REQUESTED` — spec explicitly prescribes `GITHUB_REF_NAME`; fix requires spec
    update. CID cannot proceed autonomously.

## Kotlin Multiplatform Bindings

**Status**: met

- Scaffold complete — packages/kotlin/ with build.gradle.kts, Gradle 8.12.1, JNA 5.16.0
- 3214-line UniFFI-generated bindings, conformance tests (9 methods, 50 vectors)
- Version sync, CI job, docs, release workflow all complete

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
- **Gap** (low, CID skips): Language logos in docs howto headers

## Benchmarks

**Status**: partially met

- Criterion benchmarks for all 10 gen\_\*\_v0 functions + 2 additional (12 total in Rust)
- Bench (compile check) CI job SUCCESS
- pytest-benchmark: 18 functions (9 gen\_\*\_v0 x 2 — iscc-lib vs iscc-core)
- **Missing**: Speedup factors not yet published in documentation

## CI/CD and Publishing

**Status**: met

- **LATEST COMPLETED RUN** — run 23391076552: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23391076552
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 9 registries (maven-kotlin to be exercised on next release)
- Release workflow has 9 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift
- version_sync.py manages 16 sync targets (all OK)

## Open Issues (2 total — 1 normal, 1 low)

1. **Swift release job `--ref main` re-trigger incompatible** `normal` — `build-xcframework` uses
    `GITHUB_REF_NAME` for version extraction (line 1198 of release.yml). Re-triggering with
    `--ref main -f swift=true` would set `releaseTag = "main"`. Fix: derive version from
    Cargo.toml. `HUMAN REVIEW REQUESTED` — spec explicitly uses `GITHUB_REF_NAME`; CID blocked.
2. **Language logos in docs** `low` — CID skips

## Next Milestone

**Blocked on human review** — the only `normal`-priority issue (GITHUB_REF_NAME bug) has
`HUMAN REVIEW REQUESTED` because the spec explicitly prescribes `GITHUB_REF_NAME` and fixing it
requires a spec change. CID cannot proceed autonomously on this item. The `low`-priority issues
(docs logos, speedup factors in docs) are human-directed only.

**When human approves:** Update spec to use Cargo.toml-derived version, then fix line 1198 of
`release.yml` to `VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')` and
construct tag as `v$VERSION`.

**Remaining low-priority work** (human-directed only):

- Language logos in docs howto headers
- Speedup factors published in documentation
