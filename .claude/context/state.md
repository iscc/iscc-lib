<!-- assessed-at: 0bf8f8c77005aeba753131c3e81d90ee408a5e1b -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.8.0 — all features landed, CI green; 4 normal issues owed

ISCC-SEQ is bound and documented on all 11 surfaces, and `data-edge.json` runs on the Rust core and
all 11 bindings. Iteration 235 did the pre-release security pass (rustix 1.1.6, Go x/text v0.41.0,
Go CI on toolchain go1.26.11, napi CI on Node 22) and closed that issue; CI is green on the
refreshed set. Review filed a `normal` Ruby precompiled-gem gap, so 4 `normal` issues still block
v0.8.0.

## Shared Edge-Case Vectors (`data-edge.json`)

**Status**: partially met — 1 `normal` `[human]` issue

- `crates/iscc-lib/tests/data-edge.json` has 6 sections: `iscc_decode`, `encode_seq`,
    `gen_text_code_v0`, `gen_mixed_code_v0`, `gen_iscc_code_v0`, `alg_simhash`; harnesses on all 12
    surfaces; the 5 tracked copies are byte-identical and match `VENDORED_COPIES`
- **`normal`:** the .NET, Java, napi and WASM harnesses compare only the keys their result exposes,
    so expected keys like `characters` are dropped in harness code, against the spec rule that every
    deviation is declared in the file. Waits on an output-subset rule in `edge-vectors.md`

## Rust Core Crate

**Status**: partially met (v0.8.0 scope met; v1.0.0 items human-gated)

- Tier 1 has 35 symbols at the crate root; Tier 2 `read_unit` + `SEQ_MAINTYPES`;
    `conformance_selftest` runs the seq vectors; iai, Coverage + CRAP and semver jobs pass in CI
- `Cargo.lock` now locks `rustix` 1.1.6 (advisory fix); no core source changed this iteration
- **Missing (human-gated, not v0.8.0):** crate `>= 1.0.0`; MSRV verification

## Python Bindings

**Status**: met (v0.8.0 scope)

- 35 Tier 1 symbols with `.pyi` stubs and `__all__`; abi3-py310 wheels for 4 platforms; 12
    `py.detach` sites; 0.7.0 on PyPI; data-edge harness with the iscc-core cross-check is green
- Python 3.10 floor is past end of life (2026-10-01) — `low` issue, human-only decision

## Node.js Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- 35 Tier 1 symbols with TypeScript declarations; conformance, seq and data-edge tests green in CI;
    `@iscc/lib` 0.7.0 on npm (OIDC); `ci.yml` napi job now on Node `22`
- `low` issue: the napi `Buffer` reference leak

## WASM Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- 35 Tier 1 symbols; conformance, unit and data-edge tests green in CI; SIMD release build met
- `alg_cdc_chunks` returns `Uint8Array[]` since 234 — a changed published export shape, worth a
    v0.8.0 release-notes line

## C FFI

**Status**: met (v0.8.0 scope)

- 51 exports including `iscc_encode_seq` / `iscc_decode_seq`; committed `iscc.h` with a freshness
    check; `examples/iscc_sum.c` + CMake; `docs/howto/c-cpp.md`; 0.7.0 tarballs on GitHub Releases
- `tests/test_iscc.c` runs every data-edge row except the 2 marked `c: skip`

## Java Bindings

**Status**: met (v0.8.0 scope); 1 `normal` doc issue

- `encodeSeq` / `decodeSeq`; `DataEdgeTest` runs all 6 sections (partial-key compare, see above),
    green in CI; 0.7.0 on Maven Central
- **`normal`:** `docs/howto/java.md` `genSumCodeV0` example passes 3 args (method takes 4) and calls
    accessors on `public final` fields; it does not compile
- `low` issue: the pending-exception guard has no test

## Go Bindings

**Status**: met (v0.8.0 scope)

- Pure Go, 35 Tier 1 symbols, `CGO_ENABLED=0`, IDv1 at the reference shape, strict base32;
    `data_edge_test.go` runs all sections, green in CI
- `go.mod`: `go 1.26.1` floor, `toolchain go1.26.11`, `golang.org/x/text v0.41.0`; the CI `go` job
    log confirms go1.26.11

## C# / .NET Bindings

**Status**: met (v0.8.0 scope); data-edge harness covered by the partial-compare issue

