<!-- assessed-at: cdc89254e7661725193d24958aab077aafee9280 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — data-edge fan-out complete; CI tip red (Kotlin dependency fetch), 6 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` now runs on the Rust core
and all 11 bindings (C and C++ landed this iteration via a generated header). All code is covered by
a fully green CI run (`4abafb6`), but the pushed tip `0da6e41` is red: the Kotlin job could not
download the Kotlin Gradle plugin. Still owed for v0.8.0: a green tip and the 6 open `normal`
issues.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 16 vectors in 5 sections, with 2 `except` entries (C
    skips 2 vectors; .NET and C++ expect an error on the embedded-NUL text vector)
- Harnesses on all 12 surfaces. C and C++ include `crates/iscc-ffi/tests/data_edge_vectors.h`, which
    `scripts/gen_data_edge_vectors.py` renders. `tests/test_gen_data_edge_vectors.py` fails when the
    header is stale. C skips only the rows the table marks `c: skip`
- `VENDORED_COPIES` registers 4 data-edge copies (Go, .NET, Kotlin, Swift); 5 files are tracked
- `low` issues: the Swift compare is looser than byte equality; Go/Swift ship the file against the
    spec's "absent from every published artifact" (Titusz decides)

## Rust Core Crate

**Status**: partially met

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES` in `codec.rs`;
    `conformance_selftest` runs the seq vectors; the data-edge harness is green
- iai Perf, Coverage + CRAP and semver-checks pass on the pushed tip (semver is informational)
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: partially met

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- **Missing (`normal` issue):** the `encode_seq` docstrings don't name the non-ASCII case-fold
    divergence from iscc-core

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: partially met

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met
- **`normal` issue:** `specs/wasm-bindings.md` names a `crates/iscc-wasm/package.json` that does not
    exist

## C FFI

**Status**: partially met

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`, plus a total-row
    tally; the `c-ffi` job is green
- **`normal` issue:** `docs/c-ffi-api.md` type names differ from `iscc.h`. The `src/lib.rs` split is
    `low`

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `IsccLibTest` runs both seq sections; `DataEdgeTest` green in CI; 0.7.0
    on Maven Central
- `low` issues: JNI pending-exception mapping, missing null guard on hasher handles

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32
- `data_edge_test.go` with the vendored copy is green in CI

## C# / .NET Bindings

**Status**: partially met

- `EncodeSeq` / `DecodeSeq`; embedded NUL rejected via `ToNativeUtf8`; `DataEdgeTests.cs` green in
    CI; 0.7.0 on NuGet
- **`normal` issue:** the string-array pin loop is copied three times, and the gen copies pass NULL
    for `[]`

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; embedded NUL rejected via
    `detail::checked_c_str`; 0.7.0 vcpkg/Conan metadata
- `tests/test_iscc.cpp` runs all 16 data-edge rows from the generated header (private test include)
    with a visited-row tally; the ASAN `cpp` job is green
- `low` issue: five `iscc.hpp` list wrappers pass NULL for an empty vector

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq` run by `test_conformance.rb`; `test_data_edge.rb` compares every
    expected key and is green in CI; 0.7.0 on RubyGems
- `low` issue: negative-Bignum `i32` truncation

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope) — Kotlin job red on the tip for an external reason, see CI

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; both shipped 0.7.0
- Swift `DataEdgeTests` and Kotlin `DataEdgeTest` (6 tests each) were green on `4abafb6`, each with
    a registered vendored copy
- `low` issue: the Swift compare uses canonical string equivalence and reads a JSON bool as an Int

## Documentation

**Status**: met (v0.8.0 scope)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; all how-tos cover the seq functions
- **`normal` issue `[human]`:** hand-written derived facts (Tier 1 lists, counts) drift

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- **Latest CI: FAILING** on `origin/develop` = `0da6e41` (run 44647425663): 20 of 21 check-runs
    passed. **Kotlin (gradle build, test)** failed while configuring the build: the Gradle Plugin
    Portal could not provide `kotlin-gradle-plugins-bom:2.4.10`. It looks external: nothing under
    `packages/kotlin`, `crates/iscc-uniffi` or `.github` changed since the green `39cff64`
- The run on `4abafb6` (44646030279) was fully green, and `4abafb6..0da6e41` changes only
    `.claude/`, so green CI covers all code. The only unpushed commit (`cdc8925`) touches `.claude/`
    only
- **Missing (`normal` issue):** Dependabot alerts on `soupsieve` and `setuptools` (`uv.lock`)

## Open Issues

21 open: 0 critical, **6 normal**, 15 low. Normal: c-ffi-api type names, Python `encode_seq`
docstrings, Dependabot alerts, .NET pin-loop duplication, wasm-bindings spec drift, derived-fact
drift. issues.md sits at its 300-line budget.

## Next Milestone

A green CI run on the `develop` tip (the Kotlin job is the only red one), then the 6 open `normal`
issues, which are the last v0.8.0 release blockers.
