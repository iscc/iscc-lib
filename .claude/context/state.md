<!-- assessed-at: c87ec0efef6d38f449bfa2a4b7d5cfa7e44bad3f -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; CI red on the .NET 10 leg; 11 `normal` issues block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Iteration 259 realigned the
specs and crate `CLAUDE.md` files with the code (docs only) and pushed the three human CI commits
for the first time. The first run on `eb132d2` fails `Run .NET tests` on the new .NET 10 matrix leg.
11 `normal` issues stay open, none blocked on the human or upstream.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 8 sections; the ragged video vector carries a
    `diverges` marker; the 5 tracked copies are byte-identical; C/C++ read the generated
    `data_edge_vectors.h`
- napi, WASM, JNI and .NET leave `name`, `description`, `meta`, `metahash` uncompared for meta
- Not filed: the `parts` entry is untested on any surface; the WASM `compare` has no explicit
    non-object branch

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; 308 lib tests; code untouched since iteration 252
- Specs and `crates/iscc-lib/CLAUDE.md` now match the code (audit issue closed in 259)
- `call_meta` in `tests/test_data_edge.rs` sits at CRAP 30.0, exactly the `--fail-above` limit
- **Missing (human-gated, postponed):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check
- CI matrix 3.10/3.15 now runs in CI; the 3.15 leg was still running at assessment time

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests;
    `@iscc/lib` 0.7.0 on npm (OIDC); numeric parameters are range-checked
- The napi `Buffer` reference leak is postponed (`postponed.md`)

## WASM Bindings

**Status**: partially met (one `normal` issue)

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests; SIMD release build met
- `normal`: the wasm-bindgen glue converts `null`, strings and booleans to numbers instead of
    rejecting them
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — needs a v0.8.0 release-notes line

## C FFI

**Status**: met

- 51 exports incl. `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with freshness check;
    `examples/iscc_sum.c` + CMake; 0.7.0 tarballs; `docs/howto/c-cpp.md` gated `FULL`
- `(NULL, 0)` is an empty input on every pointer+length export via `slice_or_empty`
- `normal`: the pkg-config example in `docs/howto/c-cpp.md` says `Version: 0.2.0`

## Java Bindings

**Status**: partially met (one `normal` issue)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 8 sections strictly; 0.7.0 on Maven Central
    with five bundled native libraries
- `normal`: `NativeLoader` throws on an unknown OS/arch before trying `java.library.path`
- The pending-exception guard test is postponed

## Go Bindings

**Status**: partially met (one `normal` issue)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape; data-edge green incl.
    the raw-JCS meta vector and the ragged video vector
- `normal`: Go canonical `meta` differs from the core on UTF-16 key order, `-0` and lone surrogates
- Published 0.7.0 still escapes `&`, `<`, `>`, U+2028/9 in canonical `meta` (fixed on develop)

## C# / .NET Bindings

**Status**: partially met (CI red + one `normal` issue shared with C++)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 8 sections strictly; 0.7.0 on NuGet
- **CI red:** the `dotnet-test` leg for 10.0 (net8.0 build, `DOTNET_ROLL_FORWARD=LatestMajor`) fails
    at `Run .NET tests`; the 8.0 leg was `cancelled` by matrix fail-fast, so its result is unknown.
    Cause not yet in the logs (run still in progress)
- `SmokeTests.cs` `Runtime_MatchesExpectedMajorVersion` asserts `ISCC_EXPECT_DOTNET_MAJOR`
- `normal`: the `PinnedRows` padding (and C++ `safe_data`) guard a NULL rejection the FFI no longer
    has — removal decided

## C++ Bindings

**Status**: partially met (one `normal` issue, shared with .NET)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run; ASAN `cpp` job; 0.7.0
    vcpkg/Conan
- `normal`: remove the redundant `safe_data` padding
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: partially met (two `normal` issues)

- `encode_seq` / `decode_seq`; `test_data_edge.rb` runs all 8 sections strictly; 0.7.0 on RubyGems
- `normal` (human, decided): raise the floor from 3.1 to 3.3 in gemspec, CI, Gemfile, docs, and
    extend the `release.yml` cross-gem list to every supported series
- `normal`: document the implicit Float→Integer `to_int` conversion (kept as host idiom)
- `low`: published source gems up to 0.7.0 do not build

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 8 data-edge sections; the macOS `swift` job is green; shipped 0.7.0
- `specs/swift-bindings.md` Release Flow now matches `release.yml` (an existing tag is not moved)

## Documentation

**Status**: partially met (five `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; how-to Sum-Code comments show
    `KAA`; docs overstate language coverage and what ISO 24138 specifies [audit]
- `normal` (filed 259): `docs/development.md` "Publish" says a tag push triggers the release; the
    Swift spec heading "Force-Update Tag Pattern" no longer matches its flow
- Spec release-trigger claims fixed in 259 (`specs/ci-cd.md` issue closed)

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: partially met — CI red; the v1.0.0 semver gate is human-gated

- `ci.yml` has 22 job keys (`dotnet-test` matrix + `if: always()` `dotnet` aggregator);
    `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21 version-sync targets at
    0.7.0
- `dotnet-test` matrix has no `fail-fast: false`, so one failing leg cancels the other
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates — binding marshalling is not gated
- **Latest CI: FAILING** on `origin/develop` = `eb132d2` (run in progress, already red):
    https://github.com/iscc/iscc-lib/actions/runs/44903592630 — failed: `C# / .NET 10.0` and the
    `C# / .NET` aggregator; `C# / .NET 8.0` cancelled. Last fully green: `1708d5e` (run 44895981679)
- HEAD is 1 commit ahead of origin (`cid(log)`, `.claude` only); green CI covers no code since
    `1708d5e`

## Open Issues

12 open in issues.md: 0 critical, 11 `normal`, 1 `low`; none untriaged and none blocked on the human
or upstream. 259 closed 2 (`ci-cd.md` tag trigger, spec/CLAUDE.md contradictions) and filed 1
(`docs/development.md` Publish). 6 more sit in `postponed.md`.

## Next Milestone

Fix CI first: get the .NET 10 `dotnet-test` leg (and the cancelled 8.0 leg) green on `develop`. Then
clear the 11 `normal` issues that block v0.8.0; after that Titusz cuts v0.8.0 via `/release`.