- `EncodeSeq` / `DecodeSeq`; `PinnedRows` pins all jagged-array inputs; `DataEdgeTests.cs` runs all
    6 sections, green in CI; 0.7.0 on NuGet
- .NET 8 floor past end of life (2026-11-10) — `low` issue, human-only

## C++ Bindings

**Status**: met (v0.8.0 scope)

- `iscc::encode_seq` / `iscc::decode_seq`, RAII-owned; all data-edge rows run from the generated
    header; ASAN `cpp` job green; 0.7.0 vcpkg/Conan

## Ruby Bindings

**Status**: partially met — 1 `normal` issue

- `encode_seq` / `decode_seq` in `test_conformance.rb`; `test_data_edge.rb` runs all 6 sections
    strictly, green in CI; 0.7.0 on RubyGems
- **`normal`:** the `release.yml` cross-gem list is Ruby 3.1/3.2/3.3 (tied to the rb_sys 0.9.123
    pin), so published platform gems declare `< 3.4.dev`; Ruby 3.4/4.0 users fall back to a source
    build needing Rust. Docs promise precompiled gems without naming this limit
- Ruby 3.1 floor is EOL (`low`, human-only); `low` issue: integer list elements accept a Float

## Swift and Kotlin Bindings

**Status**: met (v0.8.0 scope)

- `crates/iscc-uniffi` exports `encode_seq` / `decode_seq`; both conformance suites run all 8 seq
    vectors; Swift 30 tests (19 conformance, 11 data-edge); Kotlin `DataEdgeTest` runs all 6
    sections; both green in CI and shipped 0.7.0

## Documentation

**Status**: partially met — 1 `normal` `[human]` issue

- 22 docs pages, 11 how-to guides, Diátaxis navigation, ISCC branding, `llms-full.txt`, Development
    section; root README plus 12 per-crate/package READMEs; `docs/development.md` tool table now
    matches the Dockerfile (Python 3.13, Node 22, Go 1.26.11)
- Tier 1 doc gate `scripts/check_tier1_docs.py` (prek hook + pytest) checks registered docs name
    every symbol derived from `lib.rs`
- **Remaining in the `[human]` drift issue:** the gate fails open on `lib.rs` forms it does not
    parse; `docs/howto/rust.md`, `docs/howto/c-cpp.md`, `docs/rust-api.md` and
    `docs/architecture.md` are unregistered and incomplete; hand-written counts remain in specs and
    per-crate `CLAUDE.md` files

## Benchmarks

**Status**: met

- 12 criterion benches + iai-callgrind in `crates/iscc-lib/benches/`; pytest-benchmark in
    `benchmarks/python/`; speedups in `docs/benchmarks.md`

## CI/CD and Publishing

**Status**: met (v0.8.0 scope); EOL floors and the v1.0.0 semver gate are human-gated

- `ci.yml` has 20 jobs; `release.yml` is `workflow_dispatch`-only with 8 registry toggles; 21
    version-sync targets at 0.7.0
- Security criterion: advisory-flagged pins bumped and no CI test runtime is EOL except the exempt
    floors (Python 3.10, .NET 8, Ruby 3.1 — `low` issue for Titusz). `ci-cd.md` freshness table has
    a Runtimes row and the floors-are-human-only rule. Only Rust has a standing advisory gate
    (`cargo deny`); adding osv-scanner/govulncheck is an unfiled policy call
- Dependabot shows 6 open alerts (`soupsieve` x4, `setuptools`, `json`); it tracks `main` — on
    `develop`, `uv.lock` has no `soupsieve`, `setuptools` 84.2.1 and Ruby `json` 2.21.2, all patched
- **Latest CI: PASSING** on `origin/develop` = `edd20de` (`github-actions` suite `success`, 21/21
    check-runs `success`): https://github.com/iscc/iscc-lib/actions/runs/44807966740
- Unpushed: only the `cid(log)` commit (`.claude/` only) — green CI covers HEAD's code

## Open Issues

12 open: 0 critical, **4 normal**, 8 low. Normal: derived-fact drift `[human]`, Java how-to
`genSumCodeV0` example, data-edge partial-key compare `[human]`, precompiled Ruby gems stop at 3.3.
issues.md is at 236 of 300 lines.

## Next Milestone

Clear the 4 open `normal` issues — the remaining v0.8.0 release blockers: strict data-edge compares
on .NET/Java/napi/WASM (and the spec rule they wait on), the broken Java how-to example, precompiled
Ruby gem coverage, and the remaining derived-fact drift.
