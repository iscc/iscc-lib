<!-- assessed-at: ad566e1ca05d8181a7e5acbcd7f680b48ecfde0b -->

# Project State

## Status: IN_PROGRESS

## Phase: Swift XCFramework — version sync + docs update + release job fix

v0.3.1 released across all 9 registries. All 16/16 CI jobs pass (run 23390387523). All 12 language
bindings scaffolded, tested, documented, and shipping. XCFramework build script, root Package.swift,
and release workflow job all complete. Remaining: version sync for releaseTag, docs/howto/swift.md
update, and fix for GITHUB_REF_NAME bug in the Swift release job.

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
- docs/howto/swift.md, README, CLAUDE.md all present
- **XCFramework build script** (`scripts/build_xcframework.sh`): executable, valid shell syntax, 5
    Apple targets, lipo fat binaries, XCFramework assembly, zip + checksum
- **Root `Package.swift` restructured**: Ferrostar-style `useLocalFramework` toggle,
    `.binaryTarget(url:checksum:)` for distribution, `releaseTag = "0.3.1"`,
    `releaseChecksum = "PLACEHOLDER"`
- **Release workflow integrated**: `swift` checkbox input added (9th boolean input),
    `build-xcframework` job added to release.yml — macOS runner, build script invocation, checksum
    update via sed, force-update tag, upload zip as GitHub Release asset
- **Missing**:
    1. Version sync — `releaseTag` not yet in `version_sync.py` (still 15 targets, not 16)
    2. docs/howto/swift.md not yet updated for zero-friction SPM install (still says "planned for a
        future release")
    3. GITHUB_REF_NAME bug — Swift release job derives version from `GITHUB_REF_NAME` instead of
        Cargo.toml, breaking `--ref main` re-trigger convention (filed as normal issue)

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

- **LATEST COMPLETED RUN** — run 23390387523: **16/16 jobs SUCCESS**
- URL: https://github.com/iscc/iscc-lib/actions/runs/23390387523
- All 16 jobs passing: Version consistency, Rust, Python 3.10, Python 3.14, Python gate, Node.js,
    WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, C++, Swift, Kotlin
- v0.3.1 released across all 9 registries (maven-kotlin to be exercised on next release)
- Release workflow has 9 registry inputs: crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift
- version_sync.py manages 15 sync targets

## Open Issues (3 total — 2 normal, 1 low)

1. **Swift package does not vend native library** `normal` — Build script, Package.swift, and
    release workflow all done. Remaining: version sync (releaseTag in version_sync.py) and
    docs/howto/swift.md update for zero-friction SPM install.
2. **Swift release job `--ref main` re-trigger incompatible** `normal` — `build-xcframework` uses
    `GITHUB_REF_NAME` for version extraction. Re-triggering with `--ref main -f swift=true` would
    set `releaseTag = "main"` and corrupt the repo. Fix: derive version from Cargo.toml. Flagged
    `HUMAN REVIEW REQUESTED` — spec explicitly uses `GITHUB_REF_NAME`.
3. **Language logos in docs** `low` — CID skips

## Next Milestone

**Fix Swift GITHUB_REF_NAME bug + version sync + docs update** — three remaining tasks to close the
Swift XCFramework issues:

1. **Fix GITHUB_REF_NAME bug** (`normal` issue) — update `build-xcframework` job to derive version
    from `Cargo.toml` (like all other release jobs) and construct tag as `v$VERSION`. Requires spec
    update since spec explicitly uses `GITHUB_REF_NAME`. **HUMAN REVIEW REQUESTED** flag is set.
2. **Add `releaseTag` to `version_sync.py`** — target 16, so `mise run version:sync` propagates
    version to root `Package.swift`
3. **Update `docs/howto/swift.md`** — replace "planned for a future release" with zero-friction SPM
    install instructions using the binary target pattern

Remaining `low`-priority work (human-directed only):

- Language logos in docs howto headers
- Speedup factors published in documentation
