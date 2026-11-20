<!-- assessed-at: c2811c5 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 2 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 238 fixed the Java how-to `genSumCodeV0` example and stated the Ruby
precompiled-gem range, closing two `normal` issues. Review then found the published Ruby source gem
cannot be built, so the docs' "builds from source" fallback is false — a fresh `normal` issue. With
the derived-fact drift issue, 2 `normal` issues still block v0.8.0.

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
- No core source changed since iteration 235
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

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
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`

## Java Bindings

**Status**: met (v0.8.0 scope)

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections strictly, green in CI; 0.7.0 on
    Maven Central
- `docs/howto/java.md` `genSumCodeV0` example now passes 4 arguments and uses field access (issue
    closed in 238)
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

## Ruby Bindings

**Status**: partially met — 1 `normal` issue

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections
    strictly, green in CI; 0.7.0 on RubyGems
- `docs/howto/ruby.md` and `crates/iscc-rb/README.md` state precompiled gems for "Ruby 3.1 to 3.3";
    `tests/test_ruby_gem_docs.py` (9 tests) derives that range from the `release.yml` cross-gem list
- **`normal`:** the source gem cannot be built — the gemspec declares no `rb_sys` dependency and the
    shipped `Cargo.toml` inherits from the workspace and path-depends on `../iscc-lib`. On Ruby
    3.4/4.0 (or an unbuilt platform) `gem install iscc-lib` fails, contradicting both Ruby docs'
    "compiles the extension from source"; no workflow installs the source gem
- Ruby 3.1 floor is EOL (`low`, human-only); `low` issue: integer list elements accept a Float

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift and Kotlin data-edge suites run all 6 sections; both green in CI, shipped 0.7.0

## Documentation

**Status**: partially met — 1 `normal` `[human]` issue (+ the Ruby source-gem claim above)

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs
- Tier 1 doc gate `scripts/check_tier1_docs.py` (prek hook + pytest) checks registered docs name
    every symbol derived from `lib.rs`
- **Remaining in the `[human]` drift issue:** the gate fails open on `lib.rs` forms it does not
    parse; `docs/howto/rust.md`, `docs/howto/c-cpp.md`, `docs/rust-api.md` and
    `docs/architecture.md` are unregistered and incomplete; hand-written counts remain in specs and
    per-crate `CLAUDE.md` files
- `low` issue: Sum-Code example comments show `ISCC:KAA…` where the call returns `ISCC:KUA…`

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- `publish-rubygems` builds the source gem with a bare `gem build` and never installs it (see Ruby)
- Security criterion met except the exempt EOL floors (Python 3.10, .NET 8, Ruby 3.1 — `low`, for
    Titusz); only Rust has a standing advisory gate (`cargo deny`)
- **Latest CI: PASSING** on `origin/develop` = `732e2e0` (`github-actions` suite `success`, 21/21
    check-runs `success`): https://github.com/iscc/iscc-lib/actions/runs/44825837243
- Unpushed: only the `cid(log)` commit (`.claude/` only) — green CI covers HEAD's code

## Open Issues

11 open: 0 critical, **2 normal**, 9 low. Normal: derived-fact drift `[human]`, the Ruby source gem
cannot be built. issues.md is at 227 of 300 lines.

## Next Milestone

Clear the 2 open `normal` issues — the remaining v0.8.0 release blockers: make the Ruby install
story true (a buildable, CI-checked source gem, or no source gem and docs that say so), and the
remaining derived-fact drift.
