<!-- assessed-at: 09d68d32617a9efabbd5efc37183b900481086b6 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — data-edge.json binding fan-out (groups 1-2 done, C/C++ owed)

ISCC-SEQ is bound and documented on all 11 surfaces. `data-edge.json` now runs on Rust, Python and
eight bindings (Go, Node, WASM, Java, .NET, Ruby, Swift, Kotlin). All of it is reviewed, pushed and
green in CI. Still owed for v0.8.0: the C and C++ harnesses, and 6 open `normal` issues.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met

- `crates/iscc-lib/tests/data-edge.json` has 16 vectors in 5 sections. 2 `except` entries: `c: skip`
    on 2 vectors, plus `dotnet`/`cpp: error` on the embedded-NUL text vector
- Harnesses exist for Rust, Python (checked against iscc-core), Go, Node, WASM, Java, .NET, Ruby,
    Swift (`DataEdgeTests.swift`, 6 tests) and Kotlin (`DataEdgeTest.kt`, 6 tests). The fixture
    validator rejects `outputs: {}`, and Ruby now fails on a missing expected key
- `VENDORED_COPIES` registers 4 data-edge copies (Go, .NET, Kotlin, Swift). 5 `data-edge.json` files
    are tracked
- **Missing:** the C and C++ harnesses (nothing under `crates/iscc-ffi` or `packages/cpp` references
    `data-edge`; a generated table is allowed). `low` issues: the Swift compare is looser than byte
    equality; Go/Swift ship the file against the spec's "absent from every published artifact"
    (Titusz decides)

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES` live in `codec.rs`.
    `conformance_selftest` runs the seq vectors. The data-edge harness is green
- iai Perf, Coverage + CRAP and semver-checks pass on the pushed tip (semver is informational)
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI. The data-edge harness with the iscc-core cross-check is green
- **Missing (`normal` issue):** the `encode_seq` docstrings don't name the non-ASCII case-fold
    divergence from iscc-core

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; the conformance, seq and data-edge tests are green
    in CI; `@iscc/lib` 0.7.0 on npm (OIDC)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance, unit and data-edge tests are green in CI; SIMD release build met
- **`normal` issue:** `specs/wasm-bindings.md` names a `crates/iscc-wasm/package.json` that does not
    exist

## C FFI

**Status**: partially met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** the C data-edge harness. **`normal` issue:** `docs/c-ffi-api.md` type names differ
    from `iscc.h`. The `src/lib.rs` split is `low`

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `IsccLibTest` runs both seq sections; `DataEdgeTest` is green in CI;
    0.7.0 on Maven Central
- `low` issues: JNI pending-exception mapping, missing null guard on hasher handles

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32
- `data_edge_test.go` with the vendored copy is green in CI

## C# / .NET Bindings

**Status**: partially met

- `EncodeSeq` / `DecodeSeq`; embedded NUL rejected via `ToNativeUtf8`; `DataEdgeTests.cs` is green
    in CI; 0.7.0 on NuGet
- **`normal` issue:** the string-array pin loop is copied three times, and the gen copies pass NULL
    for `[]`

## C++ Bindings

**Status**: partially met

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected via
    `detail::checked_c_str`; 0.7.0 vcpkg/Conan metadata
- **Missing:** the data-edge harness (a generated table is allowed)

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq` run by `test_conformance.rb`; `test_data_edge.rb` compares every
    expected key and is green in CI; 0.7.0 on RubyGems
- `low` issue: negative-Bignum `i32` truncation

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; both shipped 0.7.0
- Swift `DataEdgeTests` (6 tests, run by name in the macOS-15 arm64 CI job) and Kotlin
    `DataEdgeTest` (6 tests) are green in CI, each with a registered vendored copy
- `low` issue: the Swift compare uses canonical string equivalence and reads a JSON bool as an Int

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs. All how-tos cover `encode_seq` /
    `decode_seq`
- **`normal` issue `[human]`:** hand-written derived facts (Tier 1 lists, counts) drift

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: PASSING** on `origin/develop` = `39cff64`. All 21 check-runs succeeded; the GitHub
    Actions check-suite is `success`. The only unpushed commit (`09d68d3`) touches `.claude/` only,
    so green CI covers all code at HEAD
- **Missing (`normal` issue):** Dependabot alerts on `soupsieve`, `setuptools` and the `json` gem

## Open Issues

20 open: 0 critical, **6 normal**, 14 low. Normal: c-ffi-api type names, Python `encode_seq`
docstrings, Dependabot alerts, .NET pin-loop duplication, wasm-bindings spec drift, derived-fact
drift. issues.md sits at its 300-line budget.

## Next Milestone

Bring `data-edge.json` to the last 2 surfaces (C and C++) and clear the 6 open `normal` issues.
Together these are what remains for v0.8.0.
