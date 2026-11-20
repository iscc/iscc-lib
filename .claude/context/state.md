<!-- assessed-at: d04472fdeec2190db7288be7d3e03b1e87167f2a -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 1 normal issue owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 232 made the Swift and Python data-edge compares strict and closed that
`normal` issue. CI is fully green on the pushed tip `e289ad8`, including the macOS Swift job. One
`normal` `[human]` issue (derived-fact drift) is the last v0.8.0 blocker.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met (v0.8.0 scope)

- `crates/iscc-lib/tests/data-edge.json` has 6 sections: `iscc_decode`, `encode_seq`,
    `gen_text_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `alg_simhash`; harnesses on all 12
    surfaces; the 5 tracked copies are byte-identical and match `VENDORED_COPIES`
- Swift reads every Bool/integer through one `DataEdge.scalar` classifier and compares strings by
    Unicode scalars; Python `same` never equates a `bool` with a non-`bool`
- Unfiled open question: .NET, Java, napi and WASM compare only the expected keys their result
    exposes

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- `alg_simhash` returns `InvalidInput` for `[]`; `rust-core.md` Tier 1 table matches the code
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met

## C FFI

**Status**: met (v0.8.0 scope)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `src/lib.rs` is 1717 lines; the 99 unit tests live in `src/tests.rs`
- `tests/test_iscc.c` runs every data-edge row (incl. `alg_simhash`) except the 2 marked `c: skip`

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections, green in CI; 0.7.0 on Maven Central
- Hasher exports reject a zero handle; JVM allocation failure maps to `OutOfMemoryError`; 64 `@Test`
    methods
- `low` issue: the pending-exception guard has no test

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `AlgSimhash([])` errors; `data_edge_test.go` runs the `alg_simhash` section, green in CI

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; embedded NUL rejected; `PinnedRows` pins all jagged-array inputs;
    `DataEdgeTests.cs` runs all 6 sections, green in CI; 0.7.0 on NuGet
- `docs/howto/dotnet.md` "Error handling" doesn't list `ArgumentNullException` (no issue filed)

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected; all data-edge rows run
    from the generated header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan
- Every vector input goes through `detail::safe_data`, held by `tests/test_cpp_safe_data.py`

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections,
    green in CI; 0.7.0 on RubyGems; `i32` list elements raise `RangeError` out of range
- `low` issue: integer list elements accept a Float and truncate it (Titusz decides)

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift has 30 tests (19 conformance, 11 data-edge incl. 5 strict-compare cases); Kotlin
    `DataEdgeTest` runs all 6 sections; both green in CI and shipped 0.7.0

## Documentation

**Status**: met (v0.8.0 scope); 1 `normal` issue

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; all how-tos cover the seq functions
- `scripts/check_spec_paths.py` gates spec paths
- **`normal` `[human]`:** no Tier 1 list parity check; hand-written counts and file lists remain in
    specs and per-crate `CLAUDE.md` files (e.g. `specs/java-bindings.md` still says 65 tests)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: PASSING** on `origin/develop` = `e289ad8`
    (https://github.com/iscc/iscc-lib/actions/runs/44766855703): all 21 check-runs `success`,
    including `Swift (swift build, swift test)` on macOS
- Unpushed: 1 commit (the iteration-232 log), under `.claude/` only; green CI covers all code at
    HEAD
- GitHub Dependabot alerts track `main` and stay open until the next release (triaged, no issue)

## Open Issues

8 open: 0 critical, **1 normal**, 7 low. Normal: derived-fact drift `[human]`. issues.md is at 152
of 300 lines.

## Next Milestone

Clear the last open `normal` issue (derived facts written by hand drift from the code), the only
remaining v0.8.0 release blocker.
