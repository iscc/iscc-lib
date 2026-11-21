<!-- assessed-at: fb76ee44eebadf6ac1386202c6d9822b56d06b50 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — features done; 12 triaged `normal` issues now block the release

ISCC-SEQ and `data-edge.json` run on the Rust core and all 11 bindings. Titusz's triage on
2026-11-21 (commit `07fe690`) promoted 12 of the former `low` issues to `normal` and moved 6 to the
new `postponed.md`. Each of the 12 is CID-workable with no human or upstream block, so the v0.8.0
criterion "no open `normal` issue" is unmet again. Three unpushed `cid(human)` commits change
`ci.yml` and a .NET test, so green CI does not cover HEAD yet.

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
    `conformance_selftest` runs the seq vectors; 308 lib tests; untouched since iteration 252
- `call_meta` in `tests/test_data_edge.rs` sits at CRAP 30.0, exactly the `--fail-above` limit
- `normal` (audit): specs and the core `CLAUDE.md` contradict the code in four places
- **Missing (human-gated, postponed):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check
- Decided 2026-11-21: the 3.10 floor stays; CI matrix is now 3.10/3.15 and `pyproject.toml` lists
    the 3.15 classifier (unpushed — not yet run in CI)

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

**Status**: partially met (one `normal` issue, shared with C++)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 8 sections strictly; 0.7.0 on NuGet
- Decided 2026-11-21: the `net8.0` floor stays; CI now runs a `dotnet-test` matrix (.NET 8 and 10
    runtimes, `DOTNET_ROLL_FORWARD`) with a `dotnet` aggregator, and `SmokeTests.cs` asserts the
    runtime major (unpushed — not yet run in CI)
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
- `normal` (human): raise the floor from 3.1 to 3.3 in gemspec, CI, Gemfile, docs, and extend the
    `release.yml` cross-gem list to every supported series
- `normal`: document the implicit Float→Integer `to_int` conversion (kept as host idiom)
- `low`: published source gems up to 0.7.0 do not build

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both suites run all 8 seq vectors and
    all 8 data-edge sections; the macOS `swift` job is green; shipped 0.7.0

## Documentation

**Status**: partially met (four `normal` issues)

- 22 docs pages, 11 how-to guides, Diátaxis nav, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; Tier 1 doc gate (20 `FULL`, 2 `EXACT`)
- `normal`: `architecture.md` lower sections cover half the surfaces; how-to Sum-Code comments show
    `KAA`; docs overstate language coverage and what ISO 24138 specifies [audit]
- `normal`: `specs/ci-cd.md` describes a release tag trigger the workflow does not have

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); the v1.0.0 semver gate is human-gated

- `ci.yml` now has 22 job keys (`dotnet-test` matrix + `dotnet` aggregator; job table in
    `specs/ci-cd.md` updated); `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0 (`target.md` now names `TARGETS` instead of a count)
- `iscc-ffi`, napi and WASM sit outside the iai and CRAP gates — binding marshalling is not gated
- **Latest CI: PASSING** on `origin/develop` = `1708d5e` (check-suite `success`):
    https://github.com/iscc/iscc-lib/actions/runs/44895981679
- **HEAD is 6 commits ahead of origin, 3 of them `cid(human)` with code:** `ci.yml` (Python 3.15
    leg, .NET matrix), `SmokeTests.cs`, `pyproject.toml`, `tools/cid.py`. Green CI does not cover
    them; the next push runs them for the first time

## Open Issues

13 open in issues.md: 0 critical, 12 `normal`, 1 `low`; none untriaged and none marked blocked on
the human or upstream. 6 issues moved to `postponed.md` (v1.0.0, MSRV, Unicode tracking, gate-script
remainders, JNI guard test, napi leak). The issues.md budget is now 400 lines (194 used).

## Next Milestone

Make sure the human CI changes (Python 3.15 leg, .NET 8/10 matrix) go green on the next push, then
clear the 12 `normal` issues that block v0.8.0. After that, Titusz cuts v0.8.0 via `/release`.
