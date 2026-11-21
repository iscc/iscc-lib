<!-- assessed-at: 1b3a50fdd9dd0efb6de921e8af03f555e85db02d -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — release criteria met; remaining work is human-gated

ISCC-SEQ is bound and documented on all 11 surfaces, `data-edge.json` runs on the Rust core and all
11 bindings, and CI is green on `develop`. Iteration 247 marked every spec file tree as partial
(gated in `scripts/check_spec_paths.py`) and closed the last `normal` issue. Every open issue is
`low` (human-directed only), and the v1.0.0 items are human-gated, so the loop has no CID-workable
gap left.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: met

- `crates/iscc-lib/tests/data-edge.json` has 6 sections; harnesses on all 12 surfaces; the 5 tracked
    copies are byte-identical and match `VENDORED_COPIES`
- Python validator and C/C++ generator enforce the "Output subset" rule; the .NET, Java, napi and
    WASM harnesses compare every key outside a static uncompared set
- Not filed: the `parts` entry is untested on any surface (mixed has only an error vector); the WASM
    `compare` has no explicit non-object branch

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- No core source changed since iteration 235; no criterion in `target.md` or `rust-core.md` changed
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification (both `low` issues)

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- Python 3.10 floor is past end of life (2026-10-01) — `low` issue, human-only decision

## Node.js Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and strict data-edge tests green
    in CI; `@iscc/lib` 0.7.0 on npm (OIDC)
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols; conformance, unit and strict data-edge tests green in CI; SIMD release build
    met
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — a changed published export shape, worth a
    v0.8.0 release-notes line

## C FFI

**Status**: met (v0.8.0 scope)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; 0.7.0 tarballs on GitHub Releases
- `docs/howto/c-cpp.md` names every Tier 1 symbol and is registered `FULL` in the doc gate
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections strictly, green in CI; 0.7.0 on
    Maven Central
- `low` issue: the pending-exception guard has no test

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `data_edge_test.go` runs all sections, green in CI

## C# / .NET Bindings

**Status**: met (v0.8.0 scope)

- `EncodeSeq` / `DecodeSeq`; `DataEdgeTests.cs` runs all 6 sections strictly, green in CI; 0.7.0 on
    NuGet
- .NET 8 floor past end of life (2026-11-10) — `low` issue, human-only

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run from the generated
    header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan
- The C++ examples in `docs/howto/c-cpp.md` have no committed compile or output check

## Ruby Bindings

**Status**: met (v0.8.0 scope)

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections
    strictly; 0.7.0 on RubyGems; `ruby-source-gem` CI job green
- `low`: published source gems up to 0.7.0 do not build (needs a release; yank is Titusz's call);
    Ruby 3.1 floor EOL (human-only); integer list elements accept a Float

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift and Kotlin data-edge suites run all 6 sections; both green in CI, shipped 0.7.0

## Documentation

**Status**: met

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs
- Tier 1 doc gate `scripts/check_tier1_docs.py` (prek hook + pytest): 20 `FULL`, 2 `EXACT`, 2
    `EXEMPT`; `tests/test_rust_api_signatures.py` pins `rust-api.md` signatures
- Derived-fact drift issue closed (247): spec trees carry a "not every file" note gated by
    `scripts/check_spec_paths.py` (22 tests); the ci-cd version-sync table is now a pointer to
    `TARGETS` in `scripts/version_sync.py`
- `low` issues: Sum-Code example comments show `ISCC:KAA…` (call returns `ISCC:KUA…`); the
    pkg-config example in `c-cpp.md` hard-codes `Version: 0.2.0`; `ci-cd.md` still describes a
    `v*.*.*` tag trigger that `release.yml` (`workflow_dispatch` only) does not have

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 21 jobs (the `specs/ci-cd.md` job table matches, gated); `release.yml` is
    `workflow_dispatch`-only with 8 registry toggles; 21 version-sync targets at 0.7.0
- Security criterion met except the exempt EOL floors (Python 3.10, .NET 8, Ruby 3.1 — `low`, for
    Titusz); only Rust has a standing advisory gate (`cargo deny`)
- **Latest CI: PASSING** on `origin/develop` = `4d04bf9` (github-actions suite `success`):
    https://github.com/iscc/iscc-lib/actions/runs/44867934145
- HEAD is ahead only by a `.claude` log commit — green CI covers all code at HEAD

## Open Issues

12 open: 0 critical, 0 normal, 12 low (all human-directed; 7 carry `untriaged`, incl. the ci-cd tag
trigger). issues.md is at 216 of 300 lines.

## Next Milestone

v0.8.0 meets its release criteria and no CID-workable gap remains. Next progress needs Titusz: cut
v0.8.0 via `/release`, triage the `low` issues (promote or park), and decide the end-of-life floors
and the v1.0.0 cut.
