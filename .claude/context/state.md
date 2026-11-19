<!-- assessed-at: be0c4d387df7778b3c5388280ce68e7ef060307b -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — data-edge.json binding fan-out (group 1 partly committed, unreviewed)

ISCC-SEQ is bound and documented on all 11 surfaces. `data-edge.json` runs on Rust and Python. The
fan-out has started: Go and Node harnesses are committed locally, but they are unreviewed and
unpushed. CI is green on `origin/develop` (`bd99366`), but that run does not include those two
commits. Still owed for v0.8.0: harnesses on the other 8 bindings, and 6 open `normal` issues.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met

- `crates/iscc-lib/tests/data-edge.json` holds 16 vectors in 5 sections. The `except` entries are
    `c: skip` on two vectors and `dotnet`/`cpp: error` on the embedded-NUL text vector. Rust (7
    tests) and Python (cross-checked against iscc-core) harnesses are reviewed and green
- `VENDORED_COPIES` registers 2 copies: `packages/go/testdata/` and
    `packages/dotnet/Iscc.Lib.Tests/testdata/`
- **Unreviewed, unpushed (`a84b665`, `be0c4d3`):** `packages/go/data_edge_test.go` (5 tests; the
    copy in `seq_test.go` is removed) and `crates/iscc-napi/__tests__/data_edge.test.mjs`
- **Missing:** harnesses for WASM, Java, .NET (the copy is vendored, but there is no
    `DataEdgeTests.cs` and no csproj entry), Ruby, C, C++, Swift and Kotlin

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

**Status**: partially met

- 35 Tier 1 symbols with TypeScript declarations; conformance and seq tests run the seq vectors;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- The data-edge harness (5 tests) is committed, but unreviewed and not covered by CI. The napi
    `Buffer` reference leak is a `low` issue

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance and unit tests cover seq; SIMD release build met
- **Missing:** the data-edge harness. **`normal` issue:** `specs/wasm-bindings.md` names a
    `crates/iscc-wasm/package.json` that does not exist

## C FFI

**Status**: partially met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- **Missing:** the C data-edge harness (generated table allowed). **`normal` issue:**
    `docs/c-ffi-api.md` type names differ from `iscc.h`. The `src/lib.rs` split is `low`

## Java Bindings

**Status**: partially met

- `encodeSeq` / `decodeSeq` in `IsccLib.java`; `IsccLibTest.java` runs both seq sections; 0.7.0 on
    Maven Central
- Missing: the data-edge harness. The JNI pending-exception mapping is a `low` issue

## Go Bindings

**Status**: partially met

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32, 268
    `func Test`
- The data-edge harness with its vendored copy is committed, but unreviewed and not covered by CI.
    It adds no Go `except` entry, so the `iscc_decode("")` vector is claimed to pass

## C# / .NET Bindings

**Status**: partially met

- `EncodeSeq` / `DecodeSeq` wrap the generated P/Invoke code; embedded NUL rejected via
    `ToNativeUtf8`; 0.7.0 on NuGet
- **Missing:** the data-edge harness (only the vendored copy exists). **`normal` issue:** the
    string-array pin loop is copied three times and the gen copies pass NULL for `[]`

## C++ Bindings

**Status**: partially met

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected via
    `detail::checked_c_str`; 0.7.0 vcpkg/Conan metadata
- Missing: the data-edge harness (generated table allowed)

## Ruby Bindings

**Status**: partially met

- `encode_seq` / `decode_seq` registered and run by `test_conformance.rb`; 0.7.0 on RubyGems
- Missing: the data-edge harness. The negative-Bignum `i32` truncation is a `low` issue

## Swift and Kotlin Bindings

**Status**: partially met

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq` (28 `#[test]`). Swift and Kotlin
    conformance suites have 19 tests each, including all 8 seq vectors; both shipped 0.7.0
- Missing: the data-edge harness on both

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
- **Latest CI: PASSING** on the pushed tip `bd99366` (= `origin/develop`). All 21 check-runs
    succeeded and the GitHub Actions check-suite is `success`
- **Not covered by CI:** 6 unpushed commits, including the Go and Node harness commits (code in
    `packages/go/`, `crates/iscc-napi/__tests__/`, and `tests/test_vendored_fixtures.py`)
- **Missing (`normal` issue):** Dependabot alerts on `soupsieve`, `setuptools` and the `json` gem

## Open Issues

18 open: 0 critical, **6 normal**, 12 low. Normal: c-ffi-api type names, Python `encode_seq`
docstrings, Dependabot alerts, .NET pin-loop duplication, the wasm-bindings spec drift, and
derived-fact drift.

## Next Milestone

Get the committed Go and Node harnesses reviewed and pushed. Then bring `data-edge.json` to the
remaining 8 bindings, and clear the 6 open `normal` issues. Together these are what remains for
v0.8.0.
